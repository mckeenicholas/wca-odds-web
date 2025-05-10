use std::collections::HashMap;

#[derive(Debug)]
pub struct Histogram {
    min: i32,
    max: i32,

    times: HashMap<i32, i32>,
}

impl Histogram {
    pub fn new_with_bounds(min: i32, max: i32) -> Self {
        Self {
            min,
            max,
            times: HashMap::new(),
        }
    }

    pub fn add_value(&mut self, value: i32) {
        if value < self.min || value > self.max {
            return;
        }

        *self.times.entry(value).or_insert(0) += 1;
    }

    // Gets the data and consumes self
    pub fn data(self) -> HashMap<i32, i32> {
        self.times
    }
}
