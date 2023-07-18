use std::rc::Rc; // NOTE(srp): Considera cambiar por std::sync::Arc
use super::float::*;
use super::vec3::Vec3;
type Point3 = Vec3;
use super::ray::Ray;
use super::material::{Material, NullMaterial};
use super::aabb::AABB;

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    mat: Rc<dyn Material>,
    t: Float,
    u: Float,
    v: Float,
    in_front_face: bool,
}

impl HitRecord {
    pub fn new(
            r: &Ray,
            outward_normal: Vec3,
            mat: Rc<dyn Material>,
            t: Float,
            u: Float,
            v: Float) -> Self {
        let mut normal: Vec3 = Vec3::zero();
        let mut in_front_face: bool = false;
        Self::front_face_and_normal(&r, outward_normal, &mut normal, &mut in_front_face);
        Self { p: r.at(t), normal, mat, t, u, v, in_front_face }
    }

    pub fn null() -> Self {
        Self {
            p: Point3::zero(),
            normal: Vec3::zero(),
            mat: Rc::new(NullMaterial),
            t: -1.0,
            u: -1.0,
            v: -1.0,
            in_front_face: false,
        }
    }

    pub fn p(&self) -> &Point3 {
        &(self.p)
    }

    pub fn set_p(&mut self, p: Point3) {
        self.p = p;
    }

    pub fn normal(&self) -> &Vec3 {
        &(self.normal)
    }

    pub fn mat(&self) -> &dyn Material {
        self.mat.as_ref()
    }

    pub fn t(&self) -> Float {
        self.t
    }

    pub fn in_front_face(&self) -> bool {
        self.in_front_face
    }

    pub fn u(&self) -> Float {
        self.u
    }

    pub fn v(&self) -> Float {
        self.v
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        Self::front_face_and_normal(r, outward_normal, &mut self.normal, &mut self.in_front_face);
    }

    fn front_face_and_normal(r: &Ray, outward_normal: Vec3,
                             out_normal: &mut Vec3,
                             out_in_front_face: &mut bool) {
        *out_in_front_face = Vec3::dot(r.direction(), &outward_normal) < 0.0;
        *out_normal = if *out_in_front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }

    pub fn set_mat(&mut self, mat: Rc<dyn Material>) {
        self.mat = mat;
    }
}

pub trait Hittable {
    fn hit(&self, r:&Ray, t_min:Float, t_max:Float) -> Option<HitRecord>;
    fn bounding_box(&self, time0: Float, time1: Float) -> Option<AABB>;
}
