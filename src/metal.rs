use super::rng_float::RngGen;
use super::material::{Material, Scatter};
use super::ray::Ray;
use super::hittable::HitRecord;
use super::vec3::Vec3;
use super::color::Color;

pub struct Metal {
    albedo: Color
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord, _rng: &mut RngGen) -> Scatter {
        let reflect = Vec3::reflect(&r_in.direction().unit_vector(), hit.normal());
        let new_ray = Ray::new(hit.p(), &reflect);
        return if Vec3::dot(new_ray.direction(), hit.normal()) > 0.0 {
            Scatter::Some(new_ray, self.albedo.copy())
        } else {
            Scatter::None
        }
    }
}
