use super::float::Float;
use super::vec3::Vec3;
type Point3 = Vec3;
use super::color::Color;
use super::ray::Ray;
use super::hittable::HitRecord;
use super::rng_float::RngGen;
use super::texture::{Texture, SolidColor};
use super::material::{Material, Scatter};

use std::rc::Rc;

pub struct DiffuseLight {
    emit: Rc<dyn Texture>
}

impl DiffuseLight {
    pub fn new(emit: Rc<dyn Texture>) -> Self {
        Self { emit }
    }

    pub fn new_from_color(c: &Color) -> Self {
        Self { emit: Rc::new(SolidColor::new(c.copy())) }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _hit: &HitRecord, _rng: &mut RngGen) -> Scatter {
        Scatter::None
    }

    fn emmited(&self, u: Float, v: Float, p: &Point3) -> Color {
        self.emit.value(u, v, p)
    }
}
