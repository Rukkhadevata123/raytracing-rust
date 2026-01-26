use crate::core::aabb::Aabb;
use crate::core::interaction::Interaction;
use crate::core::interval::Interval;
use crate::core::ray::Ray;
use crate::core::vec3::{Point3, Vec3};
use crate::geometry::hittable::Hittable;
use crate::sampling::random::degrees_to_radians;
use std::sync::Arc;

#[derive(Debug)]
pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        // Calculate new bounding box by rotating all 8 corners
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1 - i) as f64 * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1 - j) as f64 * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1 - k) as f64 * bbox.z.min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    min.x = min.x.min(tester.x);
                    max.x = max.x.max(tester.x);
                    min.y = min.y.min(tester.y);
                    max.y = max.y.max(tester.y);
                    min.z = min.z.min(tester.z);
                    max.z = max.z.max(tester.z);
                }
            }
        }

        Self {
            object,
            sin_theta,
            cos_theta,
            bbox: Aabb::new_point(min, max),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: Interval, isect: &mut Interaction) -> bool {
        // Change ray from world space to object space
        let mut origin = r.orig;
        let mut direction = r.dir;

        origin.x = self.cos_theta * r.orig.x - self.sin_theta * r.orig.z;
        origin.z = self.sin_theta * r.orig.x + self.cos_theta * r.orig.z;

        direction.x = self.cos_theta * r.dir.x - self.sin_theta * r.dir.z;
        direction.z = self.sin_theta * r.dir.x + self.cos_theta * r.dir.z;

        let rotated_r = Ray::new(origin, direction, r.time);

        if !self.object.hit(&rotated_r, ray_t, isect) {
            return false;
        }

        // Change intersection point and normal from object space to world space
        let mut p = isect.p;
        p.x = self.cos_theta * isect.p.x + self.sin_theta * isect.p.z;
        p.z = -self.sin_theta * isect.p.x + self.cos_theta * isect.p.z;

        let mut normal = isect.geometry_normal;
        normal.x =
            self.cos_theta * isect.geometry_normal.x + self.sin_theta * isect.geometry_normal.z;
        normal.z =
            -self.sin_theta * isect.geometry_normal.x + self.cos_theta * isect.geometry_normal.z;

        isect.p = p;
        // Update shading normal and face flags using the new world-space geometry normal
        isect.set_face_normal(r, normal);

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        // Rotate ray to object space
        let mut origin_rot = *origin;
        origin_rot.x = self.cos_theta * origin.x - self.sin_theta * origin.z;
        origin_rot.z = self.sin_theta * origin.x + self.cos_theta * origin.z;

        let mut dir_rot = *direction;
        dir_rot.x = self.cos_theta * direction.x - self.sin_theta * direction.z;
        dir_rot.z = self.sin_theta * direction.x + self.cos_theta * direction.z;

        self.object.pdf_value(&origin_rot, &dir_rot)
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        // Transform user viewpoint to object space
        let mut origin_rot = *origin;
        origin_rot.x = self.cos_theta * origin.x - self.sin_theta * origin.z;
        origin_rot.z = self.sin_theta * origin.x + self.cos_theta * origin.z;

        let local_dir = self.object.random(&origin_rot);

        // Rotate random direction back to world space
        let mut world_dir = local_dir;
        world_dir.x = self.cos_theta * local_dir.x + self.sin_theta * local_dir.z;
        world_dir.z = -self.sin_theta * local_dir.x + self.cos_theta * local_dir.z;

        world_dir
    }
}
