use crate::core::interaction::Interaction;
use crate::core::ray::Ray;
use crate::core::vec3::{Color, Point3};
use crate::materials::material_trait::{Material, ScatterRecord};
use crate::textures::texture_trait::Texture;
use std::sync::Arc;

#[derive(Debug)]
pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emit: Arc<dyn Texture>) -> Self {
        Self { emit }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _isect: &Interaction, _srec: &mut ScatterRecord) -> bool {
        false // Lights do not scatter/reflect rays in this model
    }

    fn emitted(&self, _r_in: &Ray, isect: &Interaction, u: f64, v: f64, p: &Point3) -> Color {
        // Only emit light from the front face
        if isect.front_face {
            self.emit.value(u, v, p)
        } else {
            Color::zeros()
        }
    }
}
