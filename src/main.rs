pub mod float;
pub mod vec3;
pub mod color;

use vec3::Vec3;
use color::Color;
use float::Float;
type Point3 = Vec3;

fn main() {
    // Image
    const IMAGE_WIDTH:i32 = 256;
    const IMAGE_HEIGHT:i32 = 256;

    // Render
    print!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\nScanlines remaining: {}\n", j+1);
        for i in 0..IMAGE_WIDTH {
            let r = (i as Float) / ((IMAGE_WIDTH-1) as Float);
            let g = (j as Float) / ((IMAGE_HEIGHT-1) as Float);
            let b = 0.25 as Float;
            let pixel_color = Color::new(r, g, b);
            pixel_color.write_color();
        }
    }
    eprint!("\nDone.\n");
}
