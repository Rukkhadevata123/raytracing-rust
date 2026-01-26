use crate::core::vec3::{Color, Point3};
use crate::textures::texture_trait::Texture;
use image::{DynamicImage, GenericImageView};

#[derive(Debug)]
pub struct ImageTexture {
    image: Option<DynamicImage>,
    width: u32,
    height: u32,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        let img_result = image::open(filename);

        // Simple fallback logic, can be expanded
        match img_result {
            Ok(img) => {
                let width = img.width();
                let height = img.height();
                Self {
                    image: Some(img),
                    width,
                    height,
                }
            }
            Err(e) => {
                eprintln!("ERROR: Could not load image file '{}': {}", filename, e);
                Self {
                    image: None,
                    width: 0,
                    height: 0,
                }
            }
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        if self.image.is_none() {
            return Color::new(0.0, 1.0, 1.0); // Cyan debugging color
        }

        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0); // Flip V

        let i = (u * self.width as f64) as u32;
        let j = (v * self.height as f64) as u32;

        let i = i.min(self.width - 1);
        let j = j.min(self.height - 1);

        let pixel = self.image.as_ref().unwrap().get_pixel(i, j);

        let scale = 1.0 / 255.0;
        Color::new(
            pixel[0] as f64 * scale,
            pixel[1] as f64 * scale,
            pixel[2] as f64 * scale,
        )
    }
}
