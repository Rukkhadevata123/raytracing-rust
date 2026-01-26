use crate::core::interval::Interval;
use crate::core::ray::Ray;
use crate::core::vec3::{Point3, Vec3};
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let mut aabb = Self { x, y, z };
        aabb.pad_to_minimums();
        aabb
    }

    pub fn new_point(a: Point3, b: Point3) -> Self {
        let x = if a.x <= b.x {
            Interval::new(a.x, b.x)
        } else {
            Interval::new(b.x, a.x)
        };
        let y = if a.y <= b.y {
            Interval::new(a.y, b.y)
        } else {
            Interval::new(b.y, a.y)
        };
        let z = if a.z <= b.z {
            Interval::new(a.z, b.z)
        } else {
            Interval::new(b.z, a.z)
        };

        Self::new(x, y, z)
    }

    pub fn empty() -> Self {
        Self {
            x: Interval::empty(),
            y: Interval::empty(),
            z: Interval::empty(),
        }
    }

    pub fn pad_to_minimums(&mut self) {
        const DELTA: f64 = 0.0001;
        if self.x.size() < DELTA {
            self.x = self.x.expand(DELTA);
        }
        if self.y.size() < DELTA {
            self.y = self.y.expand(DELTA);
        }
        if self.z.size() < DELTA {
            self.z = self.z.expand(DELTA);
        }
    }

    pub fn axis_interval(&self, axis: usize) -> Interval {
        match axis {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Invalid axis index"),
        }
    }

    pub fn hit(&self, ray: &Ray, mut ray_t: Interval) -> bool {
        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let ray_dir = match axis {
                0 => ray.dir.x,
                1 => ray.dir.y,
                2 => ray.dir.z,
                _ => unreachable!(),
            };
            let ray_orig = match axis {
                0 => ray.orig.x,
                1 => ray.orig.y,
                2 => ray.orig.z,
                _ => unreachable!(),
            };

            let adinv = 1.0 / ray_dir;
            let t0 = (ax.min - ray_orig) * adinv;
            let t1 = (ax.max - ray_orig) * adinv;

            let (t_min, t_max) = if adinv < 0.0 { (t1, t0) } else { (t0, t1) };

            if t_min > ray_t.min {
                ray_t.min = t_min;
            }
            if t_max < ray_t.max {
                ray_t.max = t_max;
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }

    pub fn merge(&self, other: &Self) -> Self {
        Self {
            x: self.x.merge(&other.x),
            y: self.y.merge(&other.y),
            z: self.z.merge(&other.z),
        }
    }
}

// Add vector offset
impl Add<Vec3> for Aabb {
    type Output = Self;
    fn add(self, offset: Vec3) -> Self::Output {
        Self {
            x: Interval::new(self.x.min + offset.x, self.x.max + offset.x),
            y: Interval::new(self.y.min + offset.y, self.y.max + offset.y),
            z: Interval::new(self.z.min + offset.z, self.z.max + offset.z),
        }
    }
}

impl Default for Aabb {
    fn default() -> Self {
        Self::empty()
    }
}
