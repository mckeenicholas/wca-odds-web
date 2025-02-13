use core::arch::wasm32::{
    f32x4, f32x4_add, f32x4_div, f32x4_gt, f32x4_max, f32x4_min, f32x4_mul, f32x4_neg, f32x4_splat,
    f32x4_sub, i32x4_extract_lane, i32x4_splat, i32x4_trunc_sat_f32x4, v128, v128_bitselect,
};
use rand::{rngs::ThreadRng, thread_rng};
use rand_distr::{Distribution, Normal};
use serde::Serialize;
use std::{collections::HashMap, f32::consts::PI, i32};

use crate::DatedCompetitionResult;

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
    rank_dist: Vec<u32>,
    hist_values: HashMap<i32, i32>,
}

impl SimulationResult {
    fn new(num_competitors: usize) -> Self {
        Self {
            win_count: 0,
            pod_count: 0,
            total_rank: 0,
            rank_dist: vec![0; num_competitors],
            hist_values: HashMap::new(),
        }
    }
}

macro_rules! gen_n_skewnorm_simd {
    ($n:literal, $stats:expr, $rng:expr) => {{
        let mut values = [i32x4_splat(0); $n];
        for i in 0..$n {
            values[i] = gen_skewnorm_simd($stats, $rng);
        }
        values
    }};
}

macro_rules! gen_n_random {
    ($n:literal, $source:expr, $rng:expr) => {{
        let mut values = [0.0; $n];
        for i in 0..$n {
            values[i] = $rng.sample($source);
        }
        values
    }};
}

fn i32x4_to_slice(vec: v128) -> [i32; 4] {
    [
        i32x4_extract_lane::<0>(vec),
        i32x4_extract_lane::<1>(vec),
        i32x4_extract_lane::<2>(vec),
        i32x4_extract_lane::<3>(vec),
    ]
}

fn f32x4_conditional_negate(input: v128, cond: v128) -> v128 {
    let mask = f32x4_gt(cond, f32x4_splat(0.0));

    let neg_u1 = f32x4_neg(input);

    let result = v128_bitselect(input, neg_u1, mask);

    result
}

fn calc_mean_variance_stdev(data: &[i32]) -> (f32, f32, f32) {
    let (sum, sum_sq) = data.iter().fold((0f64, 0f64), |(s, s_sq), &x| {
        let xf = x as f64;
        (s + xf, s_sq + xf * xf)
    });

    let n = data.len() as f64;
    let mean = sum / n;
    let variance = (sum_sq / n) - (mean * mean);
    let stdev = variance.sqrt();

    (mean as f32, variance as f32, stdev as f32)
}

fn collect_results(results: Vec<DatedCompetitionResult>) -> Vec<i32> {
    // Assuming at least 2 averages per competitor as competitors predicted to win will generally be on the faster end.
    let mut collected = Vec::with_capacity(results.len() * 10);
    for comp_data in results {
        collected.extend(comp_data.results);
    }
    collected
}

fn trim_errant_results(mut results: Vec<i32>, mean: f32, stdev: f32) -> Vec<i32> {
    let threshold = (mean + stdev * 3.0) as i32;
    results.retain(|&x| x <= threshold);
    results
}

fn gen_skewnorm_simd(stats: &CompetitorStats, rand_source: &mut ThreadRng) -> v128 {
    let range = Normal::new(0.0, 1.0).expect("Failed to initialize normal distribution");

    let [v1, v2, v3, v4] = gen_n_random!(4, rand_source, range);
    let [w1, w2, w3, w4] = gen_n_random!(4, rand_source, range);

    let sigma = stats.skew / (1.0 + stats.skew.powi(2)).sqrt();

    let u0 = f32x4(v1, v2, v3, v4);
    let v = f32x4(w1, w2, w3, w4);

    let u1 = f32x4_mul(
        f32x4_add(
            f32x4_mul(f32x4_splat(sigma), u0),
            f32x4_mul(f32x4_splat((1.0 - sigma.powi(2)).sqrt()), v),
        ),
        f32x4_splat(stats.shape),
    );

    let u2 = f32x4_conditional_negate(u1, u0);
    let u3 = f32x4_add(u2, f32x4_splat(stats.location));

    i32x4_trunc_sat_f32x4(u3)
}

fn calc_avg_5_lane(times: &mut [i32; 5]) -> i32 {
    times.sort_unstable();
    ((times[1] as i64 + times[2] as i64 + times[3] as i64) / 3 as i64) as i32
}

fn calc_wca_best_3(v1: v128, v2: v128, v3: v128) -> [i32; 4] {
    let max_1_2 = f32x4_max(v1, v2);
    let max_v128 = f32x4_max(max_1_2, v3);

    i32x4_to_slice(max_v128)
}

fn calc_wca_mean_3(v1: v128, v2: v128, v3: v128) -> [i32; 4] {
    let sum_1_2 = f32x4_add(v1, v2);
    let sum_v128 = f32x4_add(sum_1_2, v3);
    let mean_v128 = f32x4_div(sum_v128, f32x4_splat(3.0));

    i32x4_to_slice(mean_v128)
}

