use std::rc::Rc;
use super::float::*;
use super::vec3::Vec3;
type Point3 = Vec3;
use super::hittable::*;
use super::material::Material;
use super::aabb::AABB;

pub struct Sphere {
    center: Point3,
    radius: Float,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: Float, mat: Rc<dyn Material>) -> Sphere {
        Sphere { center, radius, mat }
    }

    pub fn center(&self) -> &Point3 {
        &self.center
    }

    pub fn radius(&self) -> Float {
        self.radius
    }

    pub fn mat(&self) -> &dyn Material {
        self.mat.as_ref()
    }
}

impl Hittable for Sphere {
    fn hit(&self, r:&crate::ray::Ray, t_min:Float, t_max:Float) -> Option<HitRecord> {
        let ref oc: Vec3 = r.origin() - self.center();
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
        let outward_normal: Vec3 = (rec.p() - self.center()) / self.radius();
        rec.set_face_normal(r, outward_normal);
        rec.set_mat(self.mat.clone());
        Some(rec)
    }

    fn bounding_box(&self, _time0: Float, _time1: Float) -> Option<AABB> {
        let ref radius_vec = Vec3::new(self.radius, self.radius, self.radius);
        Some(AABB::new(self.center() - radius_vec, self.center() + radius_vec))
    }
}
