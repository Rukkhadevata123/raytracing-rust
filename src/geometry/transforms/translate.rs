use crate::core::aabb::Aabb;
use crate::core::interaction::Interaction;
use crate::core::interval::Interval;
use crate::core::ray::Ray;
use crate::core::vec3::{Point3, Vec3};
use crate::geometry::hittable::Hittable;
use std::sync::Arc;

#[derive(Debug)]
pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bbox: Aabb,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: Interval, isect: &mut Interaction) -> bool {
        // Move ray backwards to object space
        let offset_r = Ray::new(r.orig - self.offset, r.dir, r.time);

        if !self.object.hit(&offset_r, ray_t, isect) {
            return false;
        }

        // Move intersection point forward to world space
        isect.p += self.offset;

        // Normal and wo do not change with translation

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        // Look from the perspective of the object
        self.object.pdf_value(&(*origin - self.offset), direction)
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        // Look from the perspective of the object
        self.object.random(&(*origin - self.offset))
    }
}
