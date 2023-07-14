use super::float::*;
use super::vec3::Vec3;
type Point3 = Vec3;
use super::ray::Ray;

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    pub t: Float,
    pub front_face: bool,
}

impl HitRecord {
    pub fn null() -> Self {
        Self {
            p: Point3::zero(),
            normal: Vec3::zero(),
            t: -1.0,
            front_face: false,
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

    pub fn t(&self) -> Float {
        self.t
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(r.direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }

}

pub trait Hittable {
    fn hit(&self, r:&Ray, t_min:Float, t_max:Float) -> Option<HitRecord>;
}
