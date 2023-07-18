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
pub mod image_texture;
pub mod renderer;
pub mod diffuse_light;
pub mod aarect;

use float::*;
use vec3::Vec3;
use rng_float::RngGen;
use pixel_buffer::PixelBuffer;
use default_scenes::{
    DefaultScene, select_default_scene, select_default_scene_cam_settings,
    select_default_scene_sky
};
use renderer::RenderInfo;

use minifb::{Window, WindowOptions};
use std::sync::RwLock;
use std::sync::Arc;
use std::thread::JoinHandle;
use std::rc::Rc;

fn main() {
    // RNG
    let mut rng = RngGen::new();

    // Image
    const ASPECT_RATIO:Float = 1.0;
    const IMAGE_WIDTH:usize = 300;
    const SAMPLES_PER_PIXEL:i32 = 400;
    const MAX_DEPTH: i32 = 12;

    // Pixel Buffer
    let buffer_lock = Arc::new(RwLock::new(PixelBuffer::new(IMAGE_WIDTH, ASPECT_RATIO)));

    // Scene, World, Sky and Camera Settings
    let ref scene = DefaultScene::CornellBox;
    let world = Rc::new(select_default_scene(scene, &mut rng));
    let sky = select_default_scene_sky(scene);
    let mut cam_settings = select_default_scene_cam_settings(scene);

    // Render Info
    let mut rend = RenderInfo::new(buffer_lock.clone(), &mut cam_settings);
    rend.sky(sky);
    rend.world(world);

    // Fast Render Pass for Preview
    eprintln!("Making a fast render pass for preview");
    rend.render(1, 1, &mut rng);

    // Preview Window
    let preview_thread = make_preview_window(buffer_lock.clone());

    // Render
    let timer_start = std::time::Instant::now();
    eprintln!("\nGetting serious now >:)\n");
    rend.render(SAMPLES_PER_PIXEL, MAX_DEPTH, &mut rng);
    let timer_duration = timer_start.elapsed(); 

    eprint!("\nWriting output.\n");
    let ppm_output = buffer_lock.read().unwrap().to_ppm();
    print!("{ppm_output}");
    eprint!("\nDone.\nRendering took {:?}\n", timer_duration);

    preview_thread.join().unwrap();
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

        eprintln!("\n\nPreview window closed. The render will continue to completion.");
        eprintln!("Stop render with <C-c> on the terminal.\n");
    })
}
