use super::float::Float;
use super::vec3::Vec3;
type Point3 = Vec3;
use super::color::Color;
use super::texture::Texture;

use stb_image::image::{LoadResult, load_with_depth};
use std::path::Path;


const BYTES_PER_PIXEL: usize = 3;

pub struct ImageTexture {
    maybe_image: LoadResult
}

impl ImageTexture {
    pub fn new(path: &Path) -> Self {
        Self {
            maybe_image: load_with_depth(path, BYTES_PER_PIXEL, true)
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: Float, v: Float, _p: &Point3) -> Color {
        match &self.maybe_image {
            LoadResult::ImageU8(img) => {
                let u = u.clamp(0.0, 1.0);
                let v = 1.0 - v.clamp(0.0, 1.0);
                let i = ((u * img.width as Float) as usize).min(img.width-1);
                let j = ((v * img.height as Float) as usize).min(img.height-1);

                const COLOR_SCALE: Float = 1.0 / 255.0;
                let start = j*img.depth*img.width + i*img.depth;
                let end = start + 3*img.depth;
                let pixel = &img.data[start..end];

                COLOR_SCALE * Color::new(pixel[0] as Float, pixel[1] as Float, pixel[2] as Float)
            },
            _ => Color::new(0.0, 1.0, 1.0) // Missing image cyan
        }
    }
}
