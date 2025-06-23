use core::arch::wasm32::{
    f32x4, f32x4_add, f32x4_convert_i32x4, f32x4_div, f32x4_gt, f32x4_mul, f32x4_neg, f32x4_splat,
    f32x4_sub, i32x4_extract_lane, i32x4_mul, i32x4_splat, i32x4_trunc_sat_f32x4, v128,
    v128_bitselect,
};
use rand::Rng;
use rand_distr::{Distribution, Normal, Uniform};

use crate::{competitor::CompetitorStats, simulation::RuntimeConfig};

// Represents one centisecond (1/100th of a second) worse than the highest allowable result.
// We use this rather than i32::MAX because SIMD instructions have slightly different definitions
// for inverting max values compared to Rust's default behavior.
pub const DNF_VALUE: i32 = 60 * 60 * 100 + 1;

macro_rules! f32x4_max_n {
    ($vec:expr) => {
        $vec
    };

    ($vec:expr, $($rest:expr),+) => {
        ::core::arch::wasm32::f32x4_max($vec, f32x4_max_n!($($rest),+))
    };
}

macro_rules! f32x4_min_n {
    ($vec:expr) => {
        $vec
    };

    ($vec:expr, $($rest:expr),+) => {
        ::core::arch::wasm32::f32x4_min($vec, f32x4_min_n!($($rest),+))
    };
}

macro_rules! f32x4_sum_n {
    ($vec:expr) => {
        $vec
    };

    ($vec:expr, $($rest:expr),+) => {
        ::core::arch::wasm32::f32x4_add($vec, f32x4_sum_n!($($rest),+))
    };
}

pub fn generate_skewnorm_vec(
    count: usize,
    stats: Option<&CompetitorStats>,
    rng: &mut impl Rng,
    config: &RuntimeConfig,
    entered_times: &[i32],
) -> Vec<v128> {
    let mut values = Vec::with_capacity(count);

    let include_dnf = config.include_dnf;

    for i in 0..count {
        let solves = if i < entered_times.len() && entered_times[i] != 0 {
            if entered_times[i] < 0 {
                i32x4_splat(DNF_VALUE)
            } else {
                i32x4_splat(entered_times[i])
            }
        } else if let Some(stats) = stats {
            simd_gen_skewnorm(stats, rng, include_dnf)
        } else {
            i32x4_splat(DNF_VALUE)
        };

        values.push(solves);
    }

    values
}

fn gen_random_f32x4<T>(dist: &T, rng: &mut impl Rng) -> v128
where
    T: Distribution<f32>,
{
    let v1 = rng.sample(dist);
    let v2 = rng.sample(dist);
    let v3 = rng.sample(dist);
    let v4 = rng.sample(dist);

    f32x4(v1, v2, v3, v4)
}

// Truncates vales down to the nearest factor of 100 (used for FMC)
pub fn i32x4_truncate_down_100(vec: v128) -> v128 {
    let factor_div = f32x4_splat(100.0);
    let factor_mul = i32x4_splat(100);

    let truncated = f32x4_div(f32x4_convert_i32x4(vec), factor_div);
    let truncated_i32 = i32x4_trunc_sat_f32x4(truncated);

    i32x4_mul(truncated_i32, factor_mul)
}

pub fn i32x4_to_slice(vec: v128) -> [i32; 4] {
    [
        i32x4_extract_lane::<0>(vec),
        i32x4_extract_lane::<1>(vec),
        i32x4_extract_lane::<2>(vec),
        i32x4_extract_lane::<3>(vec),
    ]
}

pub fn f32x4_conditional_negate(input: v128, cond: v128) -> v128 {
    let mask = f32x4_gt(cond, f32x4_splat(0.0));
    let neg_u1 = f32x4_neg(input);

    v128_bitselect(input, neg_u1, mask)
}

pub fn simd_gen_skewnorm(
    stats: &CompetitorStats,
    rand_source: &mut impl Rng,
    include_dnf: bool,
) -> v128 {
    let normal_dist = Normal::new(0.0, 1.0).expect("Failed to initialize normal distribution");

    if stats.location.is_nan() || stats.shape.is_nan() || stats.dnf_rate.is_nan() {
        return i32x4_splat(DNF_VALUE);
    }

    let u0 = gen_random_f32x4(&normal_dist, rand_source);
    let v = gen_random_f32x4(&normal_dist, rand_source);

    let sigma = stats.skew / (1.0f32 + stats.skew.powi(2)).sqrt();

    let u1 = f32x4_mul(
        f32x4_add(
            f32x4_mul(f32x4_splat(sigma), u0),
            f32x4_mul(f32x4_splat((1.0f32 - sigma.powi(2)).sqrt()), v),
        ),
        f32x4_splat(stats.shape),
    );

    let u2 = f32x4_conditional_negate(u1, u0);
    let u3 = f32x4_add(u2, f32x4_splat(stats.location));

    let results_i32 = i32x4_trunc_sat_f32x4(u3);

    if !include_dnf {
        return results_i32;
    }

    let uniform_dist = Uniform::new(0.0, 1.0).expect("Failed to initialize uniform distribution");
    let r = gen_random_f32x4(&uniform_dist, rand_source);

    let mask = f32x4_gt(r, f32x4_splat(stats.dnf_rate));

    v128_bitselect(results_i32, i32x4_splat(DNF_VALUE), mask)
}

pub fn calc_wca_best_3(v1: v128, v2: v128, v3: v128) -> [i32; 4] {
    let max_v128 = f32x4_min_n!(v1, v2, v3);

    i32x4_to_slice(max_v128)
}

pub fn calc_wca_mean_3(v1: v128, v2: v128, v3: v128) -> [i32; 4] {
    let sum_v128 = f32x4_sum_n!(v1, v2, v3);
    let mean_v128 = f32x4_div(sum_v128, f32x4_splat(3.0));

    i32x4_to_slice(mean_v128)
}

pub fn calc_wca_average_5(v1: v128, v2: v128, v3: v128, v4: v128, v5: v128) -> [i32; 4] {
    let max_all = f32x4_max_n!(v1, v2, v3, v4, v5);
    let min_all = f32x4_min_n!(v1, v2, v3, v4, v5);

    let sum = f32x4_sum_n!(v1, v2, v3, v4, v5);
    let adjusted_sum = f32x4_sub(f32x4_sub(sum, max_all), min_all);

    i32x4_to_slice(f32x4_div(adjusted_sum, f32x4_splat(3.0)))
}
