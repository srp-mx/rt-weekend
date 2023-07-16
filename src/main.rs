pub mod float;
pub mod vec3;
pub mod color;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod hittable_list;
pub mod rng_float;
pub mod camera;
pub mod material;
pub mod lambertian;
pub mod metal;
pub mod dielectric;
pub mod pixel_buffer;

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
use camera::{Camera, CameraBuilder};
use material::Scatter;
use metal::Metal;
use lambertian::Lambertian;
use dielectric::Dielectric;
use material::Material;
use pixel_buffer::PixelBuffer;

use minifb::{Window, WindowOptions};
use std::sync::RwLock;
use std::sync::Arc;
use std::thread::JoinHandle;

fn main() {
    // RNG
    let mut rng = RngGen::new();

    // Image
    const ASPECT_RATIO:Float = 3.0 / 2.0;
    const IMAGE_WIDTH:usize = 300;
    const IMAGE_HEIGHT:usize = ((IMAGE_WIDTH as Float) / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL:i32 = 16;
    const MAX_DEPTH: i32 = 8;

    // Pixel Buffer
    let buffer_lock = Arc::new(RwLock::new(PixelBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT)));

    // World
    let world = random_scene(&mut rng);

    // Camera
    let cam: Camera = CameraBuilder::new()
        .lookfrom(Vec3::new(12.0, 2.0, 3.0))
        .vertical_fov(20.0)
        .focus_dist(10.0)
        .aperture(0.1)
        .aspect_ratio(ASPECT_RATIO)
        .build();

    // Fast Render Pass for Preview
    eprintln!("Making a fast render pass for preview");
    let mut frpp_buffer = buffer_lock.write().unwrap();
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining on preview: {}", j+1);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::zero();
            let u = ((i as Float) + rng.get()) / ((IMAGE_WIDTH-1) as Float);
            let v = ((j as Float) + rng.get()) / ((IMAGE_HEIGHT-1) as Float);
            let r: Ray = cam.get_ray(u, v, &mut rng);
            pixel_color += r.color(&world, 2, &mut rng);
            frpp_buffer.set_pixel(i, IMAGE_HEIGHT-j-1, &pixel_color, 1);
        }
    }
    std::mem::drop(frpp_buffer);

    // Preview Window
    let preview_thread = make_preview_window(buffer_lock.clone());

    // Render
    eprintln!("\nGetting serious now >:)\n");
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\nScanlines remaining: {}\n", j+1);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::zero();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = ((i as Float) + rng.get()) / ((IMAGE_WIDTH-1) as Float);
                let v = ((j as Float) + rng.get()) / ((IMAGE_HEIGHT-1) as Float);
                let r: Ray = cam.get_ray(u, v, &mut rng);
                pixel_color += r.color(&world, MAX_DEPTH, &mut rng);
            }

            let mut buffer = loop {
                if let Ok(buffer) = buffer_lock.write() {
                    break buffer
                }
            };
            buffer.set_pixel(i, IMAGE_HEIGHT-j-1, &pixel_color, SAMPLES_PER_PIXEL);
            std::mem::drop(buffer);
        }
    }

    eprint!("\nWriting output.\n");
    let ppm_output = buffer_lock.read().unwrap().to_ppm();
    print!("{ppm_output}");
    eprint!("\nDone.\n");

    preview_thread.join().unwrap();
}

impl Ray {
    pub fn color(&self, world:&dyn Hittable, depth: i32, rng:&mut RngGen) -> Color {
        if depth <= 0 {
            return Color::zero();
        }

        if let Some(hit) = world.hit(self, 0.001, Float::INFINITY) {
            return if let Scatter::Some(scatter_ray, scatter_color) = hit.mat().scatter(&self, &hit, rng) {
                scatter_color * Ray::color(&scatter_ray, world, depth-1, rng)
            } else {
                Color::zero()
            }
        }

        let unit_direction: Vec3 = self.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        let c1 = Color::one();
        let c2 = Color::new(0.5, 0.7, 1.0);
        Vec3::lerp(&c1, &c2, t)
    }
}

fn random_scene(rng: &mut RngGen) -> HittableList {
    let mut world = HittableList::new();

    let ground_mat = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground: Rc<dyn Hittable> = Rc::new(Sphere::new(Point3::new(0.0,-1000.0,0.0), 1000.0, ground_mat));
    world.add(ground);

    for a in -11..12 {
        for b in -11..12 {
            let choose_mat = rng.get();
            let center = Point3::new(a as Float + 0.9*rng.get(), 0.2, b as Float + 0.9*rng.get());
            
            if (&center - &Point3::new(4.0, 0.2, 0.0)).length() <= 0.9 {
                continue;
            }

            let sphere_mat: Rc<dyn Material> = if choose_mat < 0.8 {
                Rc::new(Lambertian::new(Color::random(rng) * Color::random(rng)))
            } else if choose_mat < 0.95 {
                Rc::new(Metal::new(Color::random_range(rng, 0.5, 1.0), rng.range(0.0, 0.5)))
            } else {
                Rc::new(Dielectric::new(1.5))
            };

            world.add(Rc::new(Sphere::new(center, 0.2, sphere_mat)));
        }
    }

    let mat1: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));
    let sph1: Rc<dyn Hittable> = Rc::new(Sphere::new(Point3::new(0.0,1.0,0.0), 1.0, mat1));
    world.add(sph1);

    let mat2: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let sph2: Rc<dyn Hittable> = Rc::new(Sphere::new(Point3::new(-4.0,1.0,0.0), 1.0, mat2));
    world.add(sph2);

    let mat3: Rc<dyn Material> = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    let sph3: Rc<dyn Hittable> = Rc::new(Sphere::new(Point3::new(4.0,1.0,0.0), 1.0, mat3));
    world.add(sph3);

    world
}

fn make_preview_window(buffer_lock: Arc<RwLock<PixelBuffer>>) -> JoinHandle<()> {
    std::thread::spawn(move || {
        let buffer = loop {
            if let Ok(buffer) = buffer_lock.read() {
                break buffer
            }
        };
        let mut window = Window::new(
            "rt-weekend",
            buffer.width(),
            buffer.height(),
            WindowOptions::default(),
        ).unwrap_or_else(|e| {
            panic!("{}", e);
        });
        std::mem::drop(buffer);

        while window.is_open() {
            let buffer = loop {
                if let Ok(buffer) = buffer_lock.read() {
                    break buffer
                }
            };
            window.update_with_buffer(buffer.buffer(), buffer.width(), buffer.height()).unwrap();
            std::mem::drop(buffer);
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    })
}
