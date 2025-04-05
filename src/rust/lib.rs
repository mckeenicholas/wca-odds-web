use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::js_sys::Promise;

mod calc;
mod data;
mod event;
mod event_simulation;
mod simd;
mod simulation;

use data::{get_competition_data, get_solve_data, PersonResult};
use event::{Ao5Event, EventType};
use simulation::{run_simulations, SimulationResult};

thread_local! {
    static APP_STATE: AppState = AppState::new();
}

#[derive(Debug)]
pub struct Competitor {
    pub name: String,
    pub results: Vec<DatedCompetitionResult>,
    pub entered_results: Vec<i32>,
}

#[derive(Debug)]
pub struct AppState {
    competitors: RefCell<Vec<Competitor>>,
    event: RefCell<EventType>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            competitors: RefCell::new(vec![]),
            event: RefCell::new(EventType::Ao5(Ao5Event::S333)),
        }
    }

    pub fn set_event(&self, event: EventType) {
        *self.event.borrow_mut() = event;
    }

    pub fn set_competitors(&self, competitors: Vec<Competitor>) {
        *self.competitors.borrow_mut() = competitors;
    }

    pub fn get_event(&self) -> EventType {
        *self.event.borrow()
    }

    pub fn get_competitors(&self) -> &RefCell<Vec<Competitor>> {
        &self.competitors
    }

    pub fn with<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Vec<Competitor>, &EventType) -> R,
    {
        let competitors = self.competitors.borrow();
        let event = self.event.borrow();
        f(&competitors, &event)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatedCompetitionResult {
    pub date: i64,
    pub results: Vec<i32>,
}

#[derive(Serialize)]
pub struct SimulationReturn {
    name: String,
    results: SimulationResult,
    sample_size: u32,
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!($($t)*).as_str()));
    }
}

#[wasm_bindgen]
pub fn load_data(competitors: Vec<String>, event_str: String, month_cutoff: i32) -> Promise {
    let event_type = EventType::from_event_id(&event_str).expect("Invalid event");

    APP_STATE.with(|state| {
        state.set_event(event_type);
    });

    let future = async move {
        let parsed_data = fetch_and_join(competitors, event_type, month_cutoff)
            .await
            .map_err(|e| {
                console_log!("Error fetching and joining data: {:?}", e);
                serde_wasm_bindgen::to_value(&format!("Error: {:?}", e)).unwrap()
            })?;

        APP_STATE.with(|state| {
            state.set_competitors(parsed_data);
        });

        Ok(serde_wasm_bindgen::to_value(&true)
            .map_err(|_| serde_wasm_bindgen::to_value("Error serializing return value").unwrap())?)
    };

    wasm_bindgen_futures::future_to_promise(future)
}

#[wasm_bindgen]
pub fn run_simulation(
    num_simulations: u32,
    include_dnf: bool,
    entered_times_jsval: JsValue,
) -> JsValue {
    let entered_times: Vec<Vec<i32>> =
        serde_wasm_bindgen::from_value(entered_times_jsval).expect("Invalid input");

    APP_STATE.with(|state| {
        let mut competitors = state.get_competitors().borrow_mut();

        competitors
            .iter_mut()
            .zip(entered_times)
            .for_each(|(ref mut competitor, times)| {
                competitor.entered_results = times;
            });

        let simulated_data = run_simulations(
            num_simulations,
            &competitors,
            state.get_event(),
            include_dnf,
        );

        let results: Vec<_> = competitors
            .iter()
            .zip(simulated_data)
            .map(|(competitor, results)| SimulationReturn {
                name: competitor.name.clone(),
                results,
                sample_size: competitor.results.len() as u32
                    * state.event.borrow().num_attempts() as u32,
            })
            .collect();

        serde_wasm_bindgen::to_value(&results).unwrap()
    })
}

fn join_data(competitions: HashMap<String, i64>, results: Vec<PersonResult>) -> Vec<Competitor> {
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

            Competitor {
                name: competitor.name,
                results,
                entered_results: vec![],
            }
        })
        .collect()
}

pub async fn fetch_and_join(
    competitors: Vec<String>,
    event: EventType,
    month_cutoff: i32,
) -> Result<Vec<Competitor>, &'static str> {
    let competitions = get_competition_data(month_cutoff).await?;
    let results = get_solve_data(competitors, event).await?;
    Ok(join_data(competitions, results))
}
