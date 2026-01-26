use crate::core::vec3::{Color, Point3};
use crate::textures::perlin::Perlin;
use crate::textures::texture_trait::Texture;

#[derive(Debug)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        // Marble-like texture using sine of turbulence
        let s = self.scale * p.z + 10.0 * self.noise.turb(p, 7);
        Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + s.sin())
    }
}
