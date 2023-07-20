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
use super::camera::CameraBuilder;
use super::noise_texture::NoiseTexture;
use super::image_texture::ImageTexture;
use super::renderer::Sky;
use super::diffuse_light::DiffuseLight;
use super::aarect::{XyRect, YzRect, ZxRect};
use super::rect_prism::RectPrism;
use super::translate::Translate;
use super::rotate_y::RotateY;

use std::path::Path;
use std::rc::Rc;

pub enum DefaultScene {
    RandomScene,
    TwoSpheres,
    PerlinSpheres,
    Earth,
    SimpleLight,
    CornellBox,
}

pub fn select_default_scene(scene: &DefaultScene, rng: &mut RngGen) -> HittableList {
    match scene {
        DefaultScene::RandomScene => random_scene(rng),
        DefaultScene::TwoSpheres => two_spheres(),
        DefaultScene::PerlinSpheres => perlin_spheres(rng),
        DefaultScene::Earth => earth(),
        DefaultScene::SimpleLight => simple_light(rng),
        DefaultScene::CornellBox => cornell_box(),
    }
}

pub fn select_default_scene_cam_settings(scene: &DefaultScene) -> CameraBuilder {
    match scene {
        DefaultScene::RandomScene => random_scene_cam(),
        DefaultScene::TwoSpheres => two_spheres_cam(),
        DefaultScene::PerlinSpheres => perlin_spheres_cam(),
        DefaultScene::Earth => earth_cam(),
        DefaultScene::SimpleLight => simple_light_cam(),
        DefaultScene::CornellBox => cornell_box_cam(),
    }
}

