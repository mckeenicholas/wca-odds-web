use core::arch::wasm32::{
    f32x4, f32x4_abs, f32x4_add, f32x4_convert_i32x4, f32x4_div, f32x4_extract_lane, f32x4_max,
    f32x4_mul, f32x4_splat, i32x4_extract_lane, v128,
};
use rand::rngs::ThreadRng;
use rand::Rng;
use rand::{distributions::Uniform, thread_rng};
use serde::Serialize;
use std::f32::consts::PI;

use crate::{console_log, DatedCompetitionResult};

struct CompetitorStats {
    location: f32,
    shape: f32,
    skew: f32,
    dnf_rate: f32,
}

#[derive(Serialize)]
pub struct SimulationResults {
    win_count: Vec<u32>,
    pod_count: Vec<u32>,
    total_rank: Vec<u32>,
    rank_dist: Vec<Vec<u32>>,
}

macro_rules! i32x4_extract_lanes {
    ($idx:literal $(, $vec:expr)+) => {
        [
            $(i32x4_extract_lane::<$idx>($vec), )+
        ]
    };
}

// Apply a non-simd function to each lane of a f32x4 vector.
fn f32x4_apply<F: FnMut(f32) -> f32>(vec: v128, mut f: F) -> v128 {
    let v0 = f(f32x4_extract_lane::<0>(vec));
    let v1 = f(f32x4_extract_lane::<1>(vec));
    let v2 = f(f32x4_extract_lane::<2>(vec));
    let v3 = f(f32x4_extract_lane::<3>(vec));
    f32x4(v0, v1, v2, v3)
}

// fn f32x4_cast_i16(v1: v128, v2: v128) -> v128

fn f32x4_to_slice(vec: v128) -> [f32; 4] {
    [
        f32x4_extract_lane::<0>(vec),
        f32x4_extract_lane::<1>(vec),
        f32x4_extract_lane::<2>(vec),
        f32x4_extract_lane::<3>(vec),
    ]
}

fn f32x4_cast_centiseconds(vec: v128) -> v128 {
    let centiseconds = f32x4_mul(vec, f32x4_splat(100.0));
    f32x4_convert_i32x4(centiseconds)
}

fn mean(data: &[i32]) -> f32 {
    data.iter().sum::<i32>() as f32 / data.len() as f32
}

fn variance(data: &[i32], mean: f32) -> f32 {
    data.iter().map(|&x| (x as f32 - mean).powi(2)).sum::<f32>() / data.len() as f32
}

fn stdev(data: &[i32], mean: f32) -> f32 {
    variance(data, mean).sqrt()
}

fn collect_results(results: Vec<DatedCompetitionResult>) -> Vec<i32> {
    results
        .into_iter()
        .flat_map(|comp_data| comp_data.results)
        .collect::<Vec<i32>>()
}

fn trim_errant_results(results: Vec<i32>, mean: f32, stdev: f32) -> Vec<i32> {
    results
        .into_iter()
        .filter(|&x| x <= (mean + stdev * 3.0) as i32)
        .collect::<Vec<i32>>()
}

fn gen_skewnorm_simd(stats: &CompetitorStats, rand_source: &mut ThreadRng) -> v128 {
    let range = Uniform::<f32>::new(0.0, 1.0);

    let v1 = rand_source.sample(range);
    let v2 = rand_source.sample(range);
    let v3 = rand_source.sample(range);
    let v4 = rand_source.sample(range);

    let sigma = stats.skew / (1.0 + stats.skew).sqrt();
    let u0 = f32x4(v1, v2, v3, v4);
    let v = f32x4(v2, v3, v4, v1);

    let u1 = f32x4_mul(
        f32x4_add(
            f32x4_mul(f32x4_splat(sigma), u0),
            f32x4_mul(f32x4_splat((1.0 - sigma.powi(2)).sqrt()), v),
        ),
        f32x4_splat(stats.shape),
    );

    let u2 = f32x4_abs(u1);

    let u3 = f32x4_add(u2, f32x4_splat(stats.location));

    f32x4_cast_centiseconds(u3)
}

fn calc_wca_best_3(v1: v128, v2: v128, v3: v128) -> [f32; 4] {
    let max_1_2 = f32x4_max(v1, v2);
    let max_v128 = f32x4_max(max_1_2, v3);

    f32x4_to_slice(max_v128)
}

fn calc_wca_mean_3(v1: v128, v2: v128, v3: v128) -> [f32; 4] {
    let sum_1_2 = f32x4_add(v1, v2);
    let sum_v128 = f32x4_add(sum_1_2, v3);

    let mean_v128 = f32x4_div(sum_v128, f32x4_splat(3.0));

    f32x4_to_slice(mean_v128)
}

