use rand::rngs::ThreadRng;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

use crate::calc::{
    calc_weighted_mean_variance_stdev, find_lowest_indices, fit_weighted_skewnorm,
    transpose_solves, trim_weighted_results,
};
use crate::competitor::Competitor;
use crate::event::EventType;
use crate::event_simulator::{Ao5Simulation, Bo3Simulation, EventSimulation, Mo3Simulation};
use crate::histogram::Histogram;

use core::arch::wasm32::v128;
use std::default;

#[derive(Serialize)]
pub struct SimulationWASMOutput {
    name: String,
    sample_size: u32,
    win_count: u32,
    pod_count: u32,
    total_rank: u32,
    mean_no_dnf: u32,
    rank_dist: Vec<u32>,
    hist_values_single: Vec<(i32, i32)>,
    hist_values_average: Vec<(i32, i32)>,
}

pub struct CompetitionSimulator {
    event: EventType,
    event_simulator: Box<dyn EventSimulation>,
    competitors_data: Vec<Competitor>,
    simulation_results: Option<Vec<SimulationResult>>,
    rng: ThreadRng,
}

pub struct RuntimeConfig {
    pub num_simulations: u32,
    pub include_dnf: bool,
    pub decay_halflife_days: f32,
}

#[derive(Default)]
pub struct ResultHistograms {
    pub hist_single: Histogram,
    pub hist_average: Histogram,
}

pub struct SimulationResult {
    win_count: u32,
    pod_count: u32,
    total_rank: u32,
    rank_dist: Vec<u32>,
    histograms: ResultHistograms,
}

impl SimulationResult {
    pub fn new(num_competitors: usize) -> Self {
        Self {
            win_count: 0,
            pod_count: 0,
            total_rank: 0,
            rank_dist: vec![0; num_competitors],
            histograms: ResultHistograms::default(),
        }
    }
}

impl CompetitionSimulator {
    pub fn new(event: EventType, competitors: Vec<Competitor>) -> Self {
        let num_competitors = competitors.len();

        let event_simulator: Box<dyn EventSimulation> = match event {
            EventType::Ao5(_) => Box::new(Ao5Simulation),
            EventType::Mo3(mo3_event) => Box::new(Mo3Simulation { event: mo3_event }),
            EventType::Bo3(_) => Box::new(Bo3Simulation),
        };

        let default_simulation_results = (0..num_competitors)
            .map(|_| SimulationResult::new(num_competitors))
            .collect();

        Self {
            event,
            event_simulator,
            competitors_data: competitors,
            simulation_results: Some(default_simulation_results),
            rng: thread_rng(),
        }
    }

    pub fn run_simulations(&mut self, config: &mut RuntimeConfig) {
        for _ in 0..config.num_simulations {
            let sim_results = self.run_simulation_batch(config);
            self.update_rankings(sim_results);
        }
    }

    pub fn to_wasm_output(&mut self) -> Vec<SimulationWASMOutput> {
        let results: Vec<_> = self
            .competitors_data
            .iter()
            .zip(self.simulation_results.take().unwrap())
            .map(|(competitor, results)| SimulationWASMOutput {
                name: competitor.name.clone(),
                win_count: results.win_count,
                sample_size: competitor.get_sample_size(),
                pod_count: results.pod_count,
                total_rank: results.total_rank,
                mean_no_dnf: competitor.get_mean(),
                rank_dist: results.rank_dist,
                hist_values_single: results.histograms.hist_single.into(),
                hist_values_average: results.histograms.hist_average.into(),
            })
            .collect();

        results
    }

    fn run_simulation_batch(&mut self, config: &mut RuntimeConfig) -> Vec<[i32; 4]> {
        let sim_results = self.simulation_results.as_mut().unwrap();

        self.competitors_data
            .iter()
            .enumerate()
            .map(|(i, data)| {
                self.event_simulator.run_simulation(
                    data,
                    config,
                    &mut self.rng,
                    &mut sim_results[i].histograms,
                )
            })
            .collect()
    }

    fn update_rankings(&mut self, solve_results: Vec<[i32; 4]>) {
        let sim_results = self.simulation_results.as_mut().unwrap();

        let solves_by_sim = transpose_solves(solve_results);

        for i in 0..4 {
            let avg_by_competitor = &solves_by_sim[i];
            let indices = find_lowest_indices(avg_by_competitor.as_slice());

            // Update win count
            sim_results[indices[0]].win_count += 1;

            // Update podium counts
            for &index in indices.iter().take(3) {
                sim_results[index].pod_count += 1;
            }

            // Update rankings
            for (position, &competitor_index) in indices.iter().enumerate() {
                sim_results[competitor_index].rank_dist[position] += 1;
                sim_results[competitor_index].total_rank += (position as u32) + 1;
            }
        }
    }
}
