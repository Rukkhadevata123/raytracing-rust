use crate::core::interaction::Interaction;
use crate::core::ray::Ray;
use crate::core::vec3::Vec3Ext;
use crate::materials::material_trait::{Material, ScatterRecord};
use crate::sampling::pdf::CosinePDF;
use crate::textures::texture_trait::Texture;
use std::f64::consts::PI;
use std::sync::Arc;

#[derive(Debug)]
pub struct Lambertian {
    texture: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, isect: &Interaction, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = self.texture.value(isect.uv.0, isect.uv.1, &isect.p);
        srec.pdf_ptr = Some(Arc::new(CosinePDF::new(&isect.geometry_normal)));
        srec.skip_pdf = false;
        true
    }

    fn scattering_pdf(&self, _r_in: &Ray, isect: &Interaction, scattered: &Ray) -> f64 {
        // Protect against NaN generation.
        // If the scattered ray length is near zero (e.g. self-intersection or numerical precision issues),
        // we cannot normalize it safely. Return 0.0 PDF in this case.
        if scattered.dir.near_zero() {
            return 0.0;
        }

        // Normalize direction before dot product
        let cos_theta = scattered.dir.normalize().dot(&isect.geometry_normal);

        if cos_theta < 0.0 { 0.0 } else { cos_theta / PI }
    }
}
