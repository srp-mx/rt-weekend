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

    fn get_uv(p: &Point3, out_u: &mut Float, out_v: &mut Float) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + std::f64::consts::PI;

        *out_u = phi * std::f64::consts::FRAC_1_PI * 0.5;
        *out_v = theta * std::f64::consts::FRAC_1_PI;
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

        let rec_t = root;
        let hit_p = r.at(rec_t);
        let outward_normal: Vec3 = (&hit_p - self.center()) / self.radius();
        let rec_mat = self.mat.clone();
        let mut u: Float = 0.0;
        let mut v: Float = 0.0;
        Sphere::get_uv(&outward_normal, &mut u, &mut v);
        let rec = HitRecord::new(&r, outward_normal, rec_mat, rec_t, u, v);
        Some(rec)
    }

    fn bounding_box(&self, _time0: Float, _time1: Float) -> Option<AABB> {
        let ref radius_vec = Vec3::new(self.radius, self.radius, self.radius);
        Some(AABB::new(self.center() - radius_vec, self.center() + radius_vec))
    }
}
