use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::js_sys::Promise;

mod data;
use data::{get_competition_data, get_solve_data, PersonResult};

mod simulation;
use simulation::{run_simulations, SimulationResult};

#[derive(Serialize, Deserialize, Debug)]
pub struct DatedCompetitionResult {
    pub date: i64,
    pub results: Vec<i32>,
}

#[derive(Serialize)]

pub struct SimulationReturn {
    name: String,
    results: SimulationResult,
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
                console_log!("Error fetching and joining data: {:?}", e);
                serde_wasm_bindgen::to_value(&format!("Error: {:?}", e)).unwrap()
            })?;

        let (names, solve_data): (Vec<String>, _) = parsed_data.into_iter().unzip();
        let simulated_data = run_simulations(num_simulations, solve_data);

        let results: Vec<_> = names
            .into_iter()
            .zip(simulated_data)
            .map(|(name, results)| SimulationReturn { name, results })
            .collect();

        serde_wasm_bindgen::to_value(&results)
            .map_err(|_| serde_wasm_bindgen::to_value("Error serializing results").unwrap())
    };

    wasm_bindgen_futures::future_to_promise(future)
}

fn join_data(
    competitions: HashMap<String, i64>,
    results: Vec<PersonResult>,
) -> Vec<(String, Vec<DatedCompetitionResult>)> {
    results
        .into_iter()
        .map(|competitor| {
            let results = competitor
                .results
                .into_iter()
                .filter_map(|competition| {
                    let comp_date = competitions.get(&competition.id)?;
                    Some(DatedCompetitionResult {
                        date: *comp_date,
                        results: competition.results,
                    })
                })
                .collect::<Vec<_>>();

            (competitor.name, results)
        })
        .collect()
}

pub async fn fetch_and_join(
    competitors: Vec<String>,
    event: String,
    month_cutoff: i32,
) -> Result<Vec<(String, Vec<DatedCompetitionResult>)>, &'static str> {
    let competitions = get_competition_data(month_cutoff).await?;
    let results = get_solve_data(competitors, event).await?;
    Ok(join_data(competitions, results))
}
