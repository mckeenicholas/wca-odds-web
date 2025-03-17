use core::arch::wasm32::v128;
use rand::rngs::ThreadRng;
use rand::thread_rng;
use serde::Serialize;
use std::{collections::HashMap, i32};

use crate::calc::{
    calc_mean_variance_stdev, find_lowest_indices, fit_skewnorm, transpose_solves,
    trim_errant_results,
};
use crate::gen_n_skewnorm_simd;
use crate::simd::{calc_wca_average_5, calc_wca_best_3, calc_wca_mean_3, i32x4_to_slice};
use crate::{DatedCompetitionResult, EventType};

#[derive(Debug)]
pub struct CompetitorStats {
    pub location: f32,
    pub shape: f32,
    pub skew: f32,
    pub dnf_rate: f32,
}

#[derive(Serialize, Clone)]
pub struct SimulationResult {
    win_count: u32,
    pod_count: u32,
    total_rank: u32,
    average_result: u32,
    mean_no_dnf: u32,
    rank_dist: Vec<u32>,
    hist_values: HashMap<i32, i32>,
}

impl SimulationResult {
    fn new(num_competitors: usize) -> Self {
        Self {
            win_count: 0,
            pod_count: 0,
            total_rank: 0,
            average_result: 0,
            mean_no_dnf: 0,
            rank_dist: vec![0; num_competitors],
            hist_values: HashMap::new(),
        }
    }
}

fn collect_results(results: Vec<DatedCompetitionResult>) -> Vec<i32> {
    // Assuming at least 2 averages per competitor as competitors predicted to win will generally be on the faster end.
    let mut collected = Vec::with_capacity(results.len() * 10);
    for comp_data in results {
        collected.extend(comp_data.results);
    }
    collected
}

fn add_hist(
    results: &[v128],
    simulation_results: &mut HashMap<i32, i32>,
    hist_min: i32,
    hist_max: i32,
) {
    for result in results {
        let solves = i32x4_to_slice(*result);
        for solve in solves {
            if solve == i32::MAX {
                continue;
            }

            let bucket = (solve / 10).clamp(hist_min, hist_max - 1);
            *simulation_results.entry(bucket).or_insert(0) += 1;
        }
    }
}

pub fn run_simulations(
    num_simulations: u32,
    competitor_data: Vec<Vec<DatedCompetitionResult>>,
    event_type: EventType,
    include_dnf: bool,
) -> Vec<SimulationResult> {
    let num_competitors = competitor_data.len();

    let (hist_min, hist_max, competitor_stats_with_means) =
        prepare_competitor_stats(competitor_data);

    let mut simulation_results = vec![SimulationResult::new(num_competitors); num_competitors];

    // Set the mean_no_dnf for each competitor
    for (i, (_, mean)) in competitor_stats_with_means.iter().enumerate() {
        simulation_results[i].mean_no_dnf = *mean;
    }

    let competitor_stats: Vec<Option<CompetitorStats>> = competitor_stats_with_means
        .into_iter()
        .map(|(stats, _)| stats)
        .collect();

    let mut rng: ThreadRng = thread_rng();

    for _ in 0..(num_simulations / 4) {
        let sim_results = generate_simulation_results(
            &competitor_stats,
            event_type,
            include_dnf,
            &mut rng,
            &mut simulation_results,
            hist_min,
            hist_max,
        );

        let solves_by_sim = transpose_solves(sim_results);

        update_simulation_results(&mut simulation_results, &solves_by_sim, num_competitors);
    }

    simulation_results
}

