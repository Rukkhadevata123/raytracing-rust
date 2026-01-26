use crate::core::onb::ONB;
use crate::core::vec3::Vec3Ext;
use crate::core::vec3::{Point3, Vec3};
use crate::geometry::hittable::Hittable;
use crate::sampling::random::random_double;
use std::f64::consts::PI;
use std::fmt::Debug;
use std::sync::Arc;

/// Probability Density Function trait for importance sampling.
pub trait PDF: Send + Sync + Debug {
    /// Returns the probability density value for a given direction.
    fn value(&self, direction: &Vec3) -> f64;

    /// Generates a random direction based on the PDF distribution.
    fn generate(&self) -> Vec3;
}

// --- Cosine PDF (for Lambertian) ---
#[derive(Debug)]
pub struct CosinePDF {
    uvw: ONB,
}

impl CosinePDF {
    pub fn new(w: &Vec3) -> Self {
        Self {
            uvw: ONB::build_from_w(w),
        }
    }
}

impl PDF for CosinePDF {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine = direction.normalize().dot(&self.uvw.w());
        if cosine <= 0.0 { 0.0 } else { cosine / PI }
    }

    fn generate(&self) -> Vec3 {
        self.uvw.local(&Vec3::random_cosine_direction())
    }
}

// --- Sphere PDF (for Isotropic/Volume) ---
#[derive(Debug)]
pub struct SpherePDF;

impl PDF for SpherePDF {
    fn value(&self, _direction: &Vec3) -> f64 {
        1.0 / (4.0 * PI)
    }

    fn generate(&self) -> Vec3 {
        Vec3::random_unit_vector()
    }
}

// --- Hittable PDF (for Light Sampling) ---
pub struct HittablePDF {
    objects: Arc<dyn Hittable>,
    origin: Point3,
}

impl HittablePDF {
    pub fn new(objects: Arc<dyn Hittable>, origin: Point3) -> Self {
        Self { objects, origin }
    }
}

impl Debug for HittablePDF {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HittablePDF")
    }
}

impl PDF for HittablePDF {
    fn value(&self, direction: &Vec3) -> f64 {
        if direction.near_zero() {
            return 0.0;
        }

        self.objects.pdf_value(&self.origin, direction)
    }

    fn generate(&self) -> Vec3 {
        self.objects.random(&self.origin)
    }
}

// --- Mixture PDF (combines two strategies) ---
pub struct MixturePDF {
    p: [Arc<dyn PDF>; 2],
}

impl MixturePDF {
    pub fn new(p0: Arc<dyn PDF>, p1: Arc<dyn PDF>) -> Self {
        Self { p: [p0, p1] }
    }
}

impl Debug for MixturePDF {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MixturePDF")
    }
}

impl PDF for MixturePDF {
    fn value(&self, direction: &Vec3) -> f64 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }

    fn generate(&self) -> Vec3 {
        if random_double() < 0.5 {
            self.p[0].generate()
        } else {
            self.p[1].generate()
        }
    }
}
