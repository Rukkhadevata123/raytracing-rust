use crate::core::interaction::Interaction;
use crate::core::ray::Ray;
// Vec3Ext required for random_unit_vector
use crate::materials::material_trait::{Material, ScatterRecord};
use crate::sampling::pdf::SpherePDF;
use crate::textures::texture_trait::Texture;
use std::sync::Arc;

#[derive(Debug)]
pub struct Isotropic {
    texture: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }
}

impl Material for Isotropic {
    fn scatter(&self, _r_in: &Ray, isect: &Interaction, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = self.texture.value(isect.uv.0, isect.uv.1, &isect.p);
        srec.pdf_ptr = Some(Arc::new(SpherePDF));
        srec.skip_pdf = false;
        true
    }

    fn scattering_pdf(&self, _r_in: &Ray, _isect: &Interaction, _scattered: &Ray) -> f64 {
        1.0 / (4.0 * std::f64::consts::PI)
    }
}
