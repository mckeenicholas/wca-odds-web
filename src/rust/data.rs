use chrono::{Datelike, Duration, NaiveDateTime, TimeZone, Utc};
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

fn parse_competition_date(date_str: &str) -> Option<i64> {
    NaiveDateTime::parse_from_str(&format!("{} 00:00:00", date_str), "%Y-%m-%d %H:%M:%S")
        .ok()
        .map(|naive_date| Utc.from_utc_datetime(&naive_date).timestamp())
}

async fn fetch_competitions_for_year(
    year: i32,
    client: &Client,
    cutoff_timestamp: i64,
    today_timestamp: i64,
) -> Result<HashMap<String, i64>, &'static str> {
    let url = format!(
        "https://raw.githubusercontent.com/robiningelbrecht/wca-rest-api/master/api/competitions/{}.json",
        year
    );

    let response = fetch::<RequestCompetitions>(url, client).await?;
    collect_competitions(response.items, cutoff_timestamp, today_timestamp)
}

fn collect_competitions(
    competition_list: Vec<RequestCompetition>,
    cutoff_timestamp: i64,
    today_timestamp: i64,
) -> Result<HashMap<String, i64>, &'static str> {
    competition_list
        .iter()
        .filter_map(|comp| {
            let date = parse_competition_date(&comp.date.from)?;
            if date < cutoff_timestamp || date > today_timestamp {
                return None;
            }
            Some(Ok((comp.id.clone(), date)))
        })
        .collect()
}

pub async fn get_competition_data(month_cutoff: i32) -> Result<HashMap<String, i64>, &'static str> {
    let (cutoff_timestamp, today_timestamp, years) = calculate_time_range(month_cutoff);
    let client = reqwest::Client::new();

    let futures: Vec<_> = years
        .iter()
        .map(|&year| fetch_competitions_for_year(year, &client, cutoff_timestamp, today_timestamp))
        .collect();

    merge_competition_results(join_all(futures).await)
}

fn calculate_time_range(month_cutoff: i32) -> (i64, i64, Vec<i32>) {
    let now = Utc::now();
    let months_duration = Duration::days((month_cutoff * 31) as i64);
    let start_date = now - months_duration;

    (
        start_date.timestamp(),
        now.timestamp(),
        (start_date.year()..=now.year()).collect(),
    )
}

fn merge_competition_results(
    results: Vec<Result<HashMap<String, i64>, &'static str>>,
) -> Result<HashMap<String, i64>, &'static str> {
    let mut all_competitions = HashMap::new();

    for result in results {
        match result {
            Ok(competitions) => all_competitions.extend(competitions),
            Err(_) => return Err("Error fetching competition data"),
        }
    }

    Ok(all_competitions)
}

async fn fetch_competitor_data(
    competitor: &str,
    event: &str,
    client: &Client,
) -> Result<(String, Vec<CompetitionResult>), &'static str> {
    let url = format!(
        "https://raw.githubusercontent.com/robiningelbrecht/wca-rest-api/master/api/persons/{}.json",
        competitor
    );

    let response = fetch::<RequestPerson>(url, client).await?;
    let results = extract_competitor_results(&response, event);

    Ok((response.name, results))
}

fn extract_competitor_results(response: &RequestPerson, event: &str) -> Vec<CompetitionResult> {
    response
        .results
        .iter()
        .filter_map(|(comp_id, rounds)| {
            rounds.get(event).map(|event_data| CompetitionResult {
                id: comp_id.to_string(),
                results: event_data
                    .iter()
                    .flat_map(|round| &round.solves)
                    .cloned()
                    .collect(),
            })
        })
        .collect()
}

pub async fn get_solve_data(
    competitors: Vec<String>,
    event: String,
) -> Result<Vec<PersonResult>, &'static str> {
    let client = reqwest::Client::new();

    let futures: Vec<_> = competitors
        .iter()
        .map(|competitor| fetch_competitor_data(competitor, &event, &client))
        .collect();

    let results = join_all(futures).await;
    collect_solve_results(results)
}

fn collect_solve_results(
    results: Vec<Result<(String, Vec<CompetitionResult>), &'static str>>,
) -> Result<Vec<PersonResult>, &'static str> {
    results
        .into_iter()
        .map(|result| result.map(|(name, results)| PersonResult { name, results }))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| "Error fetching competitor data")
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
