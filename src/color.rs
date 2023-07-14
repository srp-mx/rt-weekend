pub use super::float::*;

pub type Color = super::Vec3;

impl Color {
    pub fn r(&self, scale: Float) -> i32 {
        Color::to_color(self.x(), scale)
    }

    pub fn g(&self, scale: Float) -> i32 {
        Color::to_color(self.y(), scale)
    }

    pub fn b(&self, scale: Float) -> i32 {
        Color::to_color(self.z(), scale)
    }

    fn to_color(input: Float, scale: Float) -> i32 {
        (256.0 * (input * scale).clamp(0.0, 0.999)) as i32
    }

    pub fn write_color(&self, samples_per_pixel: i32) {
        let s = (samples_per_pixel as Float).recip();
        print!("{} {} {}\n", self.r(s), self.g(s), self.b(s));
    }
}

