use std::rc::Rc;
use super::float::*;
use super::ray::*;
use super::hittable::*;
use super::aabb::AABB;

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

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn objects(&self) -> &Vec<Rc<dyn Hittable>> {
        &self.objects
    }

    pub fn objects_mut(&mut self) -> &mut Vec<Rc<dyn Hittable>> {
        &mut self.objects
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

    fn bounding_box(&self, time0: Float, time1: Float) -> Option<AABB> {
        let mut output: Option<AABB> = None;
        let mut is_first_box = true;

        for object in &self.objects {
            match object.bounding_box(time0, time1) {
                None => return None,
                Some(current_box) => {
                    if is_first_box {
                        output = Some(current_box);
                        is_first_box = false;
                        continue;
                    }
                    if let Some(old_box) = output {
                        output = Some(AABB::joint_box(&current_box, &old_box));
                    }
                }
            }
        }

        output
    }
}
