use super::float::Float;
use super::vec3::Vec3;
type Point3 = Vec3;
use super::ray::Ray;
use super::color::Color;
use super::hittable::HitRecord;
use super::rng_float::RngGen;

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord, rng: &mut RngGen) -> Scatter;

    fn emmited(&self, _u: Float, _v: Float, _p: &Point3) -> Color {
        Color::zero()
    }
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
