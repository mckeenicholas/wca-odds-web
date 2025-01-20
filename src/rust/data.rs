use std::{collections::HashMap, error::Error};

use chrono::{DateTime, Datelike, Duration, NaiveDateTime, TimeZone, Utc};
use futures::future::join_all;
use reqwest::Client;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, Debug)]
pub struct CompetitionResult {
    pub id: String,
    pub results: Vec<i32>,
}

pub async fn get_competition_data(
    month_cutoff: i32,
) -> Result<HashMap<String, i64>, Box<dyn Error>> {
    let now: DateTime<Utc> = Utc::now();
    let current_year = now.year();

    let months_duration = Duration::days((month_cutoff * 30) as i64);
    let start_date = now - months_duration;

    let cutoff_timestamp = start_date.timestamp();

    let first_year = start_date.year();

    let years: Vec<i32> = (first_year..=current_year).collect();

    let mut all_competitions = HashMap::<String, i64>::new();

    let o_client = reqwest::Client::new();

    let futures = years.iter().map(|year| {
        let client = o_client.clone();

        async move {
            
            let url = format!(
                "https://raw.githubusercontent.com/robiningelbrecht/wca-rest-api/master/api/competitions/{}.json",
                year
            );

            // Fetch the response and parse it as JSON
            let response: Value = fetch(url, client).await?; // Replace `fetch` with your HTTP request function

            // Extract the competitions array
            let competitions_json = response
                .get("items")
                .ok_or("Expected field 'items' on JSON response")?
                .as_array()
                .ok_or("Expected 'items' to be a list")?;

            // Convert the JSON to a Vec of Competition structs
            let competitions: Result<HashMap<String, i64>, Box<dyn Error>> = competitions_json
    .iter()
    .filter_map(|comp| {
        // Extract the date
        let date_obj = match comp.get("date") {
            Some(date) => date,
            None => return Some(Err(Box::<dyn Error>::from("field 'date' does not exist on JSON object"))),
        };

        let date_str = match date_obj.get("from") {
            Some(from) => match from.as_str() {
                Some(s) => s,
                None => return Some(Err(Box::<dyn Error>::from("Expected 'from' field to be a string"))),
            },
            None => return Some(Err(Box::<dyn Error>::from("Expected field 'from' on 'date'"))),
        };

        let naive_date = match NaiveDateTime::parse_from_str(
            &format!("{} 00:00:00", date_str),
            "%Y-%m-%d %H:%M:%S",
        ) {
            Ok(date) => date,
            Err(e) => return Some(Err(Box::new(e))), // Convert error into Box<dyn Error>
        };

        let datetime = Utc.from_utc_datetime(&naive_date);

        let date = datetime.timestamp();

        if date < cutoff_timestamp {
            return None;
        }

        // Extract the ID
        let id = match comp.get("id") {
            Some(id) => match id.as_str() {
                Some(s) => s.to_string(),
                None => return Some(Err(Box::<dyn Error>::from("Expected 'id' field to be a string"))),
            },
            None => return Some(Err(Box::<dyn Error>::from("Expected field 'id' on 'competition'"))),
        };

        Some(Ok((id, date)))
    })
    .collect();

            competitions
        }
    });

    // Await all the futures and handle errors
    let results = join_all(futures).await;

    for result in results {
        match result {
            Ok(competitions) => all_competitions.extend(competitions),
            Err(e) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Error fetching competition data: {}", e),
                )))
            }
        }
    }

    Ok(all_competitions)
}

pub async fn get_solve_data(
    competitors: Vec<String>,
    event: String,
) -> Result<Vec<Vec<CompetitionResult>>, Box<dyn std::error::Error>> {
    let mut competitor_results: Vec<Vec<CompetitionResult>> = Vec::new();
    let o_client = reqwest::Client::new();

    let futures = competitors.iter().map(|competitor| {
        let event_ref = &event;
        let client = o_client.clone();
        async move {
            let url = format!("https://raw.githubusercontent.com/robiningelbrecht/wca-rest-api/master/api/persons/{}.json", competitor);
            let response = fetch(url, client).await?;

            let results_json = response.get("results").ok_or("Missing 'results' field in JSON response")?.as_object().ok_or("Expected 'results' to be an object")?;

            let results: Vec<CompetitionResult> = results_json
                .iter()
                .filter_map(|(key, value)| {
                    value.get(event_ref).and_then(|event_data| {
                        // Attempt to extract and flatten all solves into a single vector
                        let solves: Vec<i32> = event_data
                            .as_array()?
                            .iter()
                            .flat_map(|round| {
                                round
                                    .get("solves")
                                    .and_then(|s| s.as_array())
                                    .unwrap_or(&vec![]) // Use empty vector if "solves" is missing
                                    .iter()
                                    .filter_map(|solve| solve.as_i64().map(|x| x as i32)).collect::<Vec<_>>()
                            })
                            .collect();

                        // Wrap the result in Some if successful
                        Some(CompetitionResult {
                            id: key.to_string(),
                            results: solves,
                        })
                    })
                })
                .collect();


            Ok::<Vec<CompetitionResult>, Box<dyn Error>>(results)
        }
    });

    let query_results: Vec<Result<Vec<CompetitionResult>, _>> = join_all(futures).await;

    for result in query_results {
        match result {
            Ok(competitor) => competitor_results.push(competitor),
            Err(e) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Error fetching competitor data: {}", e),
                )))
            }
        }
    }

    Ok(competitor_results)
}

async fn fetch(
    url: String,
    client: Client,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let response = client.get(&url).send().await?;
    let json: serde_json::Value = response.json().await?;
    Ok(json)
}
