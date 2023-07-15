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
        (256.0 * (input * scale).sqrt().clamp(0.0, 0.999)) as i32
    }

    pub fn output_32bit(&self, samples_per_pixel: i32) -> u32 {
        let s = (samples_per_pixel as Float).recip();
        let r = (self.r(s) as u32) << (8*2);
        let g = (self.g(s) as u32) << 8;
        r | g | self.b(s) as u32
    }
}

