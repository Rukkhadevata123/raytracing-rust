use crate::core::aabb::Aabb;
use crate::core::interaction::Interaction;
use crate::core::interval::Interval;
use crate::core::ray::Ray;
use crate::core::vec3::Vec3;
use crate::geometry::hittable::Hittable;
use crate::materials::isotropic::Isotropic;
use crate::materials::material_trait::Material;
use crate::sampling::random::random_double;
use crate::textures::texture_trait::Texture;
use std::sync::Arc;

#[derive(Debug)]
pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: f64, texture: Arc<dyn Texture>) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new(texture)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, ray_t: Interval, isect: &mut Interaction) -> bool {
        // Print debugging maybe occasionally useful, but omitted for speed
        let mut rec1 = Interaction::default();
        let mut rec2 = Interaction::default();

        if !self.boundary.hit(r, Interval::universe(), &mut rec1) {
            return false;
        }

        if !self
            .boundary
            .hit(r, Interval::new(rec1.t + 0.0001, f64::INFINITY), &mut rec2)
        {
            return false;
        }

        if rec1.t < ray_t.min {
            rec1.t = ray_t.min;
        }
        if rec2.t > ray_t.max {
            rec2.t = ray_t.max;
        }

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.dir.norm();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;

        // Clamp random value to avoid log(0) = -inf, which causes NaNs/Infs
        let rand_val = random_double().max(f64::EPSILON);
        let hit_distance = self.neg_inv_density * rand_val.ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        isect.t = rec1.t + hit_distance / ray_length;
        isect.p = r.at(isect.t);

        isect.geometry_normal = Vec3::new(1.0, 0.0, 0.0); // Arbitrary
        isect.front_face = true; // Also arbitrary
        isect.material = Some(self.phase_function.clone());

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.boundary.bounding_box()
    }
}
