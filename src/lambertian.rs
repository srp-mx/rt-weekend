use super::rng_float::RngGen;
use super::material::{Material, Scatter};
use super::ray::Ray;
use super::hittable::HitRecord;
use super::vec3::Vec3;
use super::color::Color;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, hit: &HitRecord, rng: &mut RngGen) -> Scatter {
        let mut dir = hit.normal() + Vec3::random_unit(rng);

        if dir.near_zero() {
            dir = hit.normal().copy();
        }

        let new_ray = Ray::new(hit.p(), &dir);
        let new_color = self.albedo.copy();
        return Scatter::Some(new_ray, new_color)
    }
}
