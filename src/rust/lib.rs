use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use wasm_bindgen::prelude::*;
use web_sys::js_sys::Promise;

mod data;
use data::{get_competition_data, get_solve_data, CompetitionResult};

mod simulation;

#[derive(Serialize, Deserialize, Debug)]
pub struct DatedCompetitionResult {
    pub date: i64,
    pub results: Vec<i32>,
}

#[derive(Serialize)]
pub struct PersonData {
    pub wins: i32,
    pub podiums: i32,
    pub mean: f64,
    pub gamma: f64,
    pub stdev: f64,
    pub mu: f64,
    pub sigma: f64,
    pub tau: f64,
    pub dnf_rate: f64,
    pub avg_rank: f64,
    pub ranks: Vec<i32>,
}

#[derive(Serialize)]
pub struct ReturnData {
    pub persons: Vec<PersonData>,
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!($($t)*).as_str()));
    }
}


#[wasm_bindgen]
pub fn run_odds_simulation(competitors: Vec<String>, event: String, month_cutoff: i32) -> Promise {
    let future = async move {
        let parsed_data = fetch_and_join(competitors, event, month_cutoff)
            .await
            .map_err(|e| {
                serde_wasm_bindgen::to_value(&format!("{}", e)).expect("Error parsing message")
            })?;

        console_log!("{:?}", format_results(parsed_data));

        // Testing code for WASM binding
        let result = vec![1, 2, 3];

        serde_wasm_bindgen::to_value(&result)
            .map_err(|e| serde_wasm_bindgen::to_value(&format!("{}", e)).unwrap())
    };

    wasm_bindgen_futures::future_to_promise(future)
}

