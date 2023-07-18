use super::float::Float;
use super::rng_float::RngGen;
use super::color::Color;
use super::ray::Ray;
use super::pixel_buffer::PixelBuffer;
use super::hittable::Hittable;
use super::hittable_list::HittableList;
use super::camera::{CameraBuilder, Camera};
use super::material::Scatter;

use std::sync::Arc;
use std::rc::Rc;
use std::sync::RwLock;

pub struct RenderInfo {
    camera: Camera,
    sky: Sky,
    world: Rc::<HittableList>,
    pixel_buffer_rwlock: Arc<RwLock<PixelBuffer>>, 
    image_width: usize,
    image_height: usize,
}

// Gradient default
// let c1 = Color::one();
// let c2 = Color::new(0.5, 0.7, 1.0);
pub enum Sky {
    Gradient(Color, Color),
    SolidColor(Color),
}

impl RenderInfo {
    pub fn new(pixel_buffer_rwlock: Arc<RwLock<PixelBuffer>>,
               cam_settings: &mut CameraBuilder) -> Self {
        let info_buffer = pixel_buffer_rwlock.read().unwrap();
        let image_width = info_buffer.width();
        let image_height = info_buffer.height();
        let aspect_ratio = info_buffer.aspect_ratio();
        std::mem::drop(info_buffer);
        Self {
            sky: Sky::Gradient(Color::one(), Color::new(0.5, 0.7, 1.0)),
            world: Rc::new(HittableList::new()),
            pixel_buffer_rwlock,
            image_width,
            image_height,
            camera: cam_settings.aspect_ratio(aspect_ratio).build()
        }
    }

    pub fn sky(&mut self, sky: Sky) -> &mut Self {
        self.sky = sky;
        self
    }

    pub fn world(&mut self, world: Rc<HittableList>) -> &mut Self {
        self.world = world;
        self
    }

    pub fn render(&mut self, samples_per_pixel: i32, max_bounces: i32, rng: &mut RngGen) {
        for j in (0..self.image_height).rev() {
            eprint!("\nScanlines remaining: {}\n", j+1);
            for i in 0..self.image_width {
                let mut pixel_color = Color::zero();
                for _ in 0..samples_per_pixel+1 {
                    let u = ((i as Float) + rng.get()) / ((self.image_width-1) as Float);
                    let v = ((j as Float) + rng.get()) / ((self.image_height-1) as Float);
                    let r: Ray = self.camera.get_ray(u, v, rng);
                    pixel_color += ray_color(&r, self, max_bounces+1, rng);
                }

                let mut buffer = loop {
                    if let Ok(buffer) = self.pixel_buffer_rwlock.write() {
                        break buffer
                    }
                };
                buffer.set_pixel(i, self.image_height-j-1, &pixel_color, samples_per_pixel);
                std::mem::drop(buffer);
            }
        }
    }
}

fn ray_color(this: &Ray, info: &RenderInfo, bounces_left: i32, rng: &mut RngGen)
        -> Color {
    if bounces_left <= 0 { return Color::zero() }

    if let Some(hit) = info.world.hit(this, 0.001, Float::INFINITY) {
        return if let Scatter::Some(scatter_ray, scatter_color) = hit.mat().scatter(this, &hit, rng) {
            scatter_color * ray_color(&scatter_ray, info, bounces_left-1, rng)
        } else {
            Color::zero()
        }
    }

    match &info.sky {
        Sky::Gradient(c1, c2) => {
            let unit_direction = this.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            Color::lerp(&c1, &c2, t)
        },
        Sky::SolidColor(c) => c.copy()
    }
}
