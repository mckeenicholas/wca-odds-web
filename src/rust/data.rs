use crate::{
    competitor::{Competitor, DatedCompetitionResult},
    event::{EventType, Mo3Event},
};
use chrono::{Datelike, TimeZone, Utc};
use futures::future::join_all;
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct APIRequestCompetitions {
    items: Vec<APIRequestCompetition>,
}

#[derive(Deserialize, Debug)]
struct APIRequestCompetition {
    id: String,
    date: APIRequestCompetitionDate,
}

#[derive(Deserialize, Debug)]
struct APIRequestCompetitionDate {
    from: String, // YYYY-MM-DD
}

#[derive(Deserialize, Debug)]
struct APIRequestPerson {
    name: String,
    results: HashMap<String, HashMap<String, Vec<APIRequestCompetitionResult>>>,
}

#[derive(Deserialize, Debug)]
struct APIRequestCompetitionResult {
    solves: Vec<i32>,
}

pub struct ParsedCompetitionResult {
    pub id: String,
    pub results: Vec<i32>,
}

pub struct ParsedPersonResult {
    pub name: String,
    pub results: Vec<ParsedCompetitionResult>,
}

pub struct CompetitionDataManager {
    competitors: Vec<String>,
    event: EventType,
    start_date: i64,
    end_date: i64,
    halflife: f32,
    request_client: Client,
}

struct TimeRange {
    cutoff_timestamp: i64,
    today_timestamp: i64,
    years: Vec<i32>,
}

impl TimeRange {
    fn new(start_date_js: i64, end_date_js: i64) -> Self {
        // Convert from milliseconds to seconds since JavaScript uses milliseconds
        let start_timestamp = start_date_js / 1000;
        let end_timestamp = end_date_js / 1000;

        let start_datetime = Utc
            .timestamp_opt(start_timestamp, 0)
            .single()
            .unwrap_or_else(Utc::now);
        let end_datetime = Utc
            .timestamp_opt(end_timestamp, 0)
            .single()
            .unwrap_or_else(Utc::now);

        // Generate range of years from start to end
        let years: Vec<i32> = (start_datetime.year()..=end_datetime.year()).collect();

        Self {
            cutoff_timestamp: start_timestamp,
            today_timestamp: end_timestamp,
            years,
        }
    }

    fn in_time_range(&self, time_utc: i64) -> bool {
        time_utc > self.cutoff_timestamp && time_utc < self.today_timestamp
    }

    fn days_from_cutoff(&self, time_utc: i64) -> i32 {
        const SECONDS_PER_DAY: i64 = 60 * 60 * 24;

        ((self.today_timestamp - time_utc) / SECONDS_PER_DAY) as i32
    }
}

impl CompetitionDataManager {
    pub fn create(
        competitors: Vec<String>,
        event: EventType,
        start_date: i64,
        end_date: i64,
        halflife: f32,
    ) -> Self {
        Self {
            competitors,
            event,
            start_date,
            end_date,
            halflife,
            request_client: Client::new(),
        }
    }

    pub async fn fetch_all(&self) -> Result<Vec<Competitor>, &'static str> {
        let competitions = self.get_competition_data().await?;
        let results = self.get_solve_data().await?;

