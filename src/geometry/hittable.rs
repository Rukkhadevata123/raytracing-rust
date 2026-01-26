use crate::core::aabb::Aabb;
use crate::core::interaction::Interaction;
use crate::core::interval::Interval;
use crate::core::ray::Ray;
use crate::core::vec3::{Point3, Vec3};
use std::fmt::Debug;

/// Trait representing any object that can be intersected by a ray.
pub trait Hittable: Send + Sync + Debug {
    /// Determines if a ray hits the object within the given interval.
    /// If hit, populates `isect` and returns true.
    fn hit(&self, r: &Ray, ray_t: Interval, isect: &mut Interaction) -> bool;

    /// Returns the axis-aligned bounding box of the object.
    fn bounding_box(&self) -> Aabb;

    /// Returns the Probability Density Function value for a given direction.
    fn pdf_value(&self, _origin: &Point3, _direction: &Vec3) -> f64 {
        0.0
    }

    /// Generates a random direction towards this object (for sampling).
    fn random(&self, _origin: &Point3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}
