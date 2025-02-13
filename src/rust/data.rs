use chrono::{DateTime, Datelike, Duration, NaiveDateTime, TimeZone, Utc};
use futures::future::join_all;
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Debug)]
pub struct CompetitionResult {
    pub id: String,
    pub results: Vec<i32>,
}

pub struct PersonResult {
    pub name: String,
    pub results: Vec<CompetitionResult>,
}

#[derive(Deserialize, Debug)]
struct RequestCompetitions {
    items: Vec<RequestCompetition>,
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

#[derive(Deserialize, Debug)]
struct RequestPerson {
    name: String,
    results: HashMap<String, HashMap<String, Vec<RequestCompetitionResult>>>,
}

#[derive(Deserialize, Debug)]
struct RequestCompetitionResult {
    solves: Vec<i32>,
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

            let response = fetch::<RequestCompetitions>(url, &client).await?;
            let competition_list = response.items;

            let competitions: Result<HashMap<String, i64>, &'static str> = competition_list
                .iter()
                .filter_map(|comp| {

                    let date_str = &comp.date.from;
                    let naive_date = NaiveDateTime::parse_from_str(
                        &format!("{} 00:00:00", date_str),
                        "%Y-%m-%d %H:%M:%S",
                    ).ok()?;

                    let datetime = Utc.from_utc_datetime(&naive_date);
                    let date = datetime.timestamp();

                    if date < cutoff_timestamp || date > today_timestamp {
                        return None;
                    }

                    Some(Ok((comp.id.clone(), date)))
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
) -> Result<Vec<PersonResult>, &'static str> {
    let o_client = reqwest::Client::new();

    let futures = competitors.iter().map(|competitor| {
        let event_ref = &event;
        let client = o_client.clone();
        async move {
            let url = format!("https://raw.githubusercontent.com/robiningelbrecht/wca-rest-api/master/api/persons/{}.json", competitor);
            let response = fetch::<RequestPerson>(url, &client).await?;

            let results: Vec<CompetitionResult> = response.results
                .into_iter()
                .filter_map(|(key, value)| {
                    value.get(event_ref).and_then(|event_data| {
                        let solves: Vec<i32> = event_data
                            .into_iter()
                            .flat_map(|round| {
                                &round.solves
                            })
                            .cloned()
                            .collect();

                        Some(CompetitionResult {
                            id: key.to_string(),
                            results: solves,
                        })
                    })
                })
                .collect();

            Ok::<(String, Vec<CompetitionResult>), &'static str>((response.name, results))
        }
    });

    let query_results = join_all(futures).await;

    let mut competitor_results = Vec::new();
    for result in query_results {
        match result {
            Ok(results) => competitor_results.push(PersonResult {
                name: results.0,
                results: results.1,
            }),
            Err(_) => return Err("Error fetching competitor data"),
        }
    }

    Ok(competitor_results)
}

async fn fetch<T: DeserializeOwned>(url: String, client: &Client) -> Result<T, &'static str> {
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|_| "Failed to fetch data")?;

    let json: T = response.json().await.map_err(|_| "Failed to parse JSON")?;

    Ok(json)
}
