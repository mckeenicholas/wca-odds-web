use core::arch::wasm32::*;
use rand::rngs::ThreadRng;
use rand::Rng;
use rand::{distributions::Uniform, thread_rng};
use std::f32::consts::PI;

use crate::DatedCompetitionResult;

struct CompetitorStats {
    sample_mean: f32,
    sample_dev: f32,
    gamma: f32,
    mu: f32,
    sigma: f32,
    tau: f32,
    dnf_rate: f32,
}

macro_rules! extract_lanes {
    ($idx:literal $(, $vec:expr)+) => {
        &[
            $(f32x4_extract_lane::<$idx>($vec), )+
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

fn mean(data: &[i32]) -> f32 {
    data.iter().sum::<i32>() as f32 / data.len() as f32
}

fn stdev(data: &[i32], mean: f32) -> f32 {
    (data.iter().map(|&x| (x as f32 - mean).powi(2)).sum::<f32>() / data.len() as f32).sqrt()
}

fn gen_exgauss_simd(
    mu: f32,
    sample_dev: f32,
    tau: f32,
    dnf_rate: f32,
    rng: &mut ThreadRng,
) -> v128 {
    let range = Uniform::<f32>::new(1e-10, 1.0);
    let (v1, v2, v3, v4) = (
        rng.sample(&range),
        rng.sample(&range),
        rng.sample(&range),
        rng.sample(&range),
    );

    let u1_log = f32x4(v1.ln(), v2.ln(), v3.ln(), v4.ln());
    let u2 = f32x4(v2, v3, v4, v1);
    let u3 = f32x4(v3, v4, v1, v2);
    let u4 = f32x4(v4, v1, v2, v3);

    let mu_vec = f32x4_splat(mu);
    let stdev_vec = f32x4_splat(sample_dev);

    let dnf_rate_vec = f32x4_splat(dnf_rate);
    let mask = f32x4_gt(u4, dnf_rate_vec);

    let r = f32x4_sqrt(f32x4_mul(f32x4_splat(-2.0), u1_log));
    let o = f32x4_mul(f32x4_splat(2.0 * PI as f32), u2);

    let intr = f32x4_sub(f32x4_splat(1.0), u3);
    let intr_log = f32x4_apply(intr, f32::ln);

    let exp = f32x4_mul(f32x4_splat(-tau), intr_log);
    let exp_mu = f32x4_add(exp, mu_vec);

    let o_cos = f32x4_apply(o, f32::cos);
    let r_cos = f32x4_mul(r, o_cos);

    let result = f32x4_mul(r_cos, f32x4_mul(stdev_vec, mu_vec));

    result
}

fn run_simulations(num_simulations: u32, competitor_data: Vec<Vec<DatedCompetitionResult>>) {
    let num_competitors = competitor_data.len();

    let stats = competitor_data
        .into_iter()
        .map(|results| {
            // TODO: use a date-based weighted average for this instead
            let all_results = results
                .into_iter()
                .flat_map(|comp_data| comp_data.results)
                .collect::<Vec<i32>>();

            let result_no_dnf = all_results.iter().filter(|&&x| x > 0);
            let num_dnf = all_results.iter().filter(|&&x| x < 0).count();
            let dnf_rate: f32 = num_dnf as f32 / all_results.len() as f32;

            let sample_mean = mean(&all_results);
            let sample_dev = stdev(&all_results, sample_mean);

            let trimmed_results = all_results
                .iter()
                .filter(|&&x| x <= (sample_mean + sample_dev * 3.0) as i32)
                .cloned()
                .collect::<Vec<i32>>();

            let trimmed_mean = mean(&trimmed_results);
            let trimmed_dev = stdev(&trimmed_results, trimmed_mean);

            let gamma: f32 = trimmed_results
                .iter()
                .map(|&x| ((x as f32 - trimmed_mean) / trimmed_dev).powi(3))
                .sum::<f32>()
                / trimmed_results.len() as f32;

            let gamma_trimmed = gamma.min(0.99).max(0.0);

            let mu = sample_mean - trimmed_dev * (gamma_trimmed / 2.0).powf(1.0 / 3.0);
            let sigma =
                (trimmed_dev.powi(2) * (1.0 - (gamma_trimmed / 2.0).powf(2.0 / 3.0))).sqrt();
            let tau = trimmed_dev * (gamma_trimmed / 2.0).powf(1.0 / 3.0);

            CompetitorStats {
                sample_mean,
                sample_dev,
                gamma,
                mu,
                sigma,
                tau,
                dnf_rate,
            }
        })
        .collect::<Vec<CompetitorStats>>();

    let mut win_count = vec![0; num_competitors];
    let mut pod_count = vec![0; num_competitors];
    let mut total_rank = vec![0; num_competitors];
    let mut rank_dist = vec![vec![0; num_competitors]; num_competitors];

    let mut rng = thread_rng();

    for i in 0..(num_simulations / 4) {
        for j in 0..num_competitors {
            let CompetitorStats {
                mu,
                sample_dev,
                tau,
                dnf_rate,
                ..
            } = stats[j];

            let a = gen_exgauss_simd(mu, sample_dev, tau, dnf_rate, &mut rng);
            let b = gen_exgauss_simd(mu, sample_dev, tau, dnf_rate, &mut rng);
            let c = gen_exgauss_simd(mu, sample_dev, tau, dnf_rate, &mut rng);
            let d = gen_exgauss_simd(mu, sample_dev, tau, dnf_rate, &mut rng);
            let e = gen_exgauss_simd(mu, sample_dev, tau, dnf_rate, &mut rng);

            let p1: &[f32; 5] = extract_lanes!(0, a, b, c, d, e);
        }
    }
}

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