fn calc_a5_single(times: &mut [i32; 5]) -> i32 {
    times.sort_unstable();
    (times[1] + times[2] + times[3]) / 3
}

fn calc_wca_average_5(v1: v128, v2: v128, v3: v128, v4: v128, v5: v128) -> [i32; 4] {
    let mut session_1 = i32x4_extract_lanes!(0, v1, v2, v3, v4, v5);
    let avg_1 = calc_a5_single(&mut session_1);

    let mut session_2 = i32x4_extract_lanes!(1, v1, v2, v3, v4, v5);
    let avg_2 = calc_a5_single(&mut session_2);

    let mut session_3 = i32x4_extract_lanes!(2, v1, v2, v3, v4, v5);
    let avg_3 = calc_a5_single(&mut session_3);

    let mut session_4 = i32x4_extract_lanes!(3, v1, v2, v3, v4, v5);
    let avg_4 = calc_a5_single(&mut session_4);

    [avg_1, avg_2, avg_3, avg_4]
}

fn fit_skewnorm(times: &[i32]) -> (f32, f32, f32) {
    let mean = mean(times);
    let variance = variance(times, mean);
    let skewness = times
        .iter()
        .map(|&x| ((x as f32 - mean) / variance.sqrt()).powi(3))
        .sum::<f32>()
        / times.len() as f32;

    let max_skew = 0.995 * ((4.0 - PI).sqrt() * (2.0 / PI).sqrt() * (1.0 - 2.0 / PI).powf(-1.5));

    let bounded_skew = skewness.clamp(-max_skew, max_skew);

    let delta = bounded_skew.signum()
        * ((PI / 2.0) * skewness.abs().powf(2.0 / 3.0) / ((4.0 - PI) / 2.0).powf(2.0 / 3.0))
            .sqrt()
            .clamp(-0.0999, 0.0999);

    let alpha = (1.0 - delta.powi(2)).sqrt();
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
        Vec::with_capacity(comp_length),
        Vec::with_capacity(comp_length),
        Vec::with_capacity(comp_length),
        Vec::with_capacity(comp_length),
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
) -> SimulationResults {
    let num_competitors = competitor_data.len();

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

            let sample_mean = mean(result_no_dnf.as_slice());
            let sample_dev = stdev(result_no_dnf.as_slice(), sample_mean);

            let trimmed_results = trim_errant_results(result_no_dnf, sample_mean, sample_dev);

            let (skew, shape, location) = fit_skewnorm(&trimmed_results);

            CompetitorStats {
                location,
                shape,
                skew,
                dnf_rate,
            }
        })
        .collect::<Vec<CompetitorStats>>();

    let mut win_count: Vec<u32> = vec![0; num_competitors];
    let mut pod_count: Vec<u32> = vec![0; num_competitors];
    let mut total_rank: Vec<u32> = vec![0; num_competitors];
    let mut rank_dist: Vec<Vec<u32>> = vec![vec![0; num_competitors]; num_competitors];

    let mut rng = thread_rng();

    for _ in 0..(num_simulations / 4) {
        let sim_results = competitor_stats
            .iter()
            .map(|stats| {
                // TODO: this is really stupid and should be replaced by some macro expansion
                let a1 = gen_skewnorm_simd(stats, &mut rng);
                let a2 = gen_skewnorm_simd(stats, &mut rng);
                let a3 = gen_skewnorm_simd(stats, &mut rng);
                let a4 = gen_skewnorm_simd(stats, &mut rng);
                let a5 = gen_skewnorm_simd(stats, &mut rng);

                let out = calc_wca_average_5(a1, a2, a3, a4, a5);
                console_log!("{:?}", out);

                out
            })
            .collect::<Vec<[i32; 4]>>();

        let solves_by_sim = transpose_solves(sim_results);

        for i in 0..4 {
            let indicies = find_lowest_indices(solves_by_sim[i].as_slice());

            win_count[indicies[0]] += 1;
            for &index in indicies.iter().take(3) {
                pod_count[index] += 1;
            }

            for i in 0..num_competitors {
                total_rank[i] += (indicies[i] as u32) + 1;
                rank_dist[indicies[i]][i] += 1;
            }
        }
    }

    SimulationResults {
        win_count,
        pod_count,
        total_rank,
        rank_dist,
    }
}

// TODO: calculate distribution paramters here rather than in JS

// fn calc_exgauss_coefficients(results: Vec<i32>, mean: f32, stdev: f32) -> (f32, f32, f32, f32) {
//     let results_len = results.len() as f32;

//     let gamma = (results
//         .into_iter()
//         .map(|x| ((x as f32 - mean) / stdev).powi(3))
//         .sum::<f32>()
//         / results_len)
//         .clamp(0.0, 0.99);

