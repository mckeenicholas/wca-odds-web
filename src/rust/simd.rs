use core::arch::wasm32::{
    f32x4, f32x4_add, f32x4_div, f32x4_gt, f32x4_max, f32x4_min, f32x4_mul, f32x4_neg, f32x4_splat,
    f32x4_sub, i32x4_extract_lane, i32x4_splat, i32x4_trunc_sat_f32x4, v128, v128_bitselect,
};
use rand::rngs::ThreadRng;
use rand_distr::{Distribution, Normal};

use crate::simulation::CompetitorStats;

#[macro_export]
macro_rules! gen_n_skewnorm_simd {
    ($n:literal, $stats:expr, $rng:expr, $include_dnf:expr, $entered_times:expr) => {{
        let mut values = [::core::arch::wasm32::i32x4_splat(0); $n];

        for i in 0..$n {
            values[i] = if $entered_times[i] == 0 {
                $crate::simd::gen_skewnorm_simd($stats, $rng, $include_dnf)
            } else {
                ::core::arch::wasm32::i32x4_splat($entered_times[i])
            };
        }
        values
    }};
}

#[macro_export]
macro_rules! gen_n_random {
    ($n:literal, $source:expr, $rng:expr) => {{
        let mut values = [0.0; $n];
        for i in 0..$n {
            values[i] = $rng.sample($source);
        }
        values
    }};
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

pub fn gen_skewnorm_simd(
    stats: &CompetitorStats,
    rand_source: &mut ThreadRng,
    include_dnf: bool,
) -> v128 {
    let normal_dist = Normal::new(0.0, 1.0).expect("Failed to initialize normal distribution");

    let [v1, v2, v3, v4] = gen_n_random!(4, rand_source, normal_dist);
    let [w1, w2, w3, w4] = gen_n_random!(4, rand_source, normal_dist);

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

    let results_i32 = i32x4_trunc_sat_f32x4(u3);

    if !include_dnf {
        return results_i32;
    }

    let uniform_dist = rand::distributions::Uniform::new(0.0, 1.0);
    let [r1, r2, r3, r4] = gen_n_random!(4, rand_source, uniform_dist);

    let mask = f32x4_gt(f32x4(r1, r2, r3, r4), f32x4_splat(stats.dnf_rate));

    v128_bitselect(results_i32, i32x4_splat(i32::MAX), mask)
}

pub fn calc_wca_best_3(v1: v128, v2: v128, v3: v128) -> [i32; 4] {
    let max_1_2 = f32x4_max(v1, v2);
    let max_v128 = f32x4_max(max_1_2, v3);

    i32x4_to_slice(max_v128)
}

pub fn calc_wca_mean_3(v1: v128, v2: v128, v3: v128) -> [i32; 4] {
    let sum_1_2 = f32x4_add(v1, v2);
    let sum_v128 = f32x4_add(sum_1_2, v3);
    let mean_v128 = f32x4_div(sum_v128, f32x4_splat(3.0));

    i32x4_to_slice(mean_v128)
}

pub fn calc_wca_average_5(v1: v128, v2: v128, v3: v128, v4: v128, v5: v128) -> [i32; 4] {
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
