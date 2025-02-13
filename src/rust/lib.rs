use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::js_sys::Promise;

mod data;
use data::{get_competition_data, get_solve_data, PersonResult};

mod simulation;
use simulation::{run_simulations, SimulationResult};

mod calc;
mod simd;

lazy_static! {
    static ref EVENT_MAPPINGS: HashMap<&'static str, EventType> = {
        let mut m = HashMap::new();
        // Ao5 events
        ["222", "333", "444", "555", "333oh", "skewb", "pyram", "minx", "clock", "sq1"]
            .iter()
            .for_each(|&id| { m.insert(id, EventType::Ao5); });
        // Mo3 events
        ["666", "777", "333fm"]
            .iter()
            .for_each(|&id| { m.insert(id, EventType::Mo3); });
        // Bo3 events
        ["333bf", "444bf", "555bf"]
            .iter()
            .for_each(|&id| { m.insert(id, EventType::Bo3); });
        m
    };
}

#[derive(Debug, Copy, Clone)]
pub enum EventType {
    Ao5,
    Mo3,
    Bo3,
}

impl EventType {
    fn from_event_id(event_id: &str) -> Option<Self> {
        EVENT_MAPPINGS.get(event_id).cloned()
    }
}

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
    event_str: String,
    month_cutoff: i32,
    num_simulations: u32,
) -> Promise {
    let event_type = EventType::from_event_id(&event_str).expect("Invalid event");

    let future = async move {
        let parsed_data = fetch_and_join(competitors, event_str, month_cutoff)
            .await
            .map_err(|e| {
                console_log!("Error fetching and joining data: {:?}", e);
                serde_wasm_bindgen::to_value(&format!("Error: {:?}", e)).unwrap()
            })?;

        let (names, solve_data): (Vec<String>, _) = parsed_data.into_iter().unzip();
        let simulated_data = run_simulations(num_simulations, solve_data, event_type);

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
