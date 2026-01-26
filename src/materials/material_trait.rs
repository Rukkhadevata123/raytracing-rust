use crate::core::interaction::Interaction;
use crate::core::ray::Ray;
use crate::core::vec3::{Color, Point3};
use crate::sampling::pdf::PDF;
use std::fmt::Debug;
use std::sync::Arc;

/// Record of how a ray scatters from a material.
pub struct ScatterRecord {
    pub attenuation: Color,
    pub pdf_ptr: Option<Arc<dyn PDF>>,
    pub skip_pdf: bool,
    pub skip_pdf_ray: Ray,
}

impl Default for ScatterRecord {
    fn default() -> Self {
        Self {
            attenuation: Color::zeros(),
            pdf_ptr: None,
            skip_pdf: false,
            skip_pdf_ray: Ray::default(),
        }
    }
}

pub trait Material: Send + Sync + Debug {
    /// Determines how the ray scatters. returns true if scattered, populating srec.
    fn scatter(&self, _r_in: &Ray, _isect: &Interaction, _srec: &mut ScatterRecord) -> bool {
        false
    }

    /// Emitted light (default: black).
    fn emitted(&self, _r_in: &Ray, _isect: &Interaction, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::zeros()
    }

    /// PDF for scattering direction.
    fn scattering_pdf(&self, _r_in: &Ray, _isect: &Interaction, _scattered: &Ray) -> f64 {
        0.0
    }
}
