use std::{fmt, ops};

#[derive(Clone, Copy, Debug)]
pub struct Record {
    low: f32,
    high: f32,
    sum: f64,
    count: u32,
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}/{:.1}/{:.1}", self.low, self.mean(), self.high)
    }
}

impl ops::AddAssign for Record {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            low: self.low.min(rhs.low),
            high: self.high.max(rhs.high),
            sum: self.sum + rhs.sum,
            count: self.count + rhs.count,
        }
    }
}

impl Record {
    pub fn new(val: f32) -> Self {
        Self {
            low: val,
            high: val,
            sum: val as f64,
            count: 1,
        }
    }

    fn mean(&self) -> f32 {
        (((self.sum) / (self.count as f64) * 10.0).ceil() / 10.0) as f32
    }
}
