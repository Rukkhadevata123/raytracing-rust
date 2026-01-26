use crate::core::interaction::Interaction;
use crate::core::ray::Ray;
use crate::core::vec3::{Color, Vec3Ext};
use crate::materials::material_trait::{Material, ScatterRecord};
use crate::sampling::random::random_double;

#[derive(Debug)]
pub struct Dielectric {
    ir: f64, // Index of Refraction
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, isect: &Interaction, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = Color::new(1.0, 1.0, 1.0);
        srec.skip_pdf = true;

        let refraction_ratio = if isect.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = r_in.dir.normalize();

        let cos_theta = (-unit_direction).dot(&isect.geometry_normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random_double() {
                unit_direction.reflect(&isect.geometry_normal)
            } else {
                unit_direction.refract(&isect.geometry_normal, refraction_ratio)
            };

        srec.skip_pdf_ray = Ray::new(isect.p, direction, r_in.time);
        true
    }
}
