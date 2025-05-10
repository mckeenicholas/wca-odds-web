use std::f32::consts::PI;

pub fn calc_weighted_mean_variance_stdev(data: &[(i32, f32)]) -> (f32, f32, f32) {
    if data.is_empty() {
        return (0.0, 0.0, 0.0);
    }

    let total_weight: f32 = data.iter().map(|(_, w)| *w).sum();

    if total_weight <= 0.0 {
        return (0.0, 0.0, 0.0);
    }

    // Calculate weighted mean
    let weighted_sum: f32 = data.iter().map(|&(val, weight)| val as f32 * weight).sum();
    let mean = weighted_sum / total_weight;

    // Calculate weighted variance (using reliability weights method)
    let weighted_sq_diff_sum: f32 = data
        .iter()
        .map(|&(val, weight)| weight * (val as f32 - mean).powi(2))
        .sum();

    // Bessel's correction for weighted data
    let variance = if data.len() > 1 {
        let effective_n = total_weight.powi(2) / data.iter().map(|(_, w)| w.powi(2)).sum::<f32>();
        weighted_sq_diff_sum / (total_weight * (effective_n - 1.0) / effective_n)
    } else {
        0.0
    };

    let stdev = variance.sqrt();

    (mean, variance, stdev)
}

pub fn trim_weighted_results(data: Vec<(i32, f32)>, mean: f32, stdev: f32) -> Vec<(i32, f32)> {
    let threshold = (mean + stdev * 2.0) as i32;
    data.into_iter()
        .filter(|&(val, _)| val <= threshold)
        .collect()
}

pub fn fit_weighted_skewnorm(data: &[(i32, f32)]) -> (f32, f32, f32) {
    let (mean, variance, stdev) = calc_weighted_mean_variance_stdev(data);

    if stdev == 0.0 {
        // Fall back to default values if we can't calculate meaningful statistics
        return (0.0, 1.0, mean);
    }

    let total_weight: f32 = data.iter().map(|(_, w)| *w).sum();

    // Calculate weighted skewness
    let weighted_skewness = data
        .iter()
        .map(|&(val, weight)| weight * ((val as f32 - mean) / stdev).powi(3))
        .sum::<f32>()
        / total_weight;

    let max_skew = 0.995 * ((4.0 - PI).sqrt() * (2.0 / PI).sqrt() * (1.0 - 2.0 / PI).powf(-1.5));
    let bounded_skew = weighted_skewness.clamp(-max_skew, max_skew);

    let delta = bounded_skew.signum()
        * ((PI / 2.0) * bounded_skew.abs().powf(2.0 / 3.0)
            / (bounded_skew.abs().powf(2.0 / 3.0) + ((4.0 - PI) / 2.0).powf(2.0 / 3.0)))
        .sqrt()
        .clamp(-0.995, 0.995);

    let alpha = delta / (1.0 - delta.powi(2)).sqrt();
    let omega = (variance / (1.0 - 2.0 * delta.powi(2) / PI)).sqrt();
    let xi = mean - omega * delta * (2.0 / PI).sqrt();

    (alpha, omega, xi)
}

pub fn find_lowest_indices(vec: &[i32]) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..vec.len()).collect();
    indices.sort_unstable_by_key(|&i| vec[i]);
    indices
}

pub fn transpose_solves(solves: Vec<[i32; 4]>) -> [Vec<i32>; 4] {
    let comp_length = solves.len();

    let mut out = [
        vec![0; comp_length],
        vec![0; comp_length],
        vec![0; comp_length],
        vec![0; comp_length],
    ];

    for (i, solve_set) in solves.iter().enumerate() {
        for (j, solve_value) in solve_set.iter().enumerate() {
            out[j][i] = *solve_value;
        }
    }

    out
}
