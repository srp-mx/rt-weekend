pub use super::float::*;

pub type Color = super::Vec3;

impl Color {
    pub fn r(&self) -> i32 {
        (self.x() * (255.999 as Float)) as i32
    }

    pub fn g(&self) -> i32 {
        (self.y() * (255.999 as Float)) as i32
    }

    pub fn b(&self) -> i32 {
        (self.z() * (255.999 as Float)) as i32
    }

    pub fn write_color(&self) {
        print!("{} {} {}\n", self.r(), self.g(), self.b());
    }
}

