use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;
use wasm_bindgen::prelude::*;

#[derive(Deserialize)]
pub struct Competitors {
    pub results: Vec<Vec<i32>>,
}

#[derive(Serialize)]
pub struct PersonData {
    pub wins: i32,
    pub podiums: i32,
    pub mu: f64,
    pub sigma: f64,
    pub tau: f64,
}

#[derive(Serialize)]
pub struct ReturnData {
    pub persons: Vec<PersonData>,
}

fn find_lowest_indices(vec: &[i32], count: usize) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..vec.len()).collect();
    indices.sort_by_key(|&i| vec[i]);
    indices.truncate(count);
    indices
}

fn random_exgauss(mu: f64, sample_dev: f64, tau: f64, dnf_rate: f64) -> f64 {
    let mut rng = thread_rng();
    let range = Uniform::new(1e-10, 1.0);
    let (u1, u2, u3, u4): (f64, f64, f64, f64) = (
        rng.sample(&range),
        rng.sample(&range),
        rng.sample(&range),
        rng.sample(&range),
    );

    if u4 < dnf_rate {
        return f64::MAX;
    }

    let r = (-2.0 * u1.ln()).sqrt();
    let o = 2.0 * PI * u2;
    let exp = -tau * (1.0 - u3).ln();

    (r * o.cos()) * sample_dev + exp + mu
}

fn calc_wca_average(average: &mut [i32], format: char) -> i32 {
    average.sort();
    if format == 'a' {
        (average[1] / 3) + (average[2] / 3) + (average[3] / 3)
    } else if format == 'm' {
        (average[0] / 3) + (average[1] / 3) + (average[2] / 3)
    } else {
        average[0]
    }
}

fn mean(data: &[i32]) -> f64 {
    data.iter().copied().sum::<i32>() as f64 / data.len() as f64
}

fn stdev(data: &[i32], mean: f64) -> f64 {
    (data.iter().map(|&x| (x as f64 - mean).powi(2)).sum::<f64>() / data.len() as f64).sqrt()
}

#[wasm_bindgen]
pub fn simulate(
    competitor_struct: JsValue,
    simulations: usize,
    format: char,
) -> Result<JsValue, JsValue> {
    let num_attempts: i8 = if format == 'a' { 5 } else { 3 };

    let competitors: Competitors = serde_wasm_bindgen::from_value(competitor_struct)?;
    let num_competitors = competitors.results.len();

    let results: Vec<_> = competitors
        .results
        .iter()
        .map(|result| {
            let result_no_dnf: Vec<i32> = result.iter().filter(|&&x| x != -1).copied().collect();
            let num_dnf = result.iter().filter(|&&x| x == -1).count();
            let dnf_rate: f64 = num_dnf as f64 / result.len() as f64;

            let sample_mean = mean(&result_no_dnf);
            let sample_dev = stdev(&result_no_dnf, sample_mean);

            let trimmed_results: Vec<i32> = result_no_dnf
                .iter()
                .filter(|&&x| x < (sample_mean + sample_dev * 3.0) as i32)
                .copied()
                .collect();
            let trimmed_mean = mean(&trimmed_results);
            let trimmed_dev = stdev(&trimmed_results, trimmed_mean);

            let gamma: f64 = trimmed_results
                .iter()
                .map(|&x| ((x as f64 - trimmed_mean) / trimmed_dev).powi(3))
                .sum::<f64>()
                / trimmed_results.len() as f64;
            let gamma_trimmed = gamma.min(0.99).max(0.0);

            let mu = sample_mean - trimmed_dev * (gamma_trimmed / 2.0).powf(1.0 / 3.0);
            let sigma =
                (trimmed_dev.powi(2) * (1.0 - (gamma_trimmed / 2.0).powf(2.0 / 3.0))).sqrt();
            let tau = trimmed_dev * (gamma_trimmed / 2.0).powf(1.0 / 3.0);

            (mu, sigma, tau, dnf_rate)
        })
        .collect();

    let mut win_count = vec![0; num_competitors];
    let mut pod_count = vec![0; num_competitors];

    for _ in 0..simulations {
        let mut averages = Vec::with_capacity(num_competitors);

        for i in 0..num_competitors {
            let (mu, sigma, tau, dnf_rate) = results[i];

            let mut average: Vec<i32> = (0..num_attempts)
                .map(|_| random_exgauss(mu, sigma, tau, dnf_rate) as i32)
                .collect();

            let avg = calc_wca_average(&mut average, format);
            averages.push(avg);
        }

        let lowest_index = find_lowest_indices(&averages, 1)[0];
        win_count[lowest_index] += 1;

        let lowest_pod_indices = find_lowest_indices(&averages, 3);
        for index in lowest_pod_indices {
            pod_count[index] += 1;
        }
    }

    let mut output = Vec::with_capacity(num_competitors);
    for i in 0..num_competitors {
        let (mu, sigma, tau, _) = results[i];
        let person_data = PersonData {
            wins: win_count[i],
            podiums: pod_count[i],
            mu: mu,
            sigma: sigma,
            tau: tau,
        };
        output.push(person_data);
    }
    Ok(serde_wasm_bindgen::to_value(&output)?)
}
