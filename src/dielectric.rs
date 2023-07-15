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

    // NOTE(srp): eta_ratio := $\frac{\eta_i}{\eta_t}$
    fn refract(uv: &Vec3, n: &Vec3, eta_ratio: Float) -> Vec3 {
        let cos_theta = Vec3::dot(&-uv, n).min(1.0);
        let r_out_perp = eta_ratio * (uv + cos_theta*n);
        let r_out_parallel = -n * (1.0 - r_out_perp.length_squared()).abs().sqrt();
        r_out_perp + r_out_parallel
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

        let unit_direction: Vec3 = r_in.direction().unit_vector();
        let refracted: Vec3 = Self::refract(&unit_direction, hit.normal(), refraction_ratio);
        let new_ray = Ray::new(hit.p(), &refracted);

        Scatter::Some(new_ray, new_color)
    }
}
