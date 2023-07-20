use super::float::Float;
use super::rng_float::RngGen;
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

    fn hit(&self, r:&Ray, t_min:Float, t_max:Float, _rng: &mut RngGen) -> Option<HitRecord> {
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
        let outward_normal = Vec3::fwd();
        Some(HitRecord::new(r, outward_normal, self.mp.clone(), t, rec_u, rec_v))
    }
}


pub struct YzRect {
    mp: Rc<dyn Material>,
    y0: Float,
    y1: Float,
    z0: Float,
    z1: Float,
    k: Float,
}

impl YzRect {
    pub fn new(y0: Float, y1: Float,
               z0: Float, z1: Float,
               k: Float, mat: Rc<dyn Material>) -> Self {
        Self { y0, y1, z0, z1, k, mp: mat }
    }
}

impl Hittable for YzRect {
    fn bounding_box(&self, _time0: Float, _time1: Float) -> Option<AABB> {
        let eps = 0.0001;
        Some(AABB::new(
                Point3::new(self.k-eps, self.y0, self.z0),
                Point3::new(self.k+eps, self.y1, self.z1)))
    }

    fn hit(&self, r:&Ray, t_min:Float, t_max:Float, _rng: &mut RngGen) -> Option<HitRecord> {
        let t = (self.k-r.origin().x()) / r.direction().x();
        if t < t_min || t > t_max {
            return None
        }
        let y = r.origin().y() + t*r.direction().y();
        let z = r.origin().z() + t*r.direction().z();
        if z < self.z0 || z > self.z1 || y < self.y0 || y > self.y1 {
            return None
        }
        let rec_u = (y-self.y0)/(self.y1-self.y0);
        let rec_v = (z-self.z0)/(self.z1-self.z0);
        let outward_normal = Vec3::right();
        Some(HitRecord::new(r, outward_normal, self.mp.clone(), t, rec_u, rec_v))
    }
}


pub struct ZxRect {
    mp: Rc<dyn Material>,
    z0: Float,
    z1: Float,
    x0: Float,
    x1: Float,
    k: Float,
}

impl ZxRect {
    pub fn new(z0: Float, z1: Float,
               x0: Float, x1: Float,
               k: Float, mat: Rc<dyn Material>) -> Self {
        Self { x0, x1, z0, z1, k, mp: mat }
    }
}

impl Hittable for ZxRect {
    fn bounding_box(&self, _time0: Float, _time1: Float) -> Option<AABB> {
        let eps = 0.0001;
        Some(AABB::new(
                Point3::new(self.x0, self.k-eps, self.z0),
                Point3::new(self.x1, self.k+eps, self.z1)))
    }

    fn hit(&self, r:&Ray, t_min:Float, t_max:Float, _rng: &mut RngGen) -> Option<HitRecord> {
        let t = (self.k-r.origin().y()) / r.direction().y();
        if t < t_min || t > t_max {
            return None
        }
        let x = r.origin().x() + t*r.direction().x();
        let z = r.origin().z() + t*r.direction().z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None
        }
        let rec_u = (x-self.x0)/(self.x1-self.x0);
        let rec_v = (z-self.z0)/(self.z1-self.z0);
        let outward_normal = Vec3::up();
        Some(HitRecord::new(r, outward_normal, self.mp.clone(), t, rec_u, rec_v))
    }
}