pub fn select_default_scene_sky(scene: &DefaultScene) -> Sky {
    match scene {
        DefaultScene::RandomScene
        | DefaultScene::TwoSpheres
        | DefaultScene::Earth
        | DefaultScene::PerlinSpheres =>
            Sky::Gradient(Color::one(), Color::new(0.5, 0.7, 1.0)),

        DefaultScene::SimpleLight
        | DefaultScene::CornellBox =>
            Sky::SolidColor(Color::zero()),
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

fn random_scene_cam() -> CameraBuilder {
    let mut cam = CameraBuilder::new();
    cam.lookfrom(Vec3::new(12.0, 2.0, 3.0))
        .lookat(Vec3::zero())
        .vertical_fov(20.0)
        .focus_dist(10.0)
        .aperture(0.1)
        .aspect_ratio(16.0 / 9.0)
        .shutter_open_time(0.0)
        .shutter_close_time(1.0);
    cam
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

fn two_spheres_cam() -> CameraBuilder {
    let mut cam = CameraBuilder::new();
    cam.lookfrom(Vec3::new(13.0, 2.0, 3.0))
        .lookat(Vec3::zero())
        .vertical_fov(20.0)
        .focus_dist(10.0)
        .aperture(0.0)
        .aspect_ratio(16.0 / 9.0)
        .shutter_open_time(0.0)
        .shutter_close_time(1.0);
    cam
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

fn perlin_spheres_cam() -> CameraBuilder {
    let mut cam  = CameraBuilder::new();
    cam.lookfrom(Vec3::new(13.0, 2.0, 3.0))
        .lookat(Vec3::zero())
        .vertical_fov(20.0)
        .focus_dist(10.0)
        .aperture(0.0)
        .aspect_ratio(16.0 / 9.0)
        .shutter_open_time(0.0)
        .shutter_close_time(1.0);
    cam
}

/* Previous settings:
 * IMAGE
    const ASPECT_RATIO:Float = 16.0 / 9.0;
    const IMAGE_WIDTH:usize = 300;
    const IMAGE_HEIGHT:usize = ((IMAGE_WIDTH as Float) / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL:i32 = 50;
    const MAX_DEPTH: i32 = 12;
 * */
fn earth() -> HittableList {
    let earth_tex = Rc::new(ImageTexture::new(Path::new("earthmap.jpg")));
    let earth_surf = Rc::new(Lambertian::new(earth_tex.clone()));
    let globe = Rc::new(Sphere::new(Point3::zero(), 2.0, earth_surf.clone()));
    let mut earth = HittableList::new();
    earth.add(globe);
    earth
}

fn earth_cam() -> CameraBuilder {
    let mut cam = CameraBuilder::new();
    cam.lookfrom(Vec3::new(13.0, 2.0, 3.0))
        .lookat(Vec3::zero())
        .vertical_fov(20.0)
        .focus_dist(10.0)
        .aperture(0.0)
        .aspect_ratio(16.0 / 9.0)
        .shutter_open_time(0.0)
        .shutter_close_time(1.0);
    cam
}

/* Previous settings:
 * IMAGE
    const ASPECT_RATIO:Float = 16.0 / 9.0;
    const IMAGE_WIDTH:usize = 300;
    const IMAGE_HEIGHT:usize = ((IMAGE_WIDTH as Float) / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL:i32 = 400;
    const MAX_DEPTH: i32 = 12;
 * */
fn simple_light(rng: &mut RngGen) -> HittableList {
    let mut objects = perlin_spheres(rng);
    let difflight = Rc::new(DiffuseLight::new_from_color(&(Color::one()*4.0)));
    objects.add(Rc::new(XyRect::new(3.0,5.0,1.0,3.0,-2.0,difflight)));
    objects
}

fn simple_light_cam() -> CameraBuilder {
    let mut cam = perlin_spheres_cam();
    cam.lookfrom(Point3::new(26.0,3.0,6.0))
        .lookat(Point3::new(0.0,2.0,0.0))
        .vertical_fov(20.0);
    cam
}


/* Previous settings:
 * IMAGE
    const ASPECT_RATIO:Float = 1.0;
    const IMAGE_WIDTH:usize = 600;
    const IMAGE_HEIGHT:usize = ((IMAGE_WIDTH as Float) / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL:i32 = 200;
    const MAX_DEPTH: i32 = 12;
 * */
fn cornell_box() -> HittableList {
    let mut objects = HittableList::new();
    let red = Rc::new(Lambertian::new(Rc::new(SolidColor::new(Color::new(0.65,0.05,0.05)))));
    let white = Rc::new(Lambertian::new(Rc::new(SolidColor::new(Color::new(0.73,0.73,0.73)))));
    let green = Rc::new(Lambertian::new(Rc::new(SolidColor::new(Color::new(0.12,0.45,0.15)))));
    let light = Rc::new(DiffuseLight::new_from_color(&Color::new(15.0,15.0,15.0)));
    objects.add(Rc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green.clone())));
    objects.add(Rc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red.clone())));
    objects.add(Rc::new(ZxRect::new(227.0, 332.0, 213.0, 343.0, 554.0, light.clone())));
    objects.add(Rc::new(ZxRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone())));
    objects.add(Rc::new(ZxRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));
    objects.add(Rc::new(XyRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));

    let box1 = Rc::new(RectPrism::new(&Point3::new(0.0,0.0,0.0), &Point3::new(165.0,330.0,165.0), white.clone()));
    let box1 = Rc::new(RotateY::new(box1, 15.0));
    let box1 = Rc::new(Translate::new(box1, Vec3::new(265.0,0.0,295.0)));
    objects.add(box1);
    let box2 = Rc::new(RectPrism::new(&Point3::new(0.0,0.0,0.0), &Point3::new(165.0,165.0,165.0), white.clone()));
    let box2 = Rc::new(RotateY::new(box2, -18.0));
    let box2 = Rc::new(Translate::new(box2, Vec3::new(130.0,0.0,65.0)));
    objects.add(box2);

    objects
}

fn cornell_box_cam() -> CameraBuilder {
    let mut cam = CameraBuilder::new();
    cam.aspect_ratio(1.0)
        .lookfrom(Point3::new(278.0, 278.0, -800.0))
        .lookat(Point3::new(278.0, 278.0, 0.0))
        .aperture(0.0)
        .vertical_fov(40.0);
    cam
}
