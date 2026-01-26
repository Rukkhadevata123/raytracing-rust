use crate::core::aabb::Aabb;
use crate::core::interaction::Interaction;
use crate::core::interval::Interval;
use crate::core::ray::Ray;
use crate::core::vec3::{Point3, Vec3};
use crate::geometry::hittable::Hittable;
use crate::geometry::hittable_list::HittableList;
use crate::materials::material_trait::Material;
use crate::sampling::random::random_double;
use std::sync::Arc;

#[derive(Debug)]
pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    material: Arc<dyn Material>,
    bbox: Aabb,
    normal: Vec3,
    d: f64,
    w: Vec3,
    area: f64,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, material: Arc<dyn Material>) -> Self {
        let n = u.cross(&v);
        let normal = n.normalize();
        let d = normal.dot(&q.coords);
        let w = n / n.dot(&n);
        let area = n.norm();

        // Compute BBox
        let bbox_diag1 = Aabb::new_point(q, q + u + v);
        let bbox_diag2 = Aabb::new_point(q + u, q + v);

        Self {
            q,
            u,
            v,
            material,
            bbox: bbox_diag1.merge(&bbox_diag2),
            normal,
            d,
            w,
            area,
        }
    }

    fn is_interior(&self, a: f64, b: f64, isect: &mut Interaction) -> bool {
        // Given the hit point in plane coordinates, return false if it is outside the
        // primitive. Otherwise set the hit record UV coordinates and return true.
        if !(0.0..=1.0).contains(&a) || !(0.0..=1.0).contains(&b) {
            return false;
        }

        isect.uv = (a, b);
        true
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: Interval, isect: &mut Interaction) -> bool {
        let denom = self.normal.dot(&r.dir);

        // Ray parallel to the plane
        if denom.abs() < 1e-8 {
            return false;
        }

        let t = (self.d - self.normal.dot(&r.orig.coords)) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = self.w.dot(&planar_hitpt_vector.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hitpt_vector));

        if !self.is_interior(alpha, beta, isect) {
            return false;
        }

        // Hit confirmed
        isect.t = t;
        isect.p = intersection;
        isect.material = Some(self.material.clone());
        isect.set_face_normal(r, self.normal);

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        let mut rec = Interaction::default();
        if !self.hit(
            &Ray::new(*origin, *direction, 0.0),
            Interval::new(0.001, f64::INFINITY),
            &mut rec,
        ) {
            return 0.0;
        }

        let distance_squared = rec.t * rec.t * direction.norm_squared();
        let cosine = (direction.dot(&rec.geometry_normal) / direction.norm()).abs();

        if cosine < 1e-8 {
            0.0
        } else {
            distance_squared / (cosine * self.area)
        }
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        let p = self.q + (random_double() * self.u) + (random_double() * self.v);
        // Normalize the return vector to ensure consistency with PDF expectations
        (p - *origin).normalize()
    }
}

/// Helper to create a box (6 quads)
pub fn box_new(a: Point3, b: Point3, mat: Arc<dyn Material>) -> HittableList {
    let mut sides = HittableList::new();

    let min = Point3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
    let max = Point3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

    let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y - min.y, 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z - min.z);

    sides.add(Arc::new(Quad::new(
        Point3::new(min.x, min.y, max.z),
        dx,
        dy,
        mat.clone(),
    ))); // Front
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x, min.y, max.z),
        -dz,
        dy,
        mat.clone(),
    ))); // Right
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x, min.y, min.z),
        -dx,
        dy,
        mat.clone(),
    ))); // Back
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x, min.y, min.z),
        dz,
        dy,
        mat.clone(),
    ))); // Left
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x, max.y, max.z),
        dx,
        -dz,
        mat.clone(),
    ))); // Top
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x, min.y, min.z),
        dx,
        dz,
        mat.clone(),
    ))); // Bottom

    sides
}