//     let mu = mean - stdev * (gamma / 2.0).powf(1.0 / 3.0);
//     let sigma = (stdev.powi(2) * (1.0 - (gamma / 2.0).powf(2.0 / 3.0))).sqrt();
//     let tau = stdev * (gamma / 2.0).powf(1.0 / 3.0);

//     (gamma, mu, sigma, tau)
// }

// fn gen_exgauss_simd(
//     mu: f32,
//     sample_dev: f32,
//     tau: f32,
//     dnf_rate: f32,
//     rng: &mut ThreadRng,
// ) -> v128 {
//     let range = Uniform::<f32>::new(1e-10, 1.0);
//     let (v1, v2, v3, v4) = (
//         rng.sample(&range),
//         rng.sample(&range),
//         rng.sample(&range),
//         rng.sample(&range),
//     );

//     let u1_log = f32x4(v1.ln(), v2.ln(), v3.ln(), v4.ln());
//     let u2 = f32x4(v2, v3, v4, v1);
//     let u3 = f32x4(v3, v4, v1, v2);
//     let u4 = f32x4(v4, v1, v2, v3);

//     let mu_vec = f32x4_splat(mu);
//     let stdev_vec = f32x4_splat(sample_dev);

//     let dnf_rate_vec = f32x4_splat(dnf_rate);
//     let mask = f32x4_gt(u4, dnf_rate_vec);

//     let r = f32x4_sqrt(f32x4_mul(f32x4_splat(-2.0), u1_log));
//     let o = f32x4_mul(f32x4_splat(2.0 * PI as f32), u2);

//     let intr = f32x4_sub(f32x4_splat(1.0), u3);
//     let intr_log = f32x4_apply(intr, f32::ln);

//     let exp = f32x4_mul(f32x4_splat(-tau), intr_log);
//     let exp_mu = f32x4_add(exp, mu_vec);

//     let o_cos = f32x4_apply(o, f32::cos);
//     let r_cos = f32x4_mul(r, o_cos);

//     let result = f32x4_mul(r_cos, f32x4_mul(stdev_vec, mu_vec));

//     result
// }

// fn random_exgauss_simd(mu: f64, sample_dev: f64, tau: f64, dnf_rate: f64) -> f64x64 {
// let mut rng = thread_rng();
//     let range = Uniform::new(1e-10, 1.0);

//     // Generate random values for 64 lanes
//     let u1 = f64x64::from_array([(); 64].map(|_| rng.sample(&range)));
//     let u2 = f64x64::from_array([(); 64].map(|_| rng.sample(&range)));
//     let u3 = f64x64::from_array([(); 64].map(|_| rng.sample(&range)));
//     let u4 = f64x64::from_array([(); 64].map(|_| rng.sample(&range)));

//     // SIMD conditional: replace lanes where u4 < dnf_rate with f64::MAX
//     let mut result = f64x64::splat(f64::MAX);
//     let mask = u4.simd_lt(&f64x64::splat(dnf_rate));

//     if !mask.any() {
//         // Perform the Ex-Gaussian calculation
//         let r = (f64x64::splat(-2.0) * u1.ln()).sqrt();
//         let o = f64x64::splat(2.0 * PI) * u2;
//         let exp = -tau * (f64x64::splat(1.0) - u3).ln();

//         result = (r * o.cos()) * f64x64::splat(sample_dev + exp + mu);
//     }

//     result
// }

// fn calc_wca_average_5(results: &mut [i32x64; 5]) -> i32x64 {
//     // Create a temporary array to hold transposed values for each lane
//     let mut lane_values = [[0i32; 5]; 64];

//     for lane in 0..64 {
//         for i in 0..5 {
//             lane_values[lane][i] = results[i][lane];
//         }
//         lane_values[lane].sort_unstable();
//     }

//     for i in 0..5 {
//         results[i] = i32x64::from_array(
//             lane_values.map(|lane| lane[i]).into()
//         );
//     }

//     let sum = results[1] + results[2] + results[3];
//     sum / i32x64::splat(3);
// }

// fn calc_wca_mean_3(results: &mut [i32x64; 3]) -> i32x64 {
//     let mut lane_values = [[0i32; 5]; 64];

//     for lane in 0..64 {
//         for i in 0..5 {
//             lane_values[lane][i] = results[i][lane];
//         }
//         lane_values[lane].sort_unstable();
//     }

//     for i in 0..5 {
//         results[i] = i32x64::from_array(
//             lane_values.map(|lane| lane[i]).into()
//         );
//     }

//     let sum = results[0] + results[1] + results[2];
//     sum / i32x64::splat(3);
// }

// fn calc_wca_best_3(results: &[i32x64; 3]) -> i32x64 {
//     let max_tmp = results[1].max(results[2]);
//     let max = max_tmp.max(results[3]);
// }
