use super::Float;
use rand_xoshiro::Xoshiro128Plus;
use rand::SeedableRng;
use rand::distributions::{Distribution, Uniform};

pub struct RngGen {
    rng: Xoshiro128Plus,
    dist: Uniform<Float>,
}

impl RngGen {
    pub fn new() -> Self {
        Self {
            rng: Xoshiro128Plus::from_entropy(),
            dist: Uniform::from(0.0..1.0),
        }
    }

    pub fn get(&mut self) -> Float {
        self.dist.sample(&mut self.rng)
    }

    pub fn range(&mut self, min: Float, max: Float) -> Float {
        min + (max-min)*self.get()
    }

    pub fn usize(&mut self, min: i32, max: usize) -> usize {
        self.range(min as Float, max as Float + 1.0) as usize
    }
}
