use super::ray::Ray;
use super::color::Color;
use super::hittable::HitRecord;
use super::rng_float::RngGen;

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord, rng: &mut RngGen) -> Scatter;
}

pub enum Scatter {
    Some(Ray, Color),
    None
}

pub struct NullMaterial;

impl Material for NullMaterial {
    fn scatter(&self, _: &Ray, _: &HitRecord, _: &mut RngGen) -> Scatter {
        Scatter::None
    }
}
