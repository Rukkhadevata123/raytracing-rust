use crate::sampling::random::{random_double, random_double_range};
use nalgebra::{Point3 as NalgebraPoint3, Vector3 as NalgebraVector3};
use std::f64::consts::PI;

/// 3D Vector type alias
pub type Vec3 = NalgebraVector3<f64>;
/// 3D Point type alias
pub type Point3 = NalgebraPoint3<f64>;
/// Color type alias
pub type Color = Vec3;

/// Extension trait for Vec3 to add ray tracing specific functionality
pub trait Vec3Ext {
    fn random() -> Self; // Add this line
    fn random_range(min: f64, max: f64) -> Self;
    fn random_unit_vector() -> Self;
    fn random_in_unit_disk() -> Self;
    fn random_cosine_direction() -> Self;
    /// Generates a random vector to a sphere of radius `radius` at distance squared `distance_squared`
    fn random_to_sphere(radius: f64, distance_squared: f64) -> Self;

    fn reflect(&self, n: &Vec3) -> Vec3;
    fn refract(&self, n: &Vec3, etai_over_etat: f64) -> Vec3;
    fn near_zero(&self) -> bool;
}

impl Vec3Ext for Vec3 {
    #[inline]
    fn random() -> Self {
        Self::new(random_double(), random_double(), random_double())
    }

    #[inline]
    fn random_range(min: f64, max: f64) -> Self {
        Self::new(
            random_double_range(min, max),
            random_double_range(min, max),
            random_double_range(min, max),
        )
    }

    fn random_unit_vector() -> Self {
        loop {
            let p = Self::random_range(-1.0, 1.0);
            let len_squared = p.norm_squared();
            if 1e-160 < len_squared && len_squared <= 1.0 {
                return p / len_squared.sqrt();
            }
        }
    }

    fn random_in_unit_disk() -> Self {
        loop {
            let p = Self::new(
                random_double_range(-1.0, 1.0),
                random_double_range(-1.0, 1.0),
                0.0,
            );
            if p.norm_squared() < 1.0 {
                return p;
            }
        }
    }

    fn random_cosine_direction() -> Self {
        let r1 = random_double();
        let r2 = random_double();
        let phi = 2.0 * PI * r1;

        let x = phi.cos() * r2.sqrt();
        let y = phi.sin() * r2.sqrt();
        let z = (1.0 - r2).sqrt();

        Self::new(x, y, z)
    }

    fn random_to_sphere(radius: f64, distance_squared: f64) -> Self {
        let r1 = random_double();
        let r2 = random_double();
        let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);

        let phi = 2.0 * PI * r1;
        let x = phi.cos() * (1.0 - z * z).sqrt();
        let y = phi.sin() * (1.0 - z * z).sqrt();

        Self::new(x, y, z)
    }

    #[inline]
    fn reflect(&self, n: &Vec3) -> Vec3 {
        *self - *n * 2.0 * self.dot(n)
    }

    fn refract(&self, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min((-*self).dot(n), 1.0);
        let r_out_perp = (*self + *n * cos_theta) * etai_over_etat;
        let r_out_parallel = *n * -(1.0 - r_out_perp.norm_squared()).abs().sqrt();
        r_out_perp + r_out_parallel
    }

    fn near_zero(&self) -> bool {
        const EPS: f64 = 1e-8;
        self.x.abs() < EPS && self.y.abs() < EPS && self.z.abs() < EPS
    }
}
