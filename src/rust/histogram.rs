use std::{collections::HashMap, i32};

pub struct Histogram {
    min: i32,
    max: i32,

    times: HashMap<i32, i32>,
}

impl Histogram {
    pub fn decrease_min(&mut self, new_min: i32) {
        self.min = new_min.min(self.min).min(0);
    }

    pub fn increase_max(&mut self, new_max: i32) {
        self.max = new_max.max(self.max);
    }

    pub fn add_value(&mut self, value: i32) {
        if value < self.min || value > self.max {
            return;
        }

        *self.times.entry(value).or_insert(0) += 1;
    }
}

impl Default for Histogram {
    fn default() -> Self {
        Self {
            min: i32::MAX,
            max: 0,
            times: HashMap::new(),
        }
    }
}

impl Into<Vec<(i32, i32)>> for Histogram {
    fn into(self) -> Vec<(i32, i32)> {
        self.times.into_iter().collect::<Vec<_>>()
    }
}
