use rand::rngs::ThreadRng;

use crate::competitor::Competitor;
use crate::event::Mo3Event;
use crate::histogram::Histogram;
use crate::simd::{
    calc_wca_average_5, calc_wca_best_3, calc_wca_mean_3, generate_skewnorm_vec, i32x4_to_slice,
    i32x4_truncate_down_100, DNF_TEMP_VALUE,
};
use crate::simulation::{ResultHistograms, RuntimeConfig};
use core::arch::wasm32::v128;
use std::iter;

const AO5_SOLVE_COUNT: usize = 5;
const MO3_SOLVE_COUNT: usize = 3;
const BO3_SOLVE_COUNT: usize = 3;

pub trait EventSimulation {
    fn generate_solves(
        &self,
        competitor: &Competitor,
        config: &mut RuntimeConfig,
        rng: &mut ThreadRng,
    ) -> Vec<v128>;

    fn calculate_result(&self, solves: &[v128]) -> [i32; 4];

    fn run_simulation(
        &mut self,
        competitor: &Competitor,
        config: &mut RuntimeConfig,
        rng: &mut ThreadRng,
        histograms: &mut ResultHistograms,
    ) -> [i32; 4] {
        let solves: Vec<v128> = self.generate_solves(competitor, config, rng);

        for (&solve, entered) in iter::zip(&solves, &competitor.entered_results) {
            let solve_values = i32x4_to_slice(solve);

            // For now, don't include entered times in single histogram
            if *entered != 0 {
                continue;
            }

            self.add_to_histogram(&solve_values, &mut histograms.hist_single);
        }

        let averages = self.calculate_result(solves.as_slice());

        self.add_to_histogram(&averages, &mut histograms.hist_average);

        averages
    }

    fn add_to_histogram(&mut self, values: &[i32], histogram: &mut Histogram) {
        for &value in values {
            if value == i32::MAX || value == DNF_TEMP_VALUE {
                continue;
            }

            histogram.add_value(value / 10);
        }
    }
}

// Average of 5 simulation
pub struct Ao5Simulation;

impl EventSimulation for Ao5Simulation {
    fn generate_solves(
        &self,
        competitor: &Competitor,
        config: &mut RuntimeConfig,
        rng: &mut ThreadRng,
    ) -> Vec<v128> {
        generate_skewnorm_vec(
            AO5_SOLVE_COUNT,
            competitor.stats.as_ref(),
            rng,
            config,
            competitor.entered_results.as_slice(),
        )
    }

    fn calculate_result(&self, solves: &[v128]) -> [i32; 4] {
        calc_wca_average_5(solves[0], solves[1], solves[2], solves[3], solves[4])
    }
}

// Mean of 3 simulation
pub struct Mo3Simulation {
    pub(crate) event: Mo3Event,
}

impl EventSimulation for Mo3Simulation {
    fn generate_solves(
        &self,
        competitor: &Competitor,
        config: &mut RuntimeConfig,
        rng: &mut ThreadRng,
    ) -> Vec<v128> {
        let results = generate_skewnorm_vec(
            MO3_SOLVE_COUNT,
            competitor.stats.as_ref(),
            rng,
            config,
            competitor.entered_results.as_slice(),
        );

        if self.event == Mo3Event::F333 {
            results.into_iter().map(i32x4_truncate_down_100).collect()
        } else {
            results
        }
    }

    fn calculate_result(&self, solves: &[v128]) -> [i32; 4] {
        calc_wca_mean_3(solves[0], solves[1], solves[2])
    }
}

// Best of 3 simulation
pub struct Bo3Simulation;

impl EventSimulation for Bo3Simulation {
    fn generate_solves(
        &self,
        competitor: &Competitor,
        config: &mut RuntimeConfig,
        rng: &mut ThreadRng,
    ) -> Vec<v128> {
        generate_skewnorm_vec(
            BO3_SOLVE_COUNT,
            competitor.stats.as_ref(),
            rng,
            config,
            competitor.entered_results.as_slice(),
        )
    }

    fn calculate_result(&self, solves: &[v128]) -> [i32; 4] {
        calc_wca_best_3(solves[0], solves[1], solves[2])
    }
}
