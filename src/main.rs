pub mod float;
pub mod vec3;
pub mod color;
pub mod ray;

use float::*;
use vec3::Vec3;
use color::Color;
type Point3 = Vec3;
use ray::Ray;

fn main() {
    // Image
    const ASPECT_RATIO:Float = 16 as Float / 9 as Float;
    const IMAGE_WIDTH:i32 = 400;
    const IMAGE_HEIGHT:i32 = ((IMAGE_WIDTH as Float) / ASPECT_RATIO) as i32;

    // Camera
    const VIEWPORT_HEIGHT:Float = 2 as Float;
    const VIEWPORT_WIDTH:Float = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH:Float = FLOAT_ONE;
    let ref origin:Point3 = Point3::zero();
    let ref horizontal = Vec3::new(VIEWPORT_WIDTH, FLOAT_ZERO, FLOAT_ZERO);
    let ref vertical = Vec3::new(FLOAT_ZERO, VIEWPORT_HEIGHT, FLOAT_ZERO);
    let ref focal = Vec3::new(FLOAT_ZERO, FLOAT_ZERO, FOCAL_LENGTH);
    let ref lower_left = origin - (horizontal/FLOAT_TWO) - (vertical/FLOAT_TWO) - focal;

    // Render
    print!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\nScanlines remaining: {}\n", j+1);
        for i in 0..IMAGE_WIDTH {
            let u: Float = (i as Float) / ((IMAGE_WIDTH-1) as Float);
            let v: Float = (j as Float) / ((IMAGE_HEIGHT-1) as Float);
            let dir: Vec3 = lower_left + u*horizontal + v*vertical - origin;
            let r: Ray = Ray::new(origin, &dir);
            let pixel_color = r.color();
            pixel_color.write_color();
        }
    }
    eprint!("\nDone.\n");
}

impl Ray {
    pub fn color(&self) -> Color {
        let center = Point3::new(FLOAT_ZERO, FLOAT_ZERO, -FLOAT_ONE);
        if hit_sphere(&center, FLOAT_TWO.recip(), &self) {
            return Color::new(FLOAT_ONE, FLOAT_ZERO, FLOAT_ZERO);
        }
        let unit_direction: Vec3 = self.direction().unit_vector();
        let t = (0.5 as Float) * (unit_direction.y() + FLOAT_ONE);
        let c1 = Color::new(FLOAT_ONE, FLOAT_ONE, FLOAT_ONE);
        let c2 = Color::new(0.5 as Float, 0.7 as Float, 1.0 as Float);
        Vec3::lerp(&c1, &c2, t)
    }
}

fn hit_sphere(center:&Point3, radius:Float, r:&Ray) -> bool {
    let oc: Vec3 = r.origin() - center;
    let a = Vec3::dot(r.direction(), r.direction());
    let b = FLOAT_TWO * Vec3::dot(&oc, r.direction());
    let c = Vec3::dot(&oc, &oc) - radius*radius;
    let discriminant = b*b - (4 as Float)*a*c;
    discriminant > FLOAT_ZERO
}