fn format_results(data: Vec<Vec<DatedCompetitionResult>>) -> String {
    let all_results = data.into_iter().map(|person| person.into_iter().flat_map(|dated_result| dated_result.results).collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut base = String::new();

    for item in all_results {
        base = base + &format!("Length: {}, times: {:?}", item.len(), item);
    }

    base
}

fn join_data(
    competitions: HashMap<String, i64>,
    results: Vec<Vec<CompetitionResult>>,
) -> Vec<Vec<DatedCompetitionResult>> {
    results
        .into_iter()
        .map(|competitor| {
            competitor
                .into_iter()
                .filter_map(|competition| {
                    match competitions.get(&competition.id) {
                        Some(comp_date) => {
                            Some(DatedCompetitionResult {
                                date: *comp_date,
                                results: competition.results,
                            })
                        },
                        None => None
                    }
                })
                .collect()
        })
        .collect()
}

pub async fn fetch_and_join(
    competitors: Vec<String>,
    event: String,
    month_cutoff: i32,
) -> Result<Vec<Vec<DatedCompetitionResult>>, Box<dyn Error>> {
    let competitions = get_competition_data(month_cutoff).await?;
    let results = get_solve_data(competitors, event).await?;

    Ok(join_data(competitions, results))
}

// fn find_lowest_indices(vec: &[i32]) -> Vec<usize> {
//     let mut indices: Vec<usize> = (0..vec.len()).collect();
//     indices.sort_unstable_by_key(|&i| vec[i]);
//     indices
// }

// fn calc_wca_average_5(average: &mut [i32; 5]) -> i32 {
//     average.sort_unstable();
//     (average[1] / 3) + (average[2] / 3) + (average[3] / 3)
// }

// fn calc_wca_average(average: &mut [i32], format: char) -> i32 {
//     average.sort_unstable();
//     if format == 'a' {
//         (average[1] / 3) + (average[2] / 3) + (average[3] / 3)
//     } else if format == 'm' {
//         (average[0] / 3) + (average[1] / 3) + (average[2] / 3)
//     } else {
//         average[0]
//     }
// }

// fn mean(data: &[i32]) -> f64 {
//     data.iter().copied().sum::<i32>() as f64 / data.len() as f64
// }

// fn stdev(data: &[i32], mean: f64) -> f64 {
//     (data.iter().map(|&x| (x as f64 - mean).powi(2)).sum::<f64>() / data.len() as f64).sqrt()
// }

// #[wasm_bindgen]
// pub fn simulate(
//     competitor_struct: JsValue,
//     simulations: usize,
//     format: char,
// ) -> Result<JsValue, JsValue> {
//     let num_attempts: i8 = if format == 'a' { 5 } else { 3 };

//     let competitors: Competitors = serde_wasm_bindgen::from_value(competitor_struct)?;
//     let num_competitors = competitors.results.len();

//     let results: Vec<_> = competitors
//         .results
//         .iter()
//         .map(|result| {
//             let result_no_dnf: Vec<i32> = result.iter().filter(|&&x| x > 0).copied().collect();
//             let num_dnf = result.iter().filter(|&&x| x == -1).count();
//             let dnf_rate: f64 = num_dnf as f64 / result.len() as f64;

//             let sample_mean = mean(&result_no_dnf);
//             let sample_dev = stdev(&result_no_dnf, sample_mean);

//             let trimmed_results: Vec<i32> = result_no_dnf
//                 .iter()
//                 .filter(|&&x| x <= (sample_mean + sample_dev * 3.0) as i32)
//                 .copied()
//                 .collect();
//             let trimmed_mean = mean(&trimmed_results);
//             let trimmed_dev = stdev(&trimmed_results, trimmed_mean);

//             let gamma: f64 = trimmed_results
//                 .iter()
//                 .map(|&x| ((x as f64 - trimmed_mean) / trimmed_dev).powi(3))
//                 .sum::<f64>()
//                 / trimmed_results.len() as f64;
//             let gamma_trimmed = gamma.min(0.99).max(0.0);

//             let mu = sample_mean - trimmed_dev * (gamma_trimmed / 2.0).powf(1.0 / 3.0);
//             let sigma =
//                 (trimmed_dev.powi(2) * (1.0 - (gamma_trimmed / 2.0).powf(2.0 / 3.0))).sqrt();
//             let tau = trimmed_dev * (gamma_trimmed / 2.0).powf(1.0 / 3.0);

//             (sample_mean, sample_dev, gamma, mu, sigma, tau, dnf_rate)
//         })
//         .collect();

//     let mut win_count = vec![0; num_competitors];
//     let mut pod_count = vec![0; num_competitors];
//     let mut total_rank = vec![0; num_competitors];
//     let mut rank_dist = vec![vec![0; num_competitors]; num_competitors];

//     for _ in 0..simulations {
//         let mut averages = Vec::with_capacity(num_competitors);

//         for i in 0..num_competitors {
//             let (_, _, _, mu, sigma, tau, dnf_rate) = results[i];

//             let mut average: Vec<i32> = (0..num_attempts)
//                 .map(|_| random_exgauss(mu, sigma, tau, dnf_rate) as i32)
//                 .collect();

//             let avg = calc_wca_average(&mut average, format);
//             averages.push(avg);
//         }

//         let lowest_pod_indices = find_lowest_indices(&averages);
//         win_count[lowest_pod_indices[0]] += 1;
//         for &index in lowest_pod_indices.iter().take(3) {
//             pod_count[index] += 1;
//         }

//         for i in 0..num_competitors {
//             total_rank[i] += lowest_pod_indices[i] + 1;
//             rank_dist[lowest_pod_indices[i]][i] += 1;
//         }
//     }

//     let output: Vec<_> = (0..num_competitors)
//         .into_iter()
//         .map(|i| {
//             let (sample_mean, sample_dev, gamma, mu, sigma, tau, dnf_rate) = results[i];
//             PersonData {
//                 wins: win_count[i],
//                 podiums: pod_count[i],
//                 mean: sample_mean,
//                 stdev: sample_dev,
//                 gamma: gamma,
//                 mu: mu,
//                 sigma: sigma,
//                 tau: tau,
//                 dnf_rate: dnf_rate,
//                 avg_rank: total_rank[i] as f64 / simulations as f64,
//                 ranks: rank_dist[i].clone(),
//             }
//         })
//         .collect();

//     Ok(serde_wasm_bindgen::to_value(&output)?)
// }