fn calc_wca_average_5(v1: v128, v2: v128, v3: v128, v4: v128, v5: v128) -> [i32; 4] {
    let max_1_2 = f32x4_max(v1, v2);
    let max_3_4 = f32x4_max(v3, v4);
    let max_all = f32x4_max(max_1_2, f32x4_max(max_3_4, v5));

    let min_1_2 = f32x4_min(v1, v2);
    let min_3_4 = f32x4_min(v3, v4);
    let min_all = f32x4_min(min_1_2, f32x4_min(min_3_4, v5));

    let sum = f32x4_add(f32x4_add(f32x4_add(f32x4_add(v1, v2), v3), v4), v5);
    let adjusted_sum = f32x4_sub(f32x4_sub(sum, max_all), min_all);

    i32x4_to_slice(f32x4_div(adjusted_sum, f32x4_splat(3.0)))
}

fn fit_skewnorm(times: &[i32]) -> (f32, f32, f32) {
    let (mean, variance, stdev) = calc_mean_variance_stdev(times);

    let skewness = times
        .iter()
        .map(|&x| ((x as f32 - mean) / stdev).powi(3))
        .sum::<f32>()
        / times.len() as f32;

    let max_skew = 0.995 * ((4.0 - PI).sqrt() * (2.0 / PI).sqrt() * (1.0 - 2.0 / PI).powf(-1.5));

    let bounded_skew = skewness.clamp(-max_skew, max_skew);

    let delta = bounded_skew.signum()
        * ((PI / 2.0) * bounded_skew.abs().powf(2.0 / 3.0)
            / (bounded_skew.abs().powf(2.0 / 3.0) + ((4.0 - PI) / 2.0).powf(2.0 / 3.0)))
        .sqrt()
        .clamp(-0.9999, 0.9999);

    let alpha = delta / (1.0 - delta.powi(2)).sqrt();
    let omega = (variance / (1.0 - 2.0 * delta.powi(2) / PI)).sqrt();
    let xi = mean - omega * delta * (2.0 / PI).sqrt();

    (alpha, omega, xi)
}

fn find_lowest_indices(vec: &[i32]) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..vec.len()).collect();
    indices.sort_unstable_by_key(|&i| vec[i]);
    indices
}

fn transpose_solves(solves: Vec<[i32; 4]>) -> [Vec<i32>; 4] {
    let comp_length = solves.len();

    let mut out = [
        vec![0; comp_length],
        vec![0; comp_length],
        vec![0; comp_length],
        vec![0; comp_length],
    ];

    for i in 0..comp_length {
        for j in 0..4 {
            out[j][i] = solves[i][j];
        }
    }

    out
}

pub fn run_simulations(
    num_simulations: u32,
    competitor_data: Vec<Vec<DatedCompetitionResult>>,
) -> Vec<SimulationResult> {
    let num_competitors = competitor_data.len();

    let mut hist_max = 0;
    let mut hist_min = i32::MAX;

    let competitor_stats = competitor_data
        .into_iter()
        .map(|results| {
            // TODO: use a date-based weighted average for this instead
            let all_results = collect_results(results);

            let num_dnf = all_results.iter().filter(|&&x| x < 0).count();
            let dnf_rate: f32 = num_dnf as f32 / all_results.len() as f32;
            let result_no_dnf = all_results
                .into_iter()
                .filter(|&x| x > 0)
                .collect::<Vec<_>>();

            if result_no_dnf.len() == 0 {
                return None;
            }

            let (sample_mean, _sample_variance, sample_dev) =
                calc_mean_variance_stdev(&result_no_dnf.as_slice());

            hist_max = hist_max.max(((sample_mean + sample_dev * 4.0) / 10.0) as i32);
            hist_min = hist_min.min(((sample_mean - sample_dev * 4.0) / 10.0) as i32);

            let trimmed_results = trim_errant_results(result_no_dnf, sample_mean, sample_dev);

            let (skew, shape, location) = fit_skewnorm(&trimmed_results);

            Some(CompetitorStats {
                location,
                shape,
                skew,
                dnf_rate,
            })
        })
        .collect::<Vec<Option<CompetitorStats>>>();

    hist_min = hist_min.max(0);

    let mut simulation_results = vec![SimulationResult::new(num_competitors); num_competitors];
    let mut rng = thread_rng();

    for _ in 0..(num_simulations / 4) {
        let sim_results = competitor_stats
            .iter()
            .enumerate()
            .map(|(i, opt_stats)| {
                let stats = if let Some(stat) = opt_stats {
                    stat
                } else {
                    // TODO: Refactor this into ao Option<[i32]> instead... Rust has an incredibly powerful type system, and we should use it.
                    return [i32::MAX; 4];
                };

                let results: [v128; 5] = gen_n_skewnorm_simd!(5, stats, &mut rng);
                let [a1, a2, a3, a4, a5] = results;
                let out = calc_wca_average_5(a1, a2, a3, a4, a5);

                for result in results {
                    let solves = i32x4_to_slice(result);
                    for solve in solves {
                        if solve == i32::MAX {
                            continue;
                        }

                        let bucket = (solve / 10).clamp(hist_min, hist_max - 1);
                        *simulation_results[i].hist_values.entry(bucket).or_insert(0) += 1;
                    }
                }

                out
            })
            .collect::<Vec<[i32; 4]>>();

        let solves_by_sim = transpose_solves(sim_results);

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

    simulation_results
}
