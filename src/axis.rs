use super::rng_float::RngGen;
use super::float::Float;
use super::vec3::Vec3;

pub enum Axis {
    X,
    Y,
    Z
}

impl Axis {
    pub fn rand(rng: &mut RngGen) -> Self {
        match rng.range(0.0, 3.0) as i32 {
            0 => Axis::X,
            1 => Axis::Y,
            _ => Axis::Z
        }
    }
}

impl Vec3 {
    pub fn axis(&self, axis: &Axis) -> Float {
        match axis {
            Axis::X => self.x(),
            Axis::Y => self.y(),
            Axis::Z => self.z()
        }
    }
}
