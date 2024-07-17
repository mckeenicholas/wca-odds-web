use wasm_bindgen::prelude::*;
use rand::Rng;
use std::f64::consts::PI;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Competitors {
    pub results: Vec<Vec<i32>>
}

fn random_normals() -> (f64, f64) {
    let mut rng = rand::thread_rng();
    let (mut u1, mut u2): (f64, f64) = (0.0, 0.0);

    while u1 == 0.0 {
        u1 = rng.gen();
    }
    while u2 == 0.0 {
        u2 = rng.gen();
    }

    let r = (-2.0 * u1.ln()).sqrt();
    let o = 2.0 * PI * u2;
    (r * o.cos(), r * o.sin())
}

fn random_skew_normal(xi: f64, omega: f64, alpha: f64, delta: f64) -> f64 {
    let (u0, v) = random_normals();
    if alpha == 0.0 {
        return xi + omega * u0;
    }
    let z = (delta * u0 + (1.0 - delta * delta).sqrt() * v).abs();
    xi + omega * z
}

fn calc_wca_average(average: &mut [i32], format: char) -> i32 {
    average.sort();
    if format == 'a' {
        (average[1] + average[2] + average[3]) / 3 as i32
    } else if format == 'm' {
        (average[0] + average[1] + average[2]) / 3 as i32
    } else {
        average[0]
    }
}
// data: &[i32]
#[wasm_bindgen]
pub fn simulate(competitor_struct: JsValue, simulations: usize, format: char) -> Result<(), JsValue> {
    let num_attempts: i8 = if format == 'a' { 5 } else { 3 };

    let competitors: Competitors = serde_wasm_bindgen::from_value(competitor_struct)?;
    let num_competitors = competitors.results.len();

    let results: Vec<_> = competitors.results.iter().map(|result| {
        let mu: f64 = result.iter().copied().sum::<i32>() as f64 / result.len() as f64;
        let sigma: f64 = (result.iter().map(|&x| (x as f64 - mu).powi(2)).sum::<f64>() / result.len() as f64).sqrt();
        let a: f64 = result.iter().map(|&x| ((x as f64 - mu) / sigma).powi(3)).sum::<f64>() / result.len() as f64;
        let gamma = a.abs().min(0.99);
        let delta = (gamma.powf(2.0 / 3.0) / ((gamma.powf(2.0 / 3.0) + ((4.0 - PI) / 2.0).powf(2.0 / 3.0))) * (PI / 2.0)).sqrt();
        let alpha = delta / (1.0 - delta.powi(2)).sqrt();
        let omega = sigma / (1.0 - (2.0 * delta.powi(2) / PI)).sqrt();
        let xi = mu - omega * delta * (2.0 / PI).sqrt();

        (delta, alpha, omega, xi)
    }).collect();

    // let mu: f64 = data.iter().copied().sum::<i32>() as f64 / data.len() as f64;
    // let sigma: f64 = (data.iter().map(|&x| (x as f64 - mu).powi(2)).sum::<f64>() / data.len() as f64).sqrt();
    // let a: f64 = data.iter().map(|&x| ((x as f64 - mu) / sigma).powi(3)).sum::<f64>() / data.len() as f64;
    // let gamma = a.abs().min(0.99);
    // let delta = (gamma.powf(2.0 / 3.0) / ((gamma.powf(2.0 / 3.0) + ((4.0 - PI) / 2.0).powf(2.0 / 3.0))) * (PI / 2.0)).sqrt();
    // let alpha = delta / (1.0 - delta.powi(2)).sqrt();
    // let omega = sigma / (1.0 - (2.0 * delta.powi(2) / PI)).sqrt();
    // let xi = mu - omega * delta * (2.0 / PI).sqrt();

    let mut averages = Vec::with_capacity(simulations);
    for _ in 0..simulations {
        for i in 0..num_competitors {
            let (delta, alpha, omega, xi) = results[i];
            let mut average: Vec<i32> = (0..num_attempts)
        .map(|_| random_skew_normal(xi, omega, alpha, delta) as i32)
        .collect();
        averages.push(calc_wca_average(&mut average, format))
        }
    }
    // averages

    Ok(())
}
