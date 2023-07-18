use super::perlin::Perlin;
use super::float::Float;
use super::texture::Texture;
use super::color::Color;
use super::vec3::Vec3;
type Point3 = Vec3;
use super::rng_float::RngGen;

pub struct NoiseTexture {
    perlin: Perlin,
    scale: Float,
}

impl NoiseTexture {
    pub fn new(scale: Float, rng: &mut RngGen) -> Self {
        Self { perlin: Perlin::new(rng), scale }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: Float, _v: Float, p: &Point3) -> Color {
        Color::one() * 0.5 * (1.0 + self.perlin.noise(&(self.scale * p)))
    }
}
