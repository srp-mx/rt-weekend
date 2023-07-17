use super::float::Float;
use super::vec3::Vec3;
type Point3 = Vec3;
use super::color::Color;
use super::texture::{Texture, SolidColor};

use std::rc::Rc;

pub struct CheckerTexture {
    odd: Rc<dyn Texture>,
    even: Rc<dyn Texture>
}

impl CheckerTexture {
    pub fn new(odd: Rc<dyn Texture>, even: Rc<dyn Texture>) -> Self {
        Self { odd, even }
    }

    pub fn new_solid(odd_color: Color, even_color: Color) -> Self {
        let odd = Rc::new(SolidColor::new(odd_color));
        let even = Rc::new(SolidColor::new(even_color));
        Self { odd, even }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: Float, v: Float, p: &Point3) -> Color {
        let sines = ((10.0*p.x()).sin())*((10.0*p.y()).sin())*((10.0*p.z()).sin());
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
