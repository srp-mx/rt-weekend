use super::material::{Material, Scatter};
use super::float::Float;
use super::vec3::Vec3;
use super::ray::Ray;
use super::color::Color;
use super::hittable::HitRecord;
use super::rng_float::RngGen;

pub struct Dielectric {
    ior: Float // index of refraction
}

impl Dielectric {
    pub fn new(ior: Float) -> Self {
        Self { ior }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord, _rng: &mut RngGen) -> Scatter {
        let new_color = Color::one();
        let refraction_ratio = if hit.front_face {
            self.ior.recip()
        } else {
            self.ior
        };

        let ref unit_direction: Vec3 = r_in.direction().unit_vector();
        let cos_theta = Vec3::dot(&-unit_direction, hit.normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let direction: Vec3 = if refraction_ratio * sin_theta > 1.0 {
            unit_direction.reflect(hit.normal())
        } else {
            unit_direction.refract(hit.normal(), refraction_ratio)
        };

        let new_ray = Ray::new(hit.p(), &direction);
        Scatter::Some(new_ray, new_color)
    }
}
