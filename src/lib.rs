use wasm_bindgen::prelude::*;
use rand::Rng;
use std::f64::consts::PI;
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct Competitors {
    pub results: Vec<Vec<i32>>
}

#[derive(Serialize)]
pub struct PersonData {
    pub wins: i32,
    pub podiums: i32,
    pub alpha: f64,
    pub omega: f64,
    pub xi: f64,
}

#[derive(Serialize)]
pub struct ReturnData {
    pub persons: Vec<PersonData>
}

fn find_lowest_indices(vec: &[i32], count: usize) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..vec.len()).collect();
    indices.sort_by_key(|&i| vec[i]);
    indices.truncate(count);
    indices
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

fn random_skew_normal(xi: f64, omega: f64, alpha: f64, delta: f64, dnf_rate: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let dnf: f64 = rng.gen();

    if dnf < dnf_rate {
        return f64::MAX;
    }

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

#[wasm_bindgen]
pub fn simulate(competitor_struct: JsValue, simulations: usize, format: char) -> Result<JsValue, JsValue> {
    let num_attempts: i8 = if format == 'a' { 5 } else { 3 };

    let competitors: Competitors = serde_wasm_bindgen::from_value(competitor_struct)?;
    let num_competitors = competitors.results.len();

    let results: Vec<_> = competitors.results.iter().map(|result| {
        let result_no_dnf: Vec<i32> = result.iter().filter(|&&x| x != -1).copied().collect();
        let num_dnf = result.iter().filter(|&&x| x == -1).count();
        let dnf_rate: f64 = num_dnf as f64 / result.len() as f64;

        let mu: f64 = result_no_dnf.iter().copied().sum::<i32>() as f64 / result_no_dnf.len() as f64;
        let sigma: f64 = (result_no_dnf.iter().map(|&x| (x as f64 - mu).powi(2)).sum::<f64>() / result_no_dnf.len() as f64).sqrt();
        let gamma_untrimmed: f64 = result_no_dnf.iter().map(|&x| ((x as f64 - mu) / sigma).powi(3)).sum::<f64>() / result_no_dnf.len() as f64;
        let gamma = gamma_untrimmed.abs().min(0.99);
        let delta = (gamma.powf(2.0 / 3.0) / ((gamma.powf(2.0 / 3.0) + ((4.0 - PI) / 2.0).powf(2.0 / 3.0))) * (PI / 2.0)).sqrt();
        let alpha = delta / (1.0 - delta.powi(2)).sqrt();
        let omega = sigma / (1.0 - (2.0 * delta.powi(2) / PI)).sqrt();
        let xi = mu - omega * delta * (2.0 / PI).sqrt();

        (delta, alpha, omega, xi, dnf_rate)
    }).collect();

    let mut win_count = vec![0; num_competitors];
    let mut pod_count = vec![0; num_competitors];
    
    for _ in 0..simulations {
        let mut averages = Vec::with_capacity(num_competitors);
    
        for i in 0..num_competitors {
            let (delta, alpha, omega, xi, dnf_rate) = results[i];
    
            let mut average: Vec<i32> = (0..num_attempts)
                .map(|_| random_skew_normal(xi, omega, alpha, delta, dnf_rate) as i32)
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
        let (_, alpha, omega, xi, _) = results[i];
        let person_data = PersonData {
            wins: win_count[i],
            podiums: pod_count[i],
            alpha: alpha,
            omega: omega,
            xi: xi,
        };
        output.push(person_data);
    }

    Ok(serde_wasm_bindgen::to_value(&output)?)
}


// TODO: Possibly implement table joins in rust
// use std::collections::HashMap;

// struct Competition {
//     id: String,
//     date_ms: i64, // Store date as milliseconds since epoch
// }

// struct Competitor {
//     results: HashMap<String, HashMap<String, Vec<Round>>>,
// }

// struct Round {
//     solves: Vec<i32>,
// }

// fn date_to_ms(date: &str) -> Result<i64, chrono::ParseError> {
//     let dt = chrono::DateTime::parse_from_rfc3339(date)?;
//     Ok(dt.timestamp_millis())
// }

// fn filter_competitions(
//     competitions: Vec<Competition>,
//     competitors: Vec<Competitor>,
//     event: &str,
//     start_date: i64,
// ) -> Vec<Vec<i32>> {
//     let num_solves = 1; // Assuming numSolves is a constant or computed elsewhere
//     let mut results = Vec::new();

//     for person in competitors {
//         let person_results = person.results.get(event);

//         if let Some(person_results) = person_results {
//             for (key, values) in person_results.iter() {
//                 if let Some(comp) = competitions.iter().find(|comp| comp.id == key) {
//                     if comp.date_ms > start_date {
//                         if let Some(event_rounds) = values.get(event) {
//                             for round in event_rounds {
//                                 let solves = round.solves.iter()
//                                     .take(num_solves)
//                                     .filter(|&solve| *solve != 0)
//                                     .cloned()
//                                     .collect::<Vec<i32>>();

//                                 results.push(solves);
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }

//     results
// }