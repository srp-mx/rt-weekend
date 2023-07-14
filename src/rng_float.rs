use super::Float;
use rand_xoshiro::Xoshiro128Plus;
use rand::SeedableRng;
use rand::distributions::{Distribution, Uniform};

pub struct RngGen {
    rng: Xoshiro128Plus,
    dist: Uniform<Float>,
}

impl RngGen {
    pub fn new(min: Float, max: Float) -> Self {
        Self {
            rng: Xoshiro128Plus::from_entropy(),
            dist: Uniform::from(min..max),
        }
    }

    pub fn get(&mut self) -> Float {
        self.dist.sample(&mut self.rng)
    }
}