        Ok(self.join_data(competitions, results))
    }

    async fn get_competition_data(&self) -> Result<HashMap<String, i32>, &'static str> {
        let time_range = TimeRange::new(self.start_date, self.end_date);

        let futures: Vec<_> = time_range
            .years
            .iter()
            .map(|&year| self.fetch_competitions_for_year(year, &time_range))
            .collect();

        self.merge_competition_results(join_all(futures).await)
    }

    async fn fetch_competitions_for_year(
        &self,
        year: i32,
        time_range: &TimeRange,
    ) -> Result<HashMap<String, i32>, &'static str> {
        let url = format!(
            "https://raw.githubusercontent.com/robiningelbrecht/wca-rest-api/master/api/competitions/{}.json",
            year
        );

        let response = self.fetch::<APIRequestCompetitions>(url).await?;
        self.collect_competitions(response.items, time_range)
    }

    fn collect_competitions(
        &self,
        competition_list: Vec<APIRequestCompetition>,
        time_range: &TimeRange,
    ) -> Result<HashMap<String, i32>, &'static str> {
        competition_list
            .iter()
            .filter_map(|comp| {
                let comp_timestamp = self.parse_competition_date(&comp.date.from)?;

                if !time_range.in_time_range(comp_timestamp) {
                    return None;
                }

                let days_since_comp = time_range.days_from_cutoff(comp_timestamp);

                Some(Ok((comp.id.clone(), days_since_comp)))
            })
            .collect()
    }

    fn parse_competition_date(&self, date_str: &str) -> Option<i64> {
        let parts: Vec<&str> = date_str.split('-').collect();
        if parts.len() != 3 {
            return None;
        }

        let year: i32 = parts[0].parse().ok()?;
        let month: u32 = parts[1].parse().ok()?;
        let day: u32 = parts[2].parse().ok()?;

        Utc.with_ymd_and_hms(year, month, day, 0, 0, 0)
            .single()
            .map(|dt| dt.timestamp())
    }

    fn merge_competition_results(
        &self,
        results: Vec<Result<HashMap<String, i32>, &'static str>>,
    ) -> Result<HashMap<String, i32>, &'static str> {
        let mut all_competitions = HashMap::new();

        for result in results {
            match result {
                Ok(competitions) => all_competitions.extend(competitions),
                Err(_) => return Err("Error fetching competition data"),
            }
        }

        Ok(all_competitions)
    }

    async fn get_solve_data(&self) -> Result<Vec<ParsedPersonResult>, &'static str> {
        let futures: Vec<_> = self
            .competitors
            .iter()
            .map(|competitor| self.fetch_competitor_data(competitor))
            .collect();

        let results = join_all(futures).await;
        self.collect_solve_results(results)
    }

    async fn fetch_competitor_data(
        &self,
        competitor: &str,
    ) -> Result<(String, Vec<ParsedCompetitionResult>), &'static str> {
        let url = format!(
            "https://raw.githubusercontent.com/robiningelbrecht/wca-rest-api/master/api/persons/{}.json",
            competitor
        );

        let response = self.fetch::<APIRequestPerson>(url).await?;
        let results = self.extract_competitor_results(&response);

        Ok((response.name, results))
    }

    fn extract_competitor_results(
        &self,
        response: &APIRequestPerson,
    ) -> Vec<ParsedCompetitionResult> {
        response
            .results
            .iter()
            .filter_map(|(comp_id, rounds)| {
                rounds
                    .get(self.event.id())
                    .map(|event_data| ParsedCompetitionResult {
                        id: comp_id.to_string(),
                        results: self.process_event_data(event_data),
                    })
            })
            .collect()
    }

    fn process_event_data(&self, event_data: &[APIRequestCompetitionResult]) -> Vec<i32> {
        event_data
            .iter()
            .flat_map(|round| &round.solves)
            .map(|solve| {
                if self.event == EventType::Mo3(Mo3Event::F333) {
                    solve * 100
                } else {
                    *solve
                }
            })
            .collect()
    }

    fn collect_solve_results(
        &self,
        results: Vec<Result<(String, Vec<ParsedCompetitionResult>), &'static str>>,
    ) -> Result<Vec<ParsedPersonResult>, &'static str> {
        results
            .into_iter()
            .map(|result| result.map(|(name, results)| ParsedPersonResult { name, results }))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| "Error fetching competitor data")
    }

    async fn fetch<T: DeserializeOwned>(&self, url: String) -> Result<T, &'static str> {
        let response = self
            .request_client
            .get(&url)
            .send()
            .await
            .map_err(|_| "Failed to fetch data")?;

        let json: T = response.json().await.map_err(|_| "Failed to parse JSON")?;

        Ok(json)
    }

    fn join_data(
        &self,
        competitions: HashMap<String, i32>,
        results: Vec<ParsedPersonResult>,
    ) -> Vec<Competitor> {
        results
            .into_iter()
            .map(|competitor| {
                let results = competitor
                    .results
                    .into_iter()
                    .filter_map(|competition| {
                        let days_since = competitions.get(&competition.id)?;

                        Some(DatedCompetitionResult {
                            days_since: *days_since,
                            results: competition.results,
                        })
                    })
                    .collect::<Vec<_>>();

                Competitor::new(competitor.name, results, self.halflife)
            })
            .collect()
    }
}
