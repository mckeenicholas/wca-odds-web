use chrono::{DateTime, Datelike, Duration, NaiveDateTime, TimeZone, Utc};
use futures::future::join_all;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Debug)]
pub struct CompetitionResult {
    pub id: String,
    pub results: Vec<i32>,
}

#[derive(Deserialize, Debug)]
struct RequestCompetition {
    id: String,
    date: RequestCompetitionDate, // {"from": "YYYY-MM-DD"}
}

#[derive(Deserialize, Debug)]
struct RequestCompetitionDate {
    from: String,
}

pub async fn get_competition_data(month_cutoff: i32) -> Result<HashMap<String, i64>, &'static str> {
    let now: DateTime<Utc> = Utc::now();
    let current_year = now.year();

    let months_duration = Duration::days((month_cutoff * 31) as i64);
    let start_date = now - months_duration;
    let cutoff_timestamp = start_date.timestamp();
    let today_timestamp = now.timestamp();
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

            let response = fetch(url, client).await?;
            let competitions_json = response.get("items")
                .and_then(Value::as_array)
                .ok_or("Expected 'items' as an array")?;

            let competitions: Result<HashMap<String, i64>, &'static str> = competitions_json
                .iter()
                .filter_map(|comp_json| {
                    let comp: RequestCompetition = serde_json::from_value(comp_json.clone()).ok()?;
                    let date_str = comp.date.from;

                    let naive_date = NaiveDateTime::parse_from_str(
                        &format!("{} 00:00:00", date_str),
                        "%Y-%m-%d %H:%M:%S",
                    ).ok()?;

                    let datetime = Utc.from_utc_datetime(&naive_date);
                    let date = datetime.timestamp();

                    if date < cutoff_timestamp || date > today_timestamp {
                        return None;
                    }

                    Some(Ok((comp.id, date)))
                })
                .collect();

            competitions
        }
    });

    let results = join_all(futures).await;
    for result in results {
        match result {
            Ok(competitions) => all_competitions.extend(competitions),
            Err(_) => return Err("Error fetching competition data"),
        }
    }

    Ok(all_competitions)
}

pub async fn get_solve_data(
    competitors: Vec<String>,
    event: String,
) -> Result<Vec<Vec<CompetitionResult>>, &'static str> {
    let mut competitor_results: Vec<Vec<CompetitionResult>> = Vec::new();
    let o_client = reqwest::Client::new();

    let futures = competitors.iter().map(|competitor| {
        let event_ref = &event;
        let client = o_client.clone();
        async move {
            let url = format!("https://raw.githubusercontent.com/robiningelbrecht/wca-rest-api/master/api/persons/{}.json", competitor);
            let response = fetch(url, client).await?;

            let results_json = response.get("results")
                .and_then(|v| v.as_object())
                .ok_or("Missing or invalid 'results' field")?;

            let results: Vec<CompetitionResult> = results_json
                .iter()
                .filter_map(|(key, value)| {
                    value.get(event_ref).and_then(|event_data| {
                        let solves: Vec<i32> = event_data
                            .as_array()?
                            .iter()
                            .flat_map(|round| {
                                round
                                    .get("solves")
                                    .and_then(|s| s.as_array())
                                    .unwrap_or(&vec![])
                                    .iter()
                                    .filter_map(|solve| solve.as_i64().map(|x| x as i32))
                                    .collect::<Vec<_>>()
                            })
                            .collect();

                        Some(CompetitionResult {
                            id: key.to_string(),
                            results: solves,
                        })
                    })
                })
                .collect();

            Ok::<Vec<CompetitionResult>, &'static str>(results)
        }
    });

    let query_results: Vec<Result<Vec<CompetitionResult>, _>> = join_all(futures).await;
    for result in query_results {
        match result {
            Ok(competitor) => competitor_results.push(competitor),
            Err(_) => return Err("Error fetching competitor data"),
        }
    }

    Ok(competitor_results)
}

async fn fetch(url: String, client: Client) -> Result<serde_json::Value, &'static str> {
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|_| "Failed to fetch data")?;
    let json: serde_json::Value = response.json().await.map_err(|_| "Failed to parse JSON")?;
    Ok(json)
}
