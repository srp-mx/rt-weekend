use super::float::*;
use super::vec3::Vec3;
type Point3 = Vec3;
use super::ray::Ray;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio: Float = 16.0 / 9.0;
        let viewport_height: Float = 2.0;
        let viewport_width: Float = aspect_ratio * viewport_height;
        let focal_length: Float = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let focal = Vec3::new(0.0, 0.0, focal_length);
        let lower_left_corner = &origin - &horizontal/2.0 - &vertical/2.0 - focal;

        Camera { origin, horizontal, vertical, lower_left_corner }
    }

    fn get_ray_dir(&self, u: Float, v: Float) -> Vec3 {
        &self.lower_left_corner + u*&self.horizontal + v*&self.vertical - &self.origin
    }

    pub fn get_ray(&self, u: Float, v: Float) -> Ray {
        Ray::new(&self.origin, &self.get_ray_dir(u, v))
    }
}
