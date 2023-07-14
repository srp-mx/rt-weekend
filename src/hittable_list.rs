use std::rc::Rc;
use super::float::*;
use super::ray::*;
use super::hittable::*;

// NOTE(srp): Podríamos querer cambiar Rc por std::sync::Arc
pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: &Rc<dyn Hittable>) {
        self.objects.push(object.clone());
    }
}

impl Hittable for HittableList {
    fn hit(&self, r:&Ray, t_min:Float, t_max:Float) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit: Option<HitRecord> = None;

        for object in self.objects.iter() {
            match object.hit(r, t_min, t_max) {
                Some(object_hit) => if object_hit.t < closest_so_far {
                        closest_so_far = object_hit.t;
                        hit = Some(object_hit);
                },
                None => continue
            }
        }

        hit
    }
}
