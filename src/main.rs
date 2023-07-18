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
pub mod moving_sphere;
pub mod aabb;
pub mod bvh;
pub mod axis;
pub mod texture;
pub mod checker_texture;
pub mod default_scenes;
pub mod perlin;
pub mod noise_texture;

use float::*;
use vec3::Vec3;
use color::Color;
use ray::Ray;
use hittable::*;
use rng_float::RngGen;
use material::Scatter;
use pixel_buffer::PixelBuffer;
use default_scenes::{DefaultScene, select_default_scene, select_default_scene_cam};

use minifb::{Window, WindowOptions};
use std::sync::RwLock;
use std::sync::Arc;
use std::thread::JoinHandle;

fn main() {
    // RNG
    let mut rng = RngGen::new();

    // Image
    const ASPECT_RATIO:Float = 16.0 / 9.0;
    const IMAGE_WIDTH:usize = 300;
    const IMAGE_HEIGHT:usize = ((IMAGE_WIDTH as Float) / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL:i32 = 50;
    const MAX_DEPTH: i32 = 12;

    // Pixel Buffer
    let buffer_lock = Arc::new(RwLock::new(PixelBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT)));

    // Scene, World and Camera
    let ref scene = DefaultScene::PerlinSpheres;
    let world = select_default_scene(scene, &mut rng);
    let cam = select_default_scene_cam(scene, ASPECT_RATIO);

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
    let timer_start = std::time::Instant::now();
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
    let timer_duration = timer_start.elapsed(); 

    eprint!("\nWriting output.\n");
    let ppm_output = buffer_lock.read().unwrap().to_ppm();
    print!("{ppm_output}");
    eprint!("\nDone.\nRendering took {:?}\n", timer_duration);

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

        eprint!("\n\nPreview window closed. The render will continue.\nStop render with <C-c> on the terminal.\n\n");
    })
}
