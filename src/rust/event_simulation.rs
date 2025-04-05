use crate::event::Mo3Event;
use crate::simd::{
    calc_wca_average_5, calc_wca_best_3, calc_wca_mean_3, generate_skewnorm_vec, i32x4_to_slice,
    i32x4_truncate_down_100, DNF_TEMP_VALUE,
};
use crate::simulation::{CompetitorStats, SimulationContext, SimulationResult};
use core::arch::wasm32::v128;
use std::collections::HashMap;
use std::iter;

pub trait EventSimulation {
    fn generate_solves(
        &self,
        competitor_data: &[i32],
        stats: &CompetitorStats,
        context: &mut SimulationContext,
    ) -> Vec<v128>;

    fn calculate_result(&self, solves: &[v128]) -> [i32; 4];

    fn run_simulation(
        &self,
        competitor_data: &[i32],
        stats: &CompetitorStats,
        context: &mut SimulationContext,
        result: &mut SimulationResult,
    ) -> [i32; 4] {
        let solves: Vec<v128> = self.generate_solves(competitor_data, stats, context);

        for (&solve, &entered) in iter::zip(&solves, competitor_data) {
            let solve_values = i32x4_to_slice(solve);

            // For now, don't include entered times in single histogram
            if entered != 0 {
                continue;
            }

            self.add_to_histogram(
                &solve_values,
                &mut result.hist_values_single,
                context.config.hist_min,
                context.config.hist_max,
            );
        }

        let final_result = self.calculate_result(&solves.as_slice());

        self.add_to_histogram(
            &final_result,
            &mut result.hist_values_average,
            context.config.hist_min,
            context.config.hist_max,
        );

        final_result
    }

    fn add_to_histogram(
        &self,
        values: &[i32],
        histogram: &mut HashMap<i32, i32>,
        hist_min: i32,
        hist_max: i32,
    ) {
        for &value in values {
            if value == i32::MAX || value == DNF_TEMP_VALUE {
                continue;
            }

            let bucket = (value / 10).clamp(hist_min, hist_max - 1);
            *histogram.entry(bucket).or_insert(0) += 1;
        }
    }
}

// Average of 5 simulation
pub struct Ao5Simulation;

impl EventSimulation for Ao5Simulation {
    fn generate_solves(
        &self,
        competitor_data: &[i32],
        stats: &CompetitorStats,
        context: &mut SimulationContext,
    ) -> Vec<v128> {
        generate_skewnorm_vec(
            5,
            stats,
            context.rng,
            context.config.include_dnf,
            competitor_data,
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
        competitor_data: &[i32],
        stats: &CompetitorStats,
        context: &mut SimulationContext,
    ) -> Vec<v128> {
        let results = generate_skewnorm_vec(
            3,
            stats,
            context.rng,
            context.config.include_dnf,
            competitor_data,
        );

        if self.event == Mo3Event::F333 {
            results
                .into_iter()
                .map(|solve_set| i32x4_truncate_down_100(solve_set))
                .collect()
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
        competitor_data: &[i32],
        stats: &CompetitorStats,
        context: &mut SimulationContext,
    ) -> Vec<v128> {
        generate_skewnorm_vec(
            3,
            stats,
            context.rng,
            context.config.include_dnf,
            competitor_data,
        )
    }

    fn calculate_result(&self, solves: &[v128]) -> [i32; 4] {
        calc_wca_best_3(solves[0], solves[1], solves[2])
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::simulation::{SimulationConfig, SimulationResult};
//     use rand::thread_rng;

//     // TODO: Add tests
// }
