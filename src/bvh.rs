use super::float::Float;
use super::ray::Ray;
use super::hittable_list::HittableList;
use super::hittable::{Hittable, HitRecord};
use super::aabb::AABB;
use super::axis::Axis;
use super::rng_float::RngGen;

use std::cmp::Ordering;
use std::rc::Rc;

pub struct BVH {
    root: Node
}

struct Node {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    aabb: AABB
}

impl BVH {
    pub fn new(list: &mut HittableList, time0: Float, time1: Float, rng: &mut RngGen) -> Self {
        Self {
            root: Node::new(list.objects_mut(), time0, time1, rng)
        }
    }
}

impl Hittable for BVH {
    fn bounding_box(&self, time0: Float, time1: Float) -> Option<AABB> {
        self.root.bounding_box(time0, time1)
    }

    fn hit(&self, r:&Ray, t_min:Float, t_max:Float) -> Option<HitRecord> {
        self.root.hit(r, t_min, t_max)
    }
}

impl Node {
    fn new(src_objects: &mut[Rc<dyn Hittable>],
           time0: Float, time1: Float, rng: &mut RngGen) -> Self {
        let left: Rc<dyn Hittable>;
        let right: Rc<dyn Hittable>;

        let axis = Axis::rand(rng);

        let cmp = |a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>| -> Ordering {
            Self::box_cmp(a, b, &axis, time0, time1)
        };


        match src_objects.len() {
            1 => {
                left = src_objects[0].clone();
                right = left.clone();
            },
            2 => {
                if let Ordering::Less = cmp(&src_objects[0], &src_objects[1]) {
                    left = src_objects[0].clone();
                    right = src_objects[1].clone();
                } else {
                    left = src_objects[1].clone();
                    right = src_objects[0].clone();
                }
            },
            _ => {
                src_objects.sort_by(cmp);
                let (half1, half2) = src_objects.split_at_mut(src_objects.len()/2);
                left = Rc::new(Node::new(half1, time0, time1, rng));
                right = Rc::new(Node::new(half2, time0, time1, rng));
            }
        }

        let box_left = left.bounding_box(time0, time1);
        let box_right = right.bounding_box(time0, time1);

        // NOTE(srp): If we add an infinite plane or something similar that
        // doesn't have a bounding box, we should handle that case here and
        // update the interface for Hittable::bounding_box(Float, Float).
        // In particular the return enum, to deal with such cases, given that
        // it is a special case.
        // Thus, a Hittable with no AABB is currently NOT supported.

        let err = "\n\nNo bounding box in BVH Node constructor.\n\n";
        let aabb = AABB::joint_box(&box_left.expect(err), &box_right.expect(err));

        Self { left, right, aabb }
    }

    fn box_cmp(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis: &Axis,
               time0: Float, time1: Float) -> Ordering {
        let box_a = a.bounding_box(time0, time1);
        let box_b = b.bounding_box(time0, time1);

        // NOTE(srp): See note above about Hittable without AABB support.
        let err = "\n\nNo bounding box in BVH Node constructor.\n\n";
        
        let axis_a = box_a.expect(err).min().axis(axis);
        let axis_b = box_b.expect(err).min().axis(axis);

        if axis_a < axis_b {
            Ordering::Less
        } else if axis_a > axis_b {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl Hittable for Node {
    fn hit(&self, r:&Ray, t_min:Float, t_max:Float) -> Option<HitRecord> {
        if !self.aabb.hit(r, t_min, t_max) { return None }

        let hit_left = self.left.hit(r, t_min, t_max);
        let new_t_max = if let Some(left) = &hit_left { left.t() } else { t_max };
        let hit_right = self.right.hit(r, t_min, new_t_max);

        if let None = hit_right { hit_left } else { hit_right }
    }

    fn bounding_box(&self, _time0: Float, _time1: Float) -> Option<AABB> {
        Some(self.aabb.copy())
    }
}
