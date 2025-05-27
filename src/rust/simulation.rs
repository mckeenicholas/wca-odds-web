use rand::rng;
use rand::rngs::ThreadRng;
use serde::Serialize;
use std::collections::HashMap;
use std::iter::zip;

use crate::calc::{find_lowest_indices, transpose_solves};
use crate::competitor::Competitor;
use crate::event::EventType;
use crate::event_simulator::{Ao5Simulation, Bo3Simulation, EventSimulation, Mo3Simulation};
use crate::histogram::Histogram;

#[derive(Serialize, Debug)]
pub struct SimulationWASMOutput {
    name: String,
    sample_size: u32,
    win_count: u32,
    pod_count: u32,
    total_rank: u32,
    mean_no_dnf: u32,
    rank_dist: Vec<u32>,
    hist_values_single: HashMap<i32, i32>,
    hist_values_average: HashMap<i32, i32>,
}

pub struct CompetitionSimulator {
    event_simulator: Box<dyn EventSimulation>,
    competitors_data: Vec<Competitor>,
    simulation_results: Option<Vec<SimulationResult>>,
    rng: ThreadRng,
}

pub struct RuntimeConfig {
    pub num_simulations: u32,
    pub include_dnf: bool,
}

#[derive(Debug)]
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
    pub fn new(num_competitors: usize, hist_min: i32, hist_max: i32) -> Self {
        let histograms = ResultHistograms {
            hist_single: Histogram::new_with_bounds(hist_min, hist_max),
            hist_average: Histogram::new_with_bounds(hist_min, hist_max),
        };

        Self {
            win_count: 0,
            pod_count: 0,
            total_rank: 0,
            rank_dist: vec![0; num_competitors],
            histograms,
        }
    }
}

impl CompetitionSimulator {
    pub fn new(event: EventType, competitors: Vec<Competitor>) -> Self {
        let event_simulator: Box<dyn EventSimulation> = match event {
            EventType::Ao5(_) => Box::new(Ao5Simulation),
            EventType::Mo3(mo3_event) => Box::new(Mo3Simulation { event: mo3_event }),
            EventType::Bo3(_) => Box::new(Bo3Simulation),
        };

        Self {
            event_simulator,
            competitors_data: competitors,
            simulation_results: None,
            rng: rng(),
        }
    }

    fn get_default_results(&self) -> Vec<SimulationResult> {
        let (hist_min, hist_max) = self
            .competitors_data
            .iter()
            .fold((i32::MAX, 0), |acc, comp| {
                let (min, max) = comp.get_person_hist_bounds();
                (min.min(acc.0), max.max(acc.1))
            });

        let num_competitors = self.competitors_data.len();

        (0..num_competitors)
            .map(|_| SimulationResult::new(num_competitors, hist_min, hist_max))
            .collect()
    }

    pub fn run_simulations(&mut self, config: &mut RuntimeConfig) {
        self.simulation_results = Some(self.get_default_results());

        for _ in 0..config.num_simulations / 4 {
            let sim_results = self.run_simulation_batch(config);
            self.update_rankings(sim_results);
        }
    }

    pub fn generate_wasm_output(&mut self) -> Vec<SimulationWASMOutput> {
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
                hist_values_single: results.histograms.hist_single.data(),
                hist_values_average: results.histograms.hist_average.data(),
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

        for avg_by_competitor in solves_by_sim {
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

    pub fn set_entered_results(&mut self, entered_times: Vec<Vec<i32>>) {
        for (competitor, average) in zip(&mut self.competitors_data, entered_times) {
            competitor.add_entered_results(average);
        }
    }
}
