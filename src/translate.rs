use super::hittable::{Hittable, HitRecord};
use super::float::Float;
use super::rng_float::RngGen;
use super::vec3::Vec3;
use super::ray::Ray;
use super::aabb::AABB;

use std::rc::Rc;

pub struct Translate {
    source: Rc<dyn Hittable>,
    offset: Vec3
}

impl Translate {
    pub fn new(source: Rc<dyn Hittable>, offset: Vec3) -> Self {
        Self { source, offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, r:&Ray, t_min:Float, t_max:Float, rng: &mut RngGen) -> Option<HitRecord> {
        let offset_ray = Ray::new(&(r.origin() - &self.offset), r.direction(), r.time());
        match self.source.hit(&offset_ray, t_min, t_max, rng) {
            Some(mut hit) => {
                hit.set_p(hit.p() + &self.offset);
                hit.set_face_normal(&offset_ray, hit.normal().copy());
                Some(hit)
            },
            None => None
        }
    }

    fn bounding_box(&self, time0: Float, time1: Float) -> Option<AABB> {
        match self.source.bounding_box(time0, time1) {
            Some(input_box) => {
                Some(AABB::new(input_box.min() + &self.offset, input_box.max() + &self.offset))
            },
            None => None
        }
    }
}
