use crate::calc::{
    calc_weighted_mean_variance_stdev, find_lowest_indices, fit_weighted_skewnorm,
    transpose_solves, trim_weighted_results,
};
use crate::event_simulation::{Ao5Simulation, Bo3Simulation, EventSimulation, Mo3Simulation};
use crate::{Competitor, DatedCompetitionResult, EventType};
use rand::{rngs::ThreadRng, thread_rng};
use serde::Serialize;
use std::{collections::HashMap, i32};

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

        update_rankings(&mut simulation_results, sim_results);
    }

    simulation_results
}

fn get_event_simulator(event_type: EventType) -> Box<dyn EventSimulation> {
    match event_type {
        EventType::Ao5(_) => Box::new(Ao5Simulation),
        EventType::Mo3(mo3_event) => Box::new(Mo3Simulation { event: mo3_event }),
        EventType::Bo3(_) => Box::new(Bo3Simulation),
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

            // Apply exponential decay weighting to results based on recency
            let weighted_results = apply_exponential_weights(results);

            if weighted_results.is_empty() {
                return (None, 0);
            }

            // Extract all results (including DNFs) for DNF rate calculation
            let all_weighted_results: Vec<(i32, f32)> = weighted_results.clone();

            // Calculate weighted DNF rate
            let (dnf_weighted_count, total_weight) = all_weighted_results.iter().fold(
                (0.0, 0.0),
                |(dnf_sum, weight_sum), &(val, weight)| {
                    if val < 0 {
                        (dnf_sum + weight, weight_sum + weight)
                    } else {
                        (dnf_sum, weight_sum + weight)
                    }
                },
            );

            let dnf_rate = if total_weight > 0.0 {
                dnf_weighted_count / total_weight
            } else {
                0.0
            };

            // Filter out DNFs for time calculations
            let non_dnf_weighted_results: Vec<(i32, f32)> = weighted_results
                .into_iter()
                .filter(|&(val, _)| val > 0)
                .collect();

            if non_dnf_weighted_results.is_empty() {
                return (None, 0);
            }

            // Calculate weighted statistics
            let (sample_mean, _sample_variance, sample_dev) =
                calc_weighted_mean_variance_stdev(&non_dnf_weighted_results);

            hist_max = hist_max.max(((sample_mean + sample_dev * 4.0) / 10.0) as i32);
            hist_min = hist_min.min(((sample_mean - sample_dev * 4.0) / 10.0) as i32);

            // Trim outliers
            let trimmed_weighted_results =
                trim_weighted_results(non_dnf_weighted_results, sample_mean, sample_dev);

            // Fit distribution
            let (skew, shape, location) = fit_weighted_skewnorm(&trimmed_weighted_results);

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

// New function to apply exponential decay weights based on recency
fn apply_exponential_weights(results: &Vec<DatedCompetitionResult>) -> Vec<(i32, f32)> {
    const HALF_LIFE_DAYS: f32 = 180.0; // 6 months
    const DECAY_CONSTANT: f32 = 0.69314718056 / HALF_LIFE_DAYS; // ln(2) / half_life

    let mut weighted_results = Vec::new();

    for result_set in results {
        let weight = (-DECAY_CONSTANT * result_set.days_since as f32).exp();

        for &time in &result_set.results {
            weighted_results.push((time, weight));
        }
    }

    weighted_results
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

fn update_rankings(simulation_results: &mut [SimulationResult], solve_results: Vec<[i32; 4]>) {
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
        for (position, &competitor_index) in indices.iter().enumerate() {
            simulation_results[competitor_index].rank_dist[position] += 1;
            simulation_results[competitor_index].total_rank += (position as u32) + 1;
        }
    }
}
