use super::rng_float::RngGen;
use super::vec3::Vec3;
use super::color::Color;
use super::ray::Ray;
use super::hittable::HitRecord;
use super::texture::{Texture, SolidColor};
use super::material::{Material, Scatter};

use std::rc::Rc;

pub struct Isotropic {
    albedo: Rc<dyn Texture>
}

impl Isotropic {
    pub fn new(albedo: Rc<dyn Texture>) -> Self {
        Self { albedo }
    }

    pub fn new_from_color(c: Color) -> Self {
        Self { albedo: Rc::new(SolidColor::new(c)) }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord, rng: &mut RngGen) -> Scatter {
        let scattered = Ray::new(hit.p(), &Vec3::random_sphere(rng), r_in.time());
        let attenuation = self.albedo.value(hit.u(), hit.v(), hit.p());
        Scatter::Some(scattered, attenuation)
    }
}
