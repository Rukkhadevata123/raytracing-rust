use crate::core::vec3::Vec3;

/// Orthonormal Basis
#[derive(Debug, Clone, Copy)]
#[allow(clippy::upper_case_acronyms)]
pub struct ONB {
    axis: [Vec3; 3],
}

impl ONB {
    /// Constructs an ONB from a vector `w` (usually the normal).
    /// `w` does not need to be unit length but it's safer if it is.
    pub fn build_from_w(w: &Vec3) -> Self {
        let unit_w = w.normalize();
        let a = if unit_w.x.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };

        let v = unit_w.cross(&a).normalize();
        let u = unit_w.cross(&v);

        Self {
            axis: [u, v, unit_w],
        }
    }

    #[inline]
    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }

    #[inline]
    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }

    #[inline]
    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }

    /// Transforms a vector from local ONB coordinates to World coordinates.
    #[inline]
    pub fn local(&self, a: &Vec3) -> Vec3 {
        self.u() * a.x + self.v() * a.y + self.w() * a.z
    }
}
