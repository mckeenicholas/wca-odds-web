use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

        let results = run_simulations(num_simulations, parsed_data);

        serde_wasm_bindgen::to_value(&results)
            .map_err(|_| serde_wasm_bindgen::to_value("Error serializing results").unwrap())
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
                .filter_map(|competition| {
                    let comp_date = competitions.get(&competition.id)?;
                    Some(DatedCompetitionResult {
                        date: *comp_date,
                        results: competition.results,
                    })
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

pub async fn fetch_and_join(
    competitors: Vec<String>,
    event: String,
    month_cutoff: i32,
) -> Result<Vec<Vec<DatedCompetitionResult>>, &'static str> {
    let competitions = get_competition_data(month_cutoff).await?;
    let results = get_solve_data(competitors, event).await?;
    Ok(join_data(competitions, results))
}
