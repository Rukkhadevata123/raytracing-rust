use crate::core::ray::Ray;
use crate::core::vec3::{Point3, Vec3};
use crate::materials::material_trait::Material;
use std::sync::Arc;

/// Interaction represents a point on a surface where a light ray interacts with geometry.
/// It replaces the legacy 'HitRecord'.
#[derive(Clone)]
pub struct Interaction {
    pub p: Point3,                           // Intersection point
    pub geometry_normal: Vec3,               // The true geometric normal
    pub shading_normal: Vec3,                // The interpolated/perturbed normal (for shading)
    pub wo: Vec3,                            // Outgoing direction (usually -ray.direction)
    pub t: f64,                              // Ray parameter t
    pub uv: (f64, f64),                      // Texture coordinates
    pub front_face: bool,                    // Is incident ray hitting the front face?
    pub material: Option<Arc<dyn Material>>, // The material at this point
}

impl Interaction {
    pub fn new(p: Point3, t: f64, uv: (f64, f64), material: Option<Arc<dyn Material>>) -> Self {
        Self {
            p,
            geometry_normal: Vec3::zeros(),
            shading_normal: Vec3::zeros(),
            wo: Vec3::zeros(),
            t,
            uv,
            front_face: true,
            material,
        }
    }

    /// Default initializer for empty/temp interactions
    pub fn default() -> Self {
        Self {
            p: Point3::origin(),
            geometry_normal: Vec3::zeros(),
            shading_normal: Vec3::zeros(),
            wo: Vec3::zeros(),
            t: 0.0,
            uv: (0.0, 0.0),
            front_face: true,
            material: None,
        }
    }

    /// Sets face normals based on ray direction.
    /// `outward_normal` must be normalized.
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.dir.dot(&outward_normal) < 0.0;
        self.geometry_normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
        // For now, shading normal equals geometry normal
        self.shading_normal = self.geometry_normal;
        self.wo = -ray.dir.normalize();
    }
}