fn prepare_competitor_stats(
    competitor_data: Vec<Vec<DatedCompetitionResult>>,
) -> (i32, i32, Vec<(Option<CompetitorStats>, u32)>) {
    // Modified return type
    let mut hist_max = 0;
    let mut hist_min = i32::MAX;

    let competitor_stats = competitor_data
        .into_iter()
        .map(|results| {
            let all_results = collect_results(results);

            let num_dnf = all_results.iter().filter(|&&x| x < 0).count();
            let dnf_rate: f32 = num_dnf as f32 / all_results.len() as f32;
            let result_no_dnf = all_results
                .into_iter()
                .filter(|&x| x > 0)
                .collect::<Vec<_>>();

            if result_no_dnf.len() == 0 {
                return (None, 0);
            }

            let (sample_mean, _sample_variance, sample_dev) =
                calc_mean_variance_stdev(&result_no_dnf.as_slice());

            hist_max = hist_max.max(((sample_mean + sample_dev * 4.0) / 10.0) as i32);
            hist_min = hist_min.min(((sample_mean - sample_dev * 4.0) / 10.0) as i32);

            let trimmed_results = trim_errant_results(result_no_dnf, sample_mean, sample_dev);

            let (skew, shape, location) = fit_skewnorm(&trimmed_results);

            (
                Some(CompetitorStats {
                    location,
                    shape,
                    skew,
                    dnf_rate,
                }),
                sample_mean as u32,
            )
        })
        .collect::<Vec<(Option<CompetitorStats>, u32)>>();

    hist_min = hist_min.max(0);

    (hist_min, hist_max, competitor_stats)
}

fn generate_simulation_results(
    competitor_stats: &[Option<CompetitorStats>],
    event_type: EventType,
    include_dnf: bool,
    rng: &mut ThreadRng,
    simulation_results: &mut [SimulationResult],
    hist_min: i32,
    hist_max: i32,
) -> Vec<[i32; 4]> {
    competitor_stats
        .iter()
        .enumerate()
        .map(|(i, opt_stats)| {
            let stats = if let Some(stat) = opt_stats {
                stat
            } else {
                return [i32::MAX; 4];
            };

            simulate_event(
                event_type,
                stats,
                include_dnf,
                rng,
                &mut simulation_results[i],
                hist_min,
                hist_max,
            )
        })
        .collect::<Vec<[i32; 4]>>()
}

fn simulate_event(
    event_type: EventType,
    stats: &CompetitorStats,
    include_dnf: bool,
    rng: &mut ThreadRng,
    simulation_results: &mut SimulationResult,
    hist_min: i32,
    hist_max: i32,
) -> [i32; 4] {
    match event_type {
        EventType::Ao5 => {
            let results: [v128; 5] = gen_n_skewnorm_simd!(5, stats, rng, include_dnf);
            let [a1, a2, a3, a4, a5] = results;
            add_hist(
                &results,
                &mut simulation_results.hist_values,
                hist_min,
                hist_max,
            );
            calc_wca_average_5(a1, a2, a3, a4, a5)
        }
        EventType::Mo3 => {
            let results: [v128; 3] = gen_n_skewnorm_simd!(3, stats, rng, include_dnf);
            let [a1, a2, a3] = results;
            add_hist(
                &results,
                &mut simulation_results.hist_values,
                hist_min,
                hist_max,
            );
            calc_wca_mean_3(a1, a2, a3)
        }
        EventType::Bo3 => {
            let results: [v128; 3] = gen_n_skewnorm_simd!(3, stats, rng, include_dnf);
            let [a1, a2, a3] = results;
            add_hist(
                &results,
                &mut simulation_results.hist_values,
                hist_min,
                hist_max,
            );
            calc_wca_best_3(a1, a2, a3)
        }
    }
}

fn update_simulation_results(
    simulation_results: &mut [SimulationResult],
    solves_by_sim: &[Vec<i32>],
    num_competitors: usize,
) {
    for i in 0..4 {
        let avg_by_competitor = &solves_by_sim[i];

        let indicies = find_lowest_indices(avg_by_competitor.as_slice());

        simulation_results[indicies[0]].win_count += 1;
        for &index in indicies.iter().take(3) {
            simulation_results[index].pod_count += 1;
        }

        for i in 0..num_competitors {
            simulation_results[i].total_rank += (indicies[i] as u32) + 1;
            simulation_results[indicies[i]].rank_dist[i] += 1;
        }
    }
}
