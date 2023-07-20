use super::float::Float;
use super::rng_float::RngGen;
use super::vec3::Vec3;
type Point3 = Vec3;
use super::ray::Ray;
use super::hittable::{HitRecord, Hittable};
use super::aabb::AABB;

use std::rc::Rc;

pub struct RotateY {
    source: Rc<dyn Hittable>,
    sin_theta: Float,
    cos_theta: Float,
    box_opt: Option<AABB>
}

impl RotateY {
    pub fn new(source: Rc<dyn Hittable>, angle: Float) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let src_box_opt = source.bounding_box(0.0, 1.0); // NOTE(srp): disregards time

        let mut min = Point3::new(Float::INFINITY, Float::INFINITY, Float::INFINITY);
        let mut max = Point3::new(Float::NEG_INFINITY, Float::NEG_INFINITY, Float::NEG_INFINITY);

        let box_opt = if let Some(src_box) = src_box_opt {
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let i_f = i as Float;
                        let j_f = j as Float;
                        let k_f = k as Float;
                        let x = i_f*src_box.max().x() + (1.0-i_f)*src_box.min().x();
                        let y = j_f*src_box.max().y() + (1.0-j_f)*src_box.min().y();
                        let z = k_f*src_box.max().z() + (1.0-k_f)*src_box.min().z();
                        let newx = cos_theta*x + sin_theta*z;
                        let newz = -sin_theta*x + cos_theta*z;
                        let tester = Vec3::new(newx, y, newz);
                        min.set_to_min(&tester);
                        max.set_to_max(&tester);
                    }
                }
            }
            Some(AABB::new(min, max))
        } else {
            None
        };

        Self { source, sin_theta, cos_theta, box_opt }
    }
}

impl Hittable for RotateY {
    fn bounding_box(&self, _time0: Float, _time1: Float) -> Option<AABB> {
        if let Some(b) = &self.box_opt {
            Some(b.copy())
        } else {
            None
        }
    }

    fn hit(&self, r:&Ray, t_min:Float, t_max:Float, rng: &mut RngGen) -> Option<HitRecord> {
        let origin_x = self.cos_theta*r.origin().x() - self.sin_theta*r.origin().z();
        let origin_z = self.sin_theta*r.origin().x() + self.cos_theta*r.origin().z();
        let origin = Vec3::new(origin_x, r.origin().y(), origin_z);
        let direction_x = self.cos_theta*r.direction().x() - self.sin_theta*r.direction().z();
        let direction_z = self.sin_theta*r.direction().x() + self.cos_theta*r.direction().z();
        let direction = Vec3::new(direction_x, r.direction().y(), direction_z);

        let rotated_r = Ray::new(&origin, &direction, r.time());
        
        match self.source.hit(&rotated_r, t_min, t_max, rng) {
            Some(mut hit) => {
                let p_x = self.cos_theta*hit.p().x() + self.sin_theta*hit.p().z();
                let p_z = self.sin_theta*hit.p().x() + self.cos_theta*hit.p().z();
                let p = Vec3::new(p_x, hit.p().y(), p_z);
                let normal_x = self.cos_theta*hit.normal().x() + self.sin_theta*hit.normal().z();
                let normal_z = -self.sin_theta*hit.normal().x() + self.cos_theta*hit.normal().z();
                let normal = Vec3::new(normal_x, hit.normal().y(), normal_z);
                hit.set_p(p);
                hit.set_face_normal(&rotated_r, normal);
                Some(hit)
            },
            None => None
        }
    }
}
