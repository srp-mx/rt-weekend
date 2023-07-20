use super::float::Float;
use super::rng_float::RngGen;
use super::vec3::Vec3;
use super::color::Color;
use super::ray::Ray;
use super::hittable::{Hittable, HitRecord};
use super::texture::Texture;
use super::material::Material;
use super::aabb::AABB;
use super::isotropic::Isotropic;

use std::rc::Rc;

pub struct ConvexConstantMedium {
    boundary: Rc<dyn Hittable>,
    phase_function: Rc<dyn Material>,
    neg_inv_density: Float
}

impl ConvexConstantMedium {
    pub fn new(boundary: Rc<dyn Hittable>, density: Float, a: Rc<dyn Texture>) -> Self {
        Self { boundary, phase_function: Rc::new(Isotropic::new(a)), neg_inv_density: -density.recip() }
    }

    pub fn new_from_color(boundary: Rc<dyn Hittable>, density: Float, c: Color) -> Self {
        Self { boundary, phase_function: Rc::new(Isotropic::new_from_color(c)), neg_inv_density: -density.recip() }
    }
}

impl Hittable for ConvexConstantMedium {
    fn bounding_box(&self, time0: Float, time1: Float) -> Option<AABB> {
        self.boundary.bounding_box(time0, time1)
    }

    fn hit(&self, r:&Ray, t_min:Float, t_max:Float, rng: &mut RngGen) -> Option<HitRecord> {
        // Print occasional samples when debugging. To enable, set enableDebug.
        const ENABLE_DEBUG: bool = false;
        let debugging = ENABLE_DEBUG && rng.get() < 0.00001;

        match self.boundary.hit(r, Float::NEG_INFINITY, Float::INFINITY, rng) {
            Some(mut hit1) => {
                match self.boundary.hit(r, hit1.t()+0.0001, Float::INFINITY, rng) {
                    Some(mut hit2) => {
                        if debugging { eprintln!("\nt_min={}, t_max={}", hit1.t(), hit2.t()); }
                        if hit1.t() < t_min { hit1.set_t(t_min); }
                        if hit2.t() > t_max { hit2.set_t(t_max); }
                        if hit1.t() >= hit2.t() { return None }
                        if hit1.t() < 0.0 { hit1.set_t(0.0); }
                        let ray_len = r.direction().length();
                        let dist_inside_boundary = (hit2.t() - hit1.t()) * ray_len;
                        let hit_dist = self.neg_inv_density * rng.get().ln();
                        if hit_dist > dist_inside_boundary { return None }
                        let rec_t = hit1.t() + hit_dist/ray_len;
                        let rec_p = r.at(rec_t);
                        if debugging {
                            eprintln!("hit_dist={}", hit_dist);
                            eprintln!("rec_t={}", rec_t);
                            eprintln!("rec_p={}", rec_p);
                        }
                        let rec_normal = Vec3::right();
                        let rec_front_face = true;
                        let rec_mat = self.phase_function.clone();
                        Some(HitRecord::from_settings(rec_p, rec_normal, rec_mat, rec_t, 0.0, 0.0, rec_front_face))
                    },
                    None => None
                }
            },
            None => None
        }
    }
}
