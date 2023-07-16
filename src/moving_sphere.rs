use super::float::Float;
use super::vec3::Vec3;
type Point3 = Vec3;
use super::material::Material;
use super::hittable::{Hittable, HitRecord};
use super::ray::Ray;
use super::aabb::AABB;

use std::rc::Rc;

pub struct MovingSphere {
    center0: Point3,
    center1: Point3,
    time0: Float,
    time1: Float,
    radius: Float,
    mat: Rc<dyn Material>,
}

impl MovingSphere {
    pub fn new(c0: Point3, c1: Point3, t0: Float, t1: Float, r: Float, m: Rc<dyn Material>) -> Self {
        Self {
            center0: c0,
            center1: c1,
            time0: t0,
            time1: t1,
            radius: r,
            mat: m,
        }
    }

    pub fn center(&self, time: Float) -> Point3 {
        &self.center0 + ((time - &self.time0) / (&self.time1 - &self.time0))*(&self.center1 - &self.center0)
    }

    pub fn radius(&self) -> Float {
        self.radius
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r:&Ray, t_min:Float, t_max:Float) -> Option<HitRecord> {
        let ref oc: Vec3 = r.origin() - self.center(r.time());
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(oc, r.direction());
        let c = oc.length_squared() - self.radius()*self.radius();

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find nearest root within range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut rec = HitRecord::null();
        rec.t = root;
        rec.set_p(r.at(rec.t));
        let outward_normal: Vec3 = (rec.p() - self.center(r.time())) / self.radius();
        rec.set_face_normal(r, outward_normal);
        rec.set_mat(self.mat.clone());
        Some(rec)
    }

    fn bounding_box(&self, time0: Float, time1: Float) -> Option<AABB> {
        let ref radius_vec = Vec3::new(self.radius, self.radius, self.radius);

        let ref center0 = self.center(time0);
        let box0 = AABB::new(center0 - radius_vec, center0 + radius_vec);
        let ref center1 = self.center(time1);
        let box1 = AABB::new(center1 - radius_vec, center1 + radius_vec);

        Some(AABB::joint_box(&box0, &box1))
    }
}
