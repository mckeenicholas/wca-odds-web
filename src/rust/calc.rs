use std::f32::consts::PI;

pub fn calc_mean_variance_stdev(data: &[i32]) -> (f32, f32, f32) {
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

pub fn trim_errant_results(mut results: Vec<i32>, mean: f32, stdev: f32) -> Vec<i32> {
    let threshold = (mean + stdev * 2.0) as i32;
    results.retain(|&x| x <= threshold);
    results
}

pub fn fit_skewnorm(times: &[i32]) -> (f32, f32, f32) {
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

    for i in 0..comp_length {
        for j in 0..4 {
            out[j][i] = solves[i][j];
        }
    }

    out
}
