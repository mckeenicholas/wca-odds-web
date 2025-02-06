use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use wasm_bindgen::prelude::*;
use web_sys::js_sys::Promise;

mod data;
use data::{get_competition_data, get_solve_data, CompetitionResult};

mod simulation;
use simulation::run_simulations;

#[derive(Serialize, Deserialize, Debug)]
pub struct DatedCompetitionResult {
    pub date: i64,
    pub results: Vec<i32>,
}

#[derive(Serialize)]
pub struct PersonData {
    pub wins: i32,
    pub podiums: i32,
    pub mean: f64,
    pub gamma: f64,
    pub stdev: f64,
    pub mu: f64,
    pub sigma: f64,
    pub tau: f64,
    pub dnf_rate: f64,
    pub avg_rank: f64,
    pub ranks: Vec<i32>,
}

#[derive(Serialize)]
pub struct ReturnData {
    pub persons: Vec<PersonData>,
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!($($t)*).as_str()));
    }
}

#[wasm_bindgen]
pub fn run_odds_simulation(
    competitors: Vec<String>,
    event: String,
    month_cutoff: i32,
    num_simulations: u32,
) -> Promise {
    let future = async move {
        let parsed_data = fetch_and_join(competitors, event, month_cutoff)
            .await
            .map_err(|e| {
                serde_wasm_bindgen::to_value(&format!("{}", e)).expect("Error parsing message")
            })?;

        let mut results = run_simulations(num_simulations, parsed_data);

        

        serde_wasm_bindgen::to_value(&results)
            .map_err(|e| serde_wasm_bindgen::to_value(&format!("{}", e)).unwrap())
    };

    wasm_bindgen_futures::future_to_promise(future)
}

fn join_data(
    competitions: HashMap<String, i64>,
    results: Vec<Vec<CompetitionResult>>,
) -> Vec<Vec<DatedCompetitionResult>> {
    results
        .into_iter()
        .map(|competitor| {
            competitor
                .into_iter()
                .filter_map(|competition| match competitions.get(&competition.id) {
                    Some(comp_date) => Some(DatedCompetitionResult {
                        date: *comp_date,
                        results: competition.results,
                    }),
                    None => None,
                })
                .collect()
        })
        .collect()
}

pub async fn fetch_and_join(
    competitors: Vec<String>,
    event: String,
    month_cutoff: i32,
) -> Result<Vec<Vec<DatedCompetitionResult>>, Box<dyn Error>> {
    let competitions = get_competition_data(month_cutoff).await?;
    let results = get_solve_data(competitors, event).await?;

    Ok(join_data(competitions, results))
}
