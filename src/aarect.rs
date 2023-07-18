use super::float::Float;
use super::vec3::Vec3;
type Point3 = Vec3;
use super::ray::Ray;
use super::hittable::{Hittable, HitRecord};
use super::material::Material;
use super::aabb::AABB;

use std::rc::Rc;

pub struct XyRect {
    mp: Rc<dyn Material>,
    x0: Float,
    x1: Float,
    y0: Float,
    y1: Float,
    k: Float,
}

impl XyRect {
    pub fn new(x0: Float, x1: Float,
               y0: Float, y1: Float,
               k: Float, mat: Rc<dyn Material>) -> Self {
        Self { x0, x1, y0, y1, k, mp: mat }
    }
}

impl Hittable for XyRect {
    fn bounding_box(&self, _time0: Float, _time1: Float) -> Option<AABB> {
        let eps = 0.0001;
        Some(AABB::new(
                Point3::new(self.x0, self.y0, self.k-eps),
                Point3::new(self.x1, self.y1, self.k+eps)))
    }

    fn hit(&self, r:&Ray, t_min:Float, t_max:Float) -> Option<HitRecord> {
        let t = (self.k-r.origin().z()) / r.direction().z();
        if t < t_min || t > t_max {
            return None
        }
        let x = r.origin().x() + t*r.direction().x();
        let y = r.origin().y() + t*r.direction().y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None
        }
        let rec_u = (x-self.x0)/(self.x1-self.x0);
        let rec_v = (y-self.y0)/(self.y1-self.y0);
        let outward_normal = Vec3::zero();
        Some(HitRecord::new(r, outward_normal, self.mp.clone(), t, rec_u, rec_v))
    }
}
