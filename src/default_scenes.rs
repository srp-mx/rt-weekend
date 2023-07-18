use super::float::Float;
use super::vec3::Vec3;
type Point3 = Vec3;
use super::color::Color;
use super::hittable::Hittable;
use super::hittable_list::HittableList;
use super::rng_float::RngGen;
use super::texture::SolidColor;
use super::checker_texture::CheckerTexture;
use super::material::Material;
use super::lambertian::Lambertian;
use super::metal::Metal;
use super::dielectric::Dielectric;
use super::sphere::Sphere;
use super::moving_sphere::MovingSphere;
use super::bvh::BVH;
use super::camera::{Camera, CameraBuilder};
use super::noise_texture::NoiseTexture;

use std::rc::Rc;

pub enum DefaultScene {
    RandomScene,
    TwoSpheres,
    PerlinSpheres,
}

pub fn select_default_scene(scene: &DefaultScene, rng: &mut RngGen) -> HittableList {
    match scene {
        DefaultScene::RandomScene => random_scene(rng),
        DefaultScene::TwoSpheres => two_spheres(),
        DefaultScene::PerlinSpheres => perlin_spheres(rng),
    }
}

pub fn select_default_scene_cam(scene: &DefaultScene, aspect_ratio: Float) -> Camera {
    match scene {
        DefaultScene::RandomScene => random_scene_cam(aspect_ratio),
        DefaultScene::TwoSpheres => two_spheres_cam(aspect_ratio),
        DefaultScene::PerlinSpheres => perlin_spheres_cam(aspect_ratio),
    }
}

/* Previous settings:
 * IMAGE
    const ASPECT_RATIO:Float = 16.0 / 9.0;
    const IMAGE_WIDTH:usize = 300;
    const IMAGE_HEIGHT:usize = ((IMAGE_WIDTH as Float) / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL:i32 = 50;
    const MAX_DEPTH: i32 = 12;
 * */
fn random_scene(rng: &mut RngGen) -> HittableList {
    let mut world = HittableList::new();

    let checker = Rc::new(CheckerTexture::new_solid(Color::new(0.2,0.3,0.1), Color::new(0.9,0.9,0.9)));
    let ground_mat = Rc::new(Lambertian::new(checker));
    let ground: Rc<dyn Hittable> = Rc::new(Sphere::new(Point3::new(0.0,-1000.0,0.0), 1000.0, ground_mat));
    world.add(ground);

    let mut balls_list = HittableList::new();

    for a in -11..12 {
        for b in -11..12 {
            let choose_mat = rng.get();
            let center = Point3::new(a as Float + 0.9*rng.get(), 0.2, b as Float + 0.9*rng.get());
            let mut center2 = center.copy();
            
            if (&center - &Point3::new(4.0, 0.2, 0.0)).length() <= 0.9 {
                continue;
            }

            let sphere_mat: Rc<dyn Material> = if choose_mat < 0.8 {
                center2 = &center + Vec3::new(0.0, rng.range(0.0, 0.5), 0.0);
                Rc::new(Lambertian::new(Rc::new(SolidColor::new(Color::random(rng) * Color::random(rng)))))
            } else if choose_mat < 0.95 {
                Rc::new(Metal::new(Color::random_range(rng, 0.5, 1.0), rng.range(0.0, 0.5)))
            } else {
                Rc::new(Dielectric::new(1.5))
            };

            balls_list.add(Rc::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_mat)));
        }
    }

    let mat1: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));
    let sph1: Rc<dyn Hittable> = Rc::new(Sphere::new(Point3::new(0.0,1.0,0.0), 1.0, mat1));
    balls_list.add(sph1);

    let mat2: Rc<dyn Material> = Rc::new(Lambertian::new(Rc::new(SolidColor::new(Color::new(0.4, 0.2, 0.1)))));
    let sph2: Rc<dyn Hittable> = Rc::new(Sphere::new(Point3::new(-4.0,1.0,0.0), 1.0, mat2));
    balls_list.add(sph2);

    let mat3: Rc<dyn Material> = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    let sph3: Rc<dyn Hittable> = Rc::new(Sphere::new(Point3::new(4.0,1.0,0.0), 1.0, mat3));
    balls_list.add(sph3);

    let ball_bvh = Rc::new(BVH::new(&mut balls_list, 0.0, 1.0, rng));
    world.add(ball_bvh);

    world
}

fn random_scene_cam(aspect_ratio: Float) -> Camera {
    CameraBuilder::new()
            .lookfrom(Vec3::new(12.0, 2.0, 3.0))
            .lookat(Vec3::zero())
            .vertical_fov(20.0)
            .focus_dist(10.0)
            .aperture(0.1)
            .aspect_ratio(aspect_ratio)
            .shutter_open_time(0.0)
            .shutter_close_time(1.0)
            .build()
}


/* Previous settings:
 * IMAGE
    const ASPECT_RATIO:Float = 16.0 / 9.0;
    const IMAGE_WIDTH:usize = 300;
    const IMAGE_HEIGHT:usize = ((IMAGE_WIDTH as Float) / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL:i32 = 50;
    const MAX_DEPTH: i32 = 12;
 * */
fn two_spheres() -> HittableList {
    let mut objects = HittableList::new();
    let odd_color = Color::new(0.2,0.3,0.1);
    let even_color = Color::new(0.9, 0.9, 0.9);
    let checker = Rc::new(CheckerTexture::new_solid(odd_color, even_color));
    let mat = Rc::new(Lambertian::new(checker));
    let s1 = Rc::new(Sphere::new(Point3::new(0.0,-10.0,0.0), 10.0, mat.clone()));
    let s2 = Rc::new(Sphere::new(Point3::new(0.0,10.0,0.0), 10.0, mat.clone()));
    objects.add(s1);
    objects.add(s2);

    objects
}

fn two_spheres_cam(aspect_ratio: Float) -> Camera {
    CameraBuilder::new()
            .lookfrom(Vec3::new(13.0, 2.0, 3.0))
            .lookat(Vec3::zero())
            .vertical_fov(20.0)
            .focus_dist(10.0)
            .aperture(0.0)
            .aspect_ratio(aspect_ratio)
            .shutter_open_time(0.0)
            .shutter_close_time(1.0)
            .build()
}

/* Previous settings:
 * IMAGE
    const ASPECT_RATIO:Float = 16.0 / 9.0;
    const IMAGE_WIDTH:usize = 300;
    const IMAGE_HEIGHT:usize = ((IMAGE_WIDTH as Float) / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL:i32 = 50;
    const MAX_DEPTH: i32 = 12;
 * */
fn perlin_spheres(rng: &mut RngGen) -> HittableList {
    let mut objects = HittableList::new();   
    let pertext = Rc::new(Lambertian::new(Rc::new(NoiseTexture::new(4.0, rng))));
    let s1 = Rc::new(Sphere::new(Point3::new(0.0,-1000.0,0.0), 1000.0, pertext.clone()));
    let s2 = Rc::new(Sphere::new(Point3::new(0.0, 2.0,0.0), 2.0, pertext.clone()));
    objects.add(s1);
    objects.add(s2);
    objects
}

fn perlin_spheres_cam(aspect_ratio: Float) -> Camera {
    CameraBuilder::new()
            .lookfrom(Vec3::new(13.0, 2.0, 3.0))
            .lookat(Vec3::zero())
            .vertical_fov(20.0)
            .focus_dist(10.0)
            .aperture(0.0)
            .aspect_ratio(aspect_ratio)
            .shutter_open_time(0.0)
            .shutter_close_time(1.0)
            .build()
}
