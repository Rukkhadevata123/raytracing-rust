use std::f64::consts::PI;
use std::sync::Arc;

use crate::core::aabb::Aabb;
use crate::core::interaction::Interaction;
use crate::core::interval::Interval;
use crate::core::onb::ONB;
use crate::core::ray::Ray;
use crate::core::vec3::{Point3, Vec3, Vec3Ext};
use crate::geometry::hittable::Hittable;
use crate::materials::material_trait::Material;

#[derive(Debug)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Material>,
    is_moving: bool,
    center_vec: Vec3,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
            is_moving: false,
            center_vec: Vec3::zeros(),
        }
    }

    pub fn new_moving(
        center1: Point3,
        center2: Point3,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        Self {
            center: center1,
            radius,
            material,
            is_moving: true,
            center_vec: center2 - center1,
        }
    }

    fn center(&self, time: f64) -> Point3 {
        if self.is_moving {
            self.center + self.center_vec * time
        } else {
            self.center
        }
    }

    fn get_sphere_uv(p: &Point3) -> (f64, f64) {
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + PI;
        (phi / (2.0 * PI), theta / PI)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, isect: &mut Interaction) -> bool {
        let center = self.center(r.time); // Use time-varying center
        let oc = r.orig - center;
        let a = r.dir.norm_squared();
        let half_b = oc.dot(&r.dir);
        let c = oc.norm_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        let t = root;
        let p = r.at(t);
        let outward_normal = (p - center) / self.radius;
        let (u, v) = Self::get_sphere_uv(&Point3::from(outward_normal));

        *isect = Interaction::new(p, t, (u, v), Some(self.material.clone()));
        isect.set_face_normal(r, outward_normal);

        true
    }

    fn bounding_box(&self) -> Aabb {
        let rvec = Vec3::new(self.radius, self.radius, self.radius);
        let box1 = Aabb::new_point(self.center - rvec, self.center + rvec);

        if self.is_moving {
            let center2 = self.center + self.center_vec;
            let box2 = Aabb::new_point(center2 - rvec, center2 + rvec);
            box1.merge(&box2)
        } else {
            box1
        }
    }

    // Usually for lights
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        // Importance sampling currently assumes static sphere for simplicity
        // or effectively samples at time=0. A full implementation would sample time.
        let mut dummy = Interaction::default();
        let test_ray = Ray::new(*origin, *direction, 0.0);

        if !self.hit(&test_ray, Interval::new(0.001, f64::INFINITY), &mut dummy) {
            return 0.0;
        }

        let cos_theta_max =
            (1.0 - self.radius.powi(2) / (self.center - origin).norm_squared()).sqrt();
        let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);

        1.0 / solid_angle
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        let direction = self.center - origin;
        let dist_sq = direction.norm_squared();
        let uvw = ONB::build_from_w(&direction);
        uvw.local(&Vec3::random_to_sphere(self.radius, dist_sq))
    }
}
