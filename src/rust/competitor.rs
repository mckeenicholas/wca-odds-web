use std::u32;

use serde::{Deserialize, Serialize};

use crate::{
    calc::{calc_weighted_mean_variance_stdev, fit_weighted_skewnorm, trim_weighted_results},
    simd::DNF_TEMP_VALUE,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatedCompetitionResult {
    pub days_since: i32,
    pub results: Vec<i32>,
}

pub struct Competitor {
    pub name: String,
    pub results: Vec<DatedCompetitionResult>,
    pub entered_results: Vec<i32>,
    pub stats: Option<CompetitorStats>,
}

#[derive(Debug)]
pub struct CompetitorStats {
    pub location: f32,
    pub shape: f32,
    pub skew: f32,
    pub dnf_rate: f32,
    pub mean: f32,
    pub stdev: f32,
    pub num_non_dnf_results: u32,
}

impl Competitor {
    pub fn new(name: String, results: Vec<DatedCompetitionResult>, halflife: f32) -> Self {
        let stats = Competitor::calculate_stats(&results, halflife);

        Self {
            name,
            results,
            entered_results: vec![],
            stats,
        }
    }

    fn calculate_stats(
        results: &Vec<DatedCompetitionResult>,
        halflife: f32,
    ) -> Option<CompetitorStats> {
        let weighted_results = Self::apply_exponential_weights(results, halflife);

        if weighted_results.is_empty() {
            return None;
        }

        // Calculate weighted DNF rate
        let (dnf_weighted_count, total_weight) =
            weighted_results
                .iter()
                .fold((0.0, 0.0), |(dnf_sum, weight_sum), &(val, weight)| {
                    if val < 0 {
                        (dnf_sum + weight, weight_sum + weight)
                    } else {
                        (dnf_sum, weight_sum + weight)
                    }
                });

        let dnf_rate = if total_weight > 0.0 {
            dnf_weighted_count / total_weight
        } else {
            0.0
        };

        // Filter out DNFs for time calculations
        let non_dnf_weighted_results: Vec<(i32, f32)> = weighted_results
            .into_iter()
            .filter(|&(val, _)| val > 0)
            .collect();

        if non_dnf_weighted_results.is_empty() {
            return None;
        }

        let num_non_dnf_results = non_dnf_weighted_results.len() as u32;

        // Calculate weighted statistics
        let (sample_mean, _sample_variance, sample_dev) =
            calc_weighted_mean_variance_stdev(&non_dnf_weighted_results);

        // Trim outliers
        let trimmed_weighted_results =
            trim_weighted_results(non_dnf_weighted_results, sample_mean, sample_dev);

        // Fit distribution
        let (skew, shape, location) = fit_weighted_skewnorm(&trimmed_weighted_results);

        Some(CompetitorStats {
            location,
            shape,
            skew,
            dnf_rate,
            mean: sample_mean,
            stdev: sample_dev,
            num_non_dnf_results,
        })
    }

    fn apply_exponential_weights(
        results: &Vec<DatedCompetitionResult>,
        halflife: f32,
    ) -> Vec<(i32, f32)> {
        const LN2: f32 = 0.69314718056;
        let decay_rate: f32 = LN2 / halflife;

        let mut weighted_results = Vec::new();

        for result_set in results {
            let weight = (-decay_rate * result_set.days_since as f32).exp();

            for &time in &result_set.results {
                weighted_results.push((time, weight));
            }
        }

        weighted_results
    }

    pub fn add_entered_results(&mut self, results: Vec<i32>) {
        self.entered_results = results;
    }

    pub fn get_sample_size(&self) -> u32 {
        self.stats
            .as_ref()
            .map_or(0, |stats| stats.num_non_dnf_results)
    }

    pub fn get_mean(&self) -> u32 {
        self.stats
            .as_ref()
            .map_or(DNF_TEMP_VALUE as u32, |stats| stats.mean as u32)
    }

    pub fn get_person_hist_bounds(&self) -> (i32, i32) {
        if let Some(stats) = &self.stats {
            let hist_min = ((stats.mean - stats.stdev * 4.0) / 10.0) as i32;
            let hist_max = ((stats.mean + stats.stdev * 4.0) / 10.0) as i32;

            (hist_min, hist_max)
        } else {
            (i32::MAX, 0)
        }
    }
}
