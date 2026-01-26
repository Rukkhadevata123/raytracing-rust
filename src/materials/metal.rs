use crate::core::interaction::Interaction;
use crate::core::ray::Ray;
use crate::core::vec3::{Color, Vec3, Vec3Ext};
use crate::materials::material_trait::{Material, ScatterRecord};

#[derive(Debug)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, isect: &Interaction, srec: &mut ScatterRecord) -> bool {
        let reflected = r_in.dir.normalize().reflect(&isect.geometry_normal);
        let fuzzed = reflected + self.fuzz * Vec3::random_unit_vector();

        srec.attenuation = self.albedo;
        srec.skip_pdf = true;
        srec.skip_pdf_ray = Ray::new(isect.p, fuzzed, r_in.time);

        true
    }
}
