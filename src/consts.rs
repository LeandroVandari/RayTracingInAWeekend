pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;
pub static MINUS_ONE_TO_ONE: once_cell::sync::Lazy<rand::distributions::Uniform<f64>> =
    once_cell::sync::Lazy::new(|| rand::distributions::Uniform::new_inclusive(-1.0, 1.0));

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn empty() -> Self {
        Self {
            min: INFINITY,
            max: -INFINITY,
        }
    }

    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, num: f64) -> bool {
        self.min <= num && num <= self.max
    }

    pub fn surrounds(&self, num: f64) -> bool {
        self.min < num && num < self.max
    }

    pub fn universe() -> Self {
        Self {
            min: -INFINITY,
            max: INFINITY,
        }
    }
    pub fn clamp(&self, val: f64) -> f64 {
        if val < self.min {
            return self.min;
        } else if val > self.max {
            return self.max;
        }
        val
    }
}
