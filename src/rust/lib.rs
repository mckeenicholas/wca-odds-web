use std::cell::RefCell;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
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

fn str_to_jsval(msg: &str) -> JsValue {
    serde_wasm_bindgen::to_value(msg).unwrap()
}

#[wasm_bindgen]
pub fn load_data(
    competitors: Vec<String>,
    event_str: String,
    start_date: i64,
    end_date: i64,
    halflife: f32,
) -> Promise {
    let event_type = match EventType::from_event_id(&event_str) {
        Some(event) => event,
        None => return str_to_jsval("Invalid event type.").into(),
    };

    let data_manager =
        CompetitionDataManager::create(competitors, event_type, start_date, end_date, halflife);

    let future = async move {
        let competitors_result = data_manager.fetch_all().await;

        let competitors = match competitors_result {
            Ok(fetch_data) => fetch_data,
            Err(e) => return Ok(serde_wasm_bindgen::to_value(&format!("Error: {e:?}")).unwrap()),
        };

        let simulator = CompetitionSimulator::new(event_type, competitors);

        APP_STATE.with(|simulation_manager| {
            simulation_manager.set_simulation_manager(simulator);
        });

        serde_wasm_bindgen::to_value(&true)
            .map_err(|_| str_to_jsval("Error serializing return value"))
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
        let sim_manager = match sim_manager_ref.as_mut() {
            Some(data) => data,
            None => return str_to_jsval("Simulation data not loaded"),
        };

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
