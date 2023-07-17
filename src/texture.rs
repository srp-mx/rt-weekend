use super::float::Float;
use super::vec3::Vec3;
type Point3 = Vec3;
use super::color::Color;

pub trait Texture {
    fn value(&self, u: Float, v: Float, p: &Point3) -> Color;
}

pub struct SolidColor {
    color_value: Color
}

impl SolidColor {
    pub fn new(color_value: Color) -> Self {
        Self { color_value }
    }

    pub fn new_rgb(red: Float, green: Float, blue: Float) -> Self {
        Self::new(Color::new(red, green, blue))
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: Float, _v: Float, _p: &Point3) -> Color {
        self.color_value.copy()
    }
}
