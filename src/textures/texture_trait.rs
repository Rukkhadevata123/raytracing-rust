use crate::core::vec3::{Color, Point3};
use std::fmt::Debug;

/// Abstract interface for textures.
pub trait Texture: Send + Sync + Debug {
    /// Returns the color value of the texture at the given coordinates.
    /// u, v: texture coordinates [0, 1]
    /// p: world space point (for procedural textures like Perlin noise)
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}
