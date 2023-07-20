use super::float::Float;
use super::rng_float::RngGen;
use super::vec3::Vec3;
type Point3 = Vec3;
use super::hittable::{Hittable, HitRecord};
use super::hittable_list::HittableList;
use super::material::Material;
use super::aarect::{XyRect, YzRect, ZxRect};
use super::aabb::AABB;
use super::ray::Ray;

use std::rc::Rc;

pub struct RectPrism {
    box_min: Point3,
    box_max: Point3,
    sides: HittableList
}

impl RectPrism {
    pub fn new(p0: &Point3, p1: &Point3, mat: Rc<dyn Material>) -> Self {
        let box_min = p0.copy();
        let box_max = p1.copy();
        let mut sides = HittableList::new();
        sides.add(Rc::new(XyRect::new(p0.x(), p1.x(), p0.y(), p1.y(), p1.z(), mat.clone())));
        sides.add(Rc::new(XyRect::new(p0.x(), p1.x(), p0.y(), p1.y(), p0.z(), mat.clone())));
        sides.add(Rc::new(ZxRect::new(p0.z(), p1.z(), p0.x(), p1.x(), p1.y(), mat.clone())));
        sides.add(Rc::new(ZxRect::new(p0.z(), p1.z(), p0.x(), p1.x(), p0.y(), mat.clone())));
        sides.add(Rc::new(YzRect::new(p0.y(), p1.y(), p0.z(), p1.z(), p1.x(), mat.clone())));
        sides.add(Rc::new(YzRect::new(p0.y(), p1.y(), p0.z(), p1.z(), p0.x(), mat.clone())));
        Self { box_min, box_max, sides }
    }
}

impl Hittable for RectPrism {
    fn bounding_box(&self, _time0: Float, _time1: Float) -> Option<AABB> {
        Some(AABB::new(self.box_min.copy(), self.box_max.copy()))
    }

    fn hit(&self, r:&Ray, t_min:Float, t_max:Float, rng: &mut RngGen) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max, rng)
    }
}
