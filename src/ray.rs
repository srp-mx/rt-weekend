use super::vec3::Vec3;
use super::float::Float;
type Point3 = Vec3;

pub struct Ray {
    orig: Point3,
    dir: Vec3,
    time: Float,
}

impl Ray {
    pub fn new(origin: &Point3, direction: &Vec3, time: Float) -> Self {
        let orig = origin.copy();
        let dir = direction.copy();
        Ray { orig, dir, time }
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: Float) -> Point3 {
        &self.orig + &(t*&self.dir)
    }

    pub fn time(&self) -> Float {
        self.time
    }
}
