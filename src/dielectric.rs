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

    fn reflectance(cosine: Float, refraction_ratio: Float) -> Float {
        // NOTE(srp): Aproximación de Schlick
        let r0 = (1.0-refraction_ratio) / (1.0+refraction_ratio);
        let r0 = r0 * r0;
        r0 + (1.0 - r0)*(1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord, rng: &mut RngGen) -> Scatter {
        let new_color = Color::one();
        let refraction_ratio = if hit.front_face {
            self.ior.recip()
        } else {
            self.ior
        };

        let ref unit_direction: Vec3 = r_in.direction().unit_vector();
        let cos_theta = Vec3::dot(&-unit_direction, hit.normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;
        let reflect: bool = Self::reflectance(cos_theta, refraction_ratio) > rng.get();

        let direction: Vec3 = if cannot_refract || reflect {
            unit_direction.reflect(hit.normal())
        } else {
            unit_direction.refract(hit.normal(), refraction_ratio)
        };

        let new_ray = Ray::new(hit.p(), &direction, r_in.time());
        Scatter::Some(new_ray, new_color)
    }
}
