use super::float::*;
use super::vec3::Vec3;
type Point3 = Vec3;
use super::ray::Ray;
use super::rng_float::RngGen;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: Float,
    shutter_open_time: Float,
    shutter_close_time: Float,
}

pub struct CameraBuilder {
    lookfrom: Point3,
    lookat: Point3,
    view_up: Vec3,
    vertical_fov: Float,
    aspect_ratio: Float,
    aperture: Float,
    focus_dist: Float,
    shutter_open_time: Float,
    shutter_close_time: Float,
}

impl CameraBuilder {
    pub fn new() -> Self {
        let lookfrom = Point3::new(3.0, 3.0, 2.0);
        let lookat = Point3::new(0.0, 0.0, -1.0);
        let view_up = Vec3::new(0.0, 1.0, 0.0);
        let vertical_fov: Float = 20.0;
        let aspect_ratio: Float = 16.0 / 9.0;
        let aperture: Float = 2.0;
        let focus_dist = (&lookfrom - &lookat).length();
        let shutter_open_time = 0.0;
        let shutter_close_time = 0.0;

        CameraBuilder {
            lookfrom, lookat, view_up, vertical_fov, aspect_ratio,
            aperture, focus_dist, shutter_open_time, shutter_close_time
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

    pub fn aperture(&mut self, aperture: Float) -> &mut Self {
        self.aperture = aperture;
        self
    }

    pub fn focus_dist(&mut self, focus_dist: Float) -> &mut Self {
        self.focus_dist = focus_dist;
        self
    }

    pub fn shutter_open_time(&mut self, shutter_open_time: Float) -> &mut Self {
        self.shutter_open_time = shutter_open_time;
        self
    }

    pub fn shutter_close_time(&mut self, shutter_close_time: Float) -> &mut Self {
        self.shutter_close_time = shutter_close_time;
        self
    }

    pub fn build(&self) -> Camera {
        Camera::new(self)
    }
}

impl Camera {
    fn new(data: &CameraBuilder) -> Self {
        let theta = data.vertical_fov.to_radians();
        let h = (theta * 0.5).tan();
        let viewport_height: Float = 2.0 * h;
        let viewport_width: Float = data.aspect_ratio * viewport_height;
        
        let w = (&data.lookfrom - &data.lookat).unit_vector();
        let u = Vec3::cross(&data.view_up, &w).unit_vector();
        let v = Vec3::cross(&w, &u);

        let origin = data.lookfrom.copy();
        let horizontal = &data.focus_dist * viewport_width * &u;
        let vertical = &data.focus_dist * viewport_height * &v;
        let lower_left_corner = &origin - &horizontal/2.0 - &vertical/2.0 - data.focus_dist*&w;
        let lens_radius = data.aperture / 2.0;
        let shutter_open_time = data.shutter_open_time;
        let shutter_close_time = data.shutter_close_time;

        Camera {
            origin, lower_left_corner, horizontal, vertical, u, v, w,
            lens_radius, shutter_open_time, shutter_close_time
        }
    }

    fn get_ray_dir(&self, s: Float, t: Float, offset: &Vec3) -> Vec3 {
        &self.lower_left_corner + s*&self.horizontal + t*&self.vertical - &self.origin - offset
    }

    pub fn get_ray(&self, s: Float, t: Float, rng: &mut RngGen) -> Ray {
        let rd: Vec3 = self.lens_radius * Vec3::random_unit_xy(rng);
        let offset: Vec3 = &self.u*rd.x() + &self.v*rd.y();
        Ray::new(
            &(&self.origin + &offset),
            &self.get_ray_dir(s, t, &offset),
            rng.range(self.shutter_open_time, self.shutter_close_time))
    }
}
