use rand::{rngs::ThreadRng, thread_rng};
use serde::Serialize;
use std::{collections::HashMap, i32};

use crate::calc::{
    calc_mean_variance_stdev, find_lowest_indices, fit_skewnorm, transpose_solves,
    trim_errant_results,
};
use crate::event_simulation::{Ao5Simulation, Bo3Simulation, EventSimulation, Mo3Simulation};
use crate::{Competitor, DatedCompetitionResult, EventType};

pub struct SimulationConfig {
    pub include_dnf: bool,
    pub hist_min: i32,
    pub hist_max: i32,
}

pub struct SimulationContext<'a> {
    pub config: &'a SimulationConfig,
    pub rng: &'a mut ThreadRng,
}

struct HistogramBounds {
    min: i32,
    max: i32,
}

#[derive(Debug)]
pub struct CompetitorStats {
    pub location: f32,
    pub shape: f32,
    pub skew: f32,
    pub dnf_rate: f32,
}

#[derive(Serialize, Clone)]
pub struct SimulationResult {
    pub win_count: u32,
    pub pod_count: u32,
    pub total_rank: u32,
    pub average_result: u32,
    pub mean_no_dnf: u32,
    pub rank_dist: Vec<u32>,
    pub hist_values_single: HashMap<i32, i32>,
    pub hist_values_average: HashMap<i32, i32>,
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
            hist_values_single: HashMap::new(),
            hist_values_average: HashMap::new(),
        }
    }
}

pub fn run_simulations(
    num_simulations: u32,
    competitors: &[Competitor],
    event_type: EventType,
    include_dnf: bool,
) -> Vec<SimulationResult> {
    // Prepare configs
    let stats_result = prepare_competitor_stats(competitors);
    let config = SimulationConfig {
        include_dnf,
        hist_min: stats_result.bounds.min,
        hist_max: stats_result.bounds.max,
    };

    // Setup results
    let mut simulation_results =
        init_simulation_results(competitors.len(), &stats_result.competitor_stats);

    // Select event simulation implementation
    let event_simulator = get_event_simulator(event_type);

    // Run simulations
    let mut rng = thread_rng();

    for _ in 0..(num_simulations / 4) {
        let mut context = SimulationContext {
            config: &config,
            rng: &mut rng,
        };

        let sim_results = run_simulation_batch(
            competitors,
            &stats_result.competitor_stats,
            event_simulator.as_ref(),
            &mut context,
            &mut simulation_results,
        );

        update_rankings(&mut simulation_results, sim_results, competitors.len());
    }

    simulation_results
}

fn get_event_simulator(event_type: EventType) -> Box<dyn EventSimulation> {
    match event_type {
        EventType::Ao5 => Box::new(Ao5Simulation),
        EventType::Mo3 => Box::new(Mo3Simulation),
        EventType::Bo3 => Box::new(Bo3Simulation),
    }
}

struct CompetitorStatsResult {
    bounds: HistogramBounds,
    competitor_stats: Vec<(Option<CompetitorStats>, u32)>,
}

fn prepare_competitor_stats(competitors: &[Competitor]) -> CompetitorStatsResult {
    let mut hist_max = 0;
    let mut hist_min = i32::MAX;

    let competitor_stats = competitors
        .iter()
        .map(|competitor| {
            let results = &competitor.results;

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
        .collect();

    hist_min = hist_min.max(0);

    CompetitorStatsResult {
        bounds: HistogramBounds {
            min: hist_min,
            max: hist_max,
        },
        competitor_stats,
    }
}

fn init_simulation_results(
    num_competitors: usize,
    competitor_stats: &[(Option<CompetitorStats>, u32)],
) -> Vec<SimulationResult> {
    let mut results = vec![SimulationResult::new(num_competitors); num_competitors];

    // Copy mean values
    for (i, (_, mean)) in competitor_stats.iter().enumerate() {
        results[i].mean_no_dnf = *mean;
    }

    results
}

fn run_simulation_batch(
    competitors: &[Competitor],
    competitor_stats: &[(Option<CompetitorStats>, u32)],
    event_simulator: &dyn EventSimulation,
    context: &mut SimulationContext,
    simulation_results: &mut [SimulationResult],
) -> Vec<[i32; 4]> {
    competitor_stats
        .iter()
        .enumerate()
        .map(|(i, opt_stats)| {
            if let Some(stats) = &opt_stats.0 {
                event_simulator.run_simulation(
                    &competitors[i].entered_results,
                    stats,
                    context,
                    &mut simulation_results[i],
                )
            } else {
                [i32::MAX; 4]
            }
        })
        .collect()
}

fn update_rankings(
    simulation_results: &mut [SimulationResult],
    solve_results: Vec<[i32; 4]>,
    num_competitors: usize,
) {
    let solves_by_sim = transpose_solves(solve_results);

    for i in 0..4 {
        let avg_by_competitor = &solves_by_sim[i];
        let indices = find_lowest_indices(avg_by_competitor.as_slice());

        // Update win count
        simulation_results[indices[0]].win_count += 1;

        // Update podium counts
        for &index in indices.iter().take(3) {
            simulation_results[index].pod_count += 1;
        }

        // Update rankings
        for i in 0..num_competitors {
            simulation_results[i].total_rank += (indices[i] as u32) + 1;
            simulation_results[indices[i]].rank_dist[i] += 1;
        }
    }
}

fn collect_results(results: &Vec<DatedCompetitionResult>) -> Vec<i32> {
    let mut collected = Vec::with_capacity(results.len() * 10);
    for comp_data in results {
        collected.extend(comp_data.results.clone());
    }
    collected
}
