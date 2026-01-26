use crate::core::aabb::Aabb;
use crate::core::interaction::Interaction;
use crate::core::interval::Interval;
use crate::core::ray::Ray;
use crate::core::vec3::{Point3, Vec3};
use crate::geometry::hittable::Hittable;
use crate::materials::material_trait::Material;
use std::sync::Arc;

#[derive(Debug)]
pub struct Triangle {
    v0: Point3,
    v1: Point3,
    v2: Point3,
    material: Arc<dyn Material>,
    uv0: (f64, f64),
    uv1: (f64, f64),
    uv2: (f64, f64),
    normal: Vec3, // Pre-computed face normal
}

impl Triangle {
    pub fn new(v0: Point3, v1: Point3, v2: Point3, material: Arc<dyn Material>) -> Self {
        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        let normal = edge1.cross(&edge2).normalize();

        Self {
            v0,
            v1,
            v2,
            material,
            uv0: (0.0, 0.0),
            uv1: (1.0, 0.0),
            uv2: (0.0, 1.0),
            normal,
        }
    }

    pub fn with_uvs(mut self, uv0: (f64, f64), uv1: (f64, f64), uv2: (f64, f64)) -> Self {
        self.uv0 = uv0;
        self.uv1 = uv1;
        self.uv2 = uv2;
        self
    }
}

impl Hittable for Triangle {
    fn hit(&self, r: &Ray, ray_t: Interval, isect: &mut Interaction) -> bool {
        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;
        let h = r.dir.cross(&edge2);
        let a = edge1.dot(&h);

        // Parallel check using epsilon
        if a.abs() < 1e-8 {
            return false;
        }

        let f = 1.0 / a;
        let s = r.orig - self.v0;
        let u = f * s.dot(&h);

        if !(0.0..=1.0).contains(&u) {
            return false;
        }

        let q = s.cross(&edge1);
        let v = f * r.dir.dot(&q);

        if v < 0.0 || u + v > 1.0 {
            return false;
        }

        let t = f * edge2.dot(&q);

        if !ray_t.contains(t) {
            return false;
        }

        // --- Intersection confirmed ---
        let intersection_point = r.at(t);

        // Barycentric UV interpolation
        // w = 1 - u - v
        let w = 1.0 - u - v;
        let tex_u = w * self.uv0.0 + u * self.uv1.0 + v * self.uv2.0;
        let tex_v = w * self.uv0.1 + u * self.uv1.1 + v * self.uv2.1;

        *isect = Interaction::new(
            intersection_point,
            t,
            (tex_u, tex_v),
            Some(self.material.clone()),
        );
        isect.set_face_normal(r, self.normal);

        true
    }

    fn bounding_box(&self) -> Aabb {
        let min_x = self.v0.x.min(self.v1.x).min(self.v2.x);
        let min_y = self.v0.y.min(self.v1.y).min(self.v2.y);
        let min_z = self.v0.z.min(self.v1.z).min(self.v2.z);

        let max_x = self.v0.x.max(self.v1.x).max(self.v2.x);
        let max_y = self.v0.y.max(self.v1.y).max(self.v2.y);
        let max_z = self.v0.z.max(self.v1.z).max(self.v2.z);

        Aabb::new_point(
            Point3::new(min_x, min_y, min_z),
            Point3::new(max_x, max_y, max_z),
        )
    }
}
