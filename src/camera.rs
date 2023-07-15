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

pub struct CameraBuilder {
    vertical_fov: Float,
    aspect_ratio: Float,
    lookfrom: Point3,
    lookat: Point3,
    view_up: Vec3,
}

impl CameraBuilder {
    pub fn new() -> Self {
        CameraBuilder {
            lookfrom: Point3::new(-2.0, 2.0, 1.0),
            lookat: Point3::new(0.0, 0.0, -1.0),
            view_up: Vec3::new(0.0, 1.0, 0.0),
            vertical_fov: 90.0,
            aspect_ratio: 16.0 / 9.0,
        }
    }

    pub fn lookfrom(&mut self, lookfrom: Point3) -> &mut Self {
        self.lookfrom = lookfrom;
        self
    }

    pub fn lookat(&mut self, lookat: Point3) -> &mut Self {
        self.lookat = lookat;
        self
    }

    pub fn view_up(&mut self, view_up: Point3) -> &mut Self {
        self.view_up = view_up;
        self
    }

    pub fn vertical_fov(&mut self, vertical_fov: Float) -> &mut Self {
        self.vertical_fov = vertical_fov;
        self
    }

    pub fn aspect_ratio(&mut self, aspect_ratio: Float) -> &mut Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn build(self) -> Camera {
        Camera::new(self)
    }
}

impl Camera {
    fn new(data: CameraBuilder) -> Self {
        let theta = data.vertical_fov.to_radians();
        let h = (theta * 0.5).tan();
        let viewport_height: Float = 2.0 * h;
        let viewport_width: Float = data.aspect_ratio * viewport_height;
        
        let w = (&data.lookfrom - data.lookat).unit_vector();
        let u = Vec3::cross(&data.view_up, &w).unit_vector();
        let v = Vec3::cross(&w, &u);

        let origin = data.lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = &origin - &horizontal/2.0 - &vertical/2.0 - w;

        Camera { origin, horizontal, vertical, lower_left_corner }
    }

    fn get_ray_dir(&self, s: Float, t: Float) -> Vec3 {
        &self.lower_left_corner + s*&self.horizontal + t*&self.vertical - &self.origin
    }

    pub fn get_ray(&self, s: Float, t: Float) -> Ray {
        Ray::new(&self.origin, &self.get_ray_dir(s, t))
    }
}
