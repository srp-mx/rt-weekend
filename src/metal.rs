use super::float::Float;
use super::rng_float::RngGen;
use super::material::{Material, Scatter};
use super::ray::Ray;
use super::hittable::HitRecord;
use super::vec3::Vec3;
use super::color::Color;

pub struct Metal {
    albedo: Color,
    fuzz: Float,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: Float) -> Self {
        Self { albedo, fuzz: if fuzz < 1.0 { fuzz } else { 1.0 } }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord, rng: &mut RngGen) -> Scatter {
        let reflect = Vec3::reflect(&r_in.direction().unit_vector(), hit.normal());
        let fuzz_offset = self.fuzz * &Vec3::random_sphere(rng);
        let new_ray = Ray::new(hit.p(), &(&reflect + fuzz_offset), r_in.time());
        return if Vec3::dot(new_ray.direction(), hit.normal()) > 0.0 {
            Scatter::Some(new_ray, self.albedo.copy())
        } else {
            Scatter::None
        }
    }
}
