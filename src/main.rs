pub mod float;
pub mod vec3;
pub mod color;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod hittable_list;
pub mod rng_float;
pub mod camera;

use std::rc::Rc;
use float::*;
use vec3::Vec3;
use color::Color;
type Point3 = Vec3;
use ray::Ray;
use hittable::*;
use sphere::Sphere;
use hittable_list::HittableList;
use rng_float::RngGen;
use camera::Camera;

fn main() {
    // RNG
    let mut rand01 = RngGen::new(0.0, 1.0);

    // Image
    const ASPECT_RATIO:Float = 16.0 / 9.0;
    const IMAGE_WIDTH:i32 = 400;
    const IMAGE_HEIGHT:i32 = ((IMAGE_WIDTH as Float) / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL:i32 = 100;

    // World
    let mut world = HittableList::new();
    let sphere1: Rc<dyn Hittable> = Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    let sphere2: Rc<dyn Hittable> = Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));
    world.add(&sphere1);
    world.add(&sphere2);

    // Camera
    let cam = Camera::new();

    // Render
    print!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\nScanlines remaining: {}\n", j+1);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::zero();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = ((i as Float) + rand01.get()) / ((IMAGE_WIDTH-1) as Float);
                let v = ((j as Float) + rand01.get()) / ((IMAGE_HEIGHT-1) as Float);
                let r: Ray = cam.get_ray(u, v);
                pixel_color += r.color(&world);
            }
            pixel_color.write_color(SAMPLES_PER_PIXEL);
        }
    }
    eprint!("\nDone.\n");
}

impl Ray {
    pub fn color(&self, world:&dyn Hittable) -> Color {
        if let Some(hit) = world.hit(self, 0.0, Float::INFINITY) {
            return 0.5 * (hit.normal() + Color::one());
        }

        let unit_direction: Vec3 = self.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        let c1 = Color::one();
        let c2 = Color::new(0.5, 0.7, 1.0);
        Vec3::lerp(&c1, &c2, t)
    }
}
