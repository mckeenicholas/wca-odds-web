use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use web_sys::js_sys::Promise;

use data::CompetitionDataManager;
use event::EventType;
use simulation::{CompetitionSimulator, RuntimeConfig};

mod calc;
mod competitor;
mod data;
mod event;
mod event_simulator;
mod histogram;
mod simd;
mod simulation;

#[macro_export]
#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!($($t)*).as_str()));
    }
}

thread_local! {
    static APP_STATE: AppState = AppState::new();
}

pub struct AppState {
    simulation_manager: RefCell<Option<CompetitionSimulator>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            simulation_manager: RefCell::new(None),
        }
    }

    pub fn set_simulation_manager(&self, simulator: CompetitionSimulator) {
        *self.simulation_manager.borrow_mut() = Some(simulator);
    }

    pub fn get_simulation_manager(&self) -> &RefCell<Option<CompetitionSimulator>> {
        &self.simulation_manager
    }

    pub fn with<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Option<CompetitionSimulator>) -> R,
    {
        let simulation_manager = self.get_simulation_manager().borrow();
        f(&simulation_manager)
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
pub fn load_data(
    competitors: Vec<String>,
    event_str: String,
    month_cutoff: i32,
    halflife: f32,
) -> Promise {
    let event_type = EventType::from_event_id(&event_str).expect("Invalid event");

    let data_manager =
        CompetitionDataManager::create(competitors, event_type, month_cutoff, halflife);

    let future = async move {
        let competitors_result = data_manager.fetch_all().await;

        let competitors = match competitors_result {
            Ok(fetch_data) => fetch_data,
            Err(e) => return Ok(serde_wasm_bindgen::to_value(&format!("Error: {:?}", e)).unwrap()),
        };

        let simulator = CompetitionSimulator::new(event_type, competitors);

        APP_STATE.with(|simulaiton_manager| {
            simulaiton_manager.set_simulation_manager(simulator);
        });

        serde_wasm_bindgen::to_value(&true)
            .map_err(|_| serde_wasm_bindgen::to_value("Error serializing return value").unwrap())
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
        let mut sim_manager_ref = state.get_simulation_manager().borrow_mut();
        let sim_manager = sim_manager_ref
            .as_mut()
            .expect("Simulation manager is not set. (Data likely not loaded yet).");

        sim_manager.set_entered_results(entered_times);

        let mut config = RuntimeConfig {
            include_dnf,
            num_simulations,
        };

        sim_manager.run_simulations(&mut config);

        let simulated_data = sim_manager.generate_wasm_output();

        serde_wasm_bindgen::to_value(&simulated_data).unwrap()
    })
}
