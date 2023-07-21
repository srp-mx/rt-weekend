use super::rng_float::RngGen;
use super::material::{Material, Scatter};
use super::ray::Ray;
use super::hittable::HitRecord;
use super::vec3::Vec3;
use super::color::Color;
use super::texture::{Texture, SolidColor};

use std::rc::Rc;

pub struct Lambertian {
    albedo: Rc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Rc<dyn Texture>) -> Self {
        Self { albedo }
    }

    pub fn new_from_color(c: Color) -> Self {
        Self::new(Rc::new(SolidColor::new(c)))
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord, rng: &mut RngGen) -> Scatter {
        let mut dir = hit.normal() + Vec3::random_unit(rng);

        if dir.near_zero() {
            dir = hit.normal().copy();
        }

        let new_ray = Ray::new(hit.p(), &dir, r_in.time());
        let new_color = self.albedo.value(hit.u(), hit.v(), hit.p());
        return Scatter::Some(new_ray, new_color)
    }
}
