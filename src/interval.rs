#[derive(Copy, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

pub const EMPTY: Interval = Interval{
    min: f64::INFINITY,
    max: f64::NEG_INFINITY,
};

pub const UNIVERSE: Interval = Interval{
    min: f64::NEG_INFINITY,
    max: f64::INFINITY,
};

impl Interval {
    pub fn new(min: f64, max: f64) -> Interval {
        Interval { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, t: f64) -> bool {
        self.min <= t && t <= self.max
    }

    pub fn surrounds(&self, t: f64) -> bool {
        self.min < t && t < self.max
    }
}

