use crate::core::aabb::Aabb;
use crate::core::interaction::Interaction;
use crate::core::interval::Interval;
use crate::core::ray::Ray;
use crate::core::vec3::{Point3, Vec3};
use crate::geometry::hittable::Hittable;
use crate::sampling::random::random_int_range;
use std::sync::Arc;

#[derive(Default, Debug)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bbox: Aabb,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: Aabb::empty(),
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.bbox = self.bbox.merge(&object.bounding_box());
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, isect: &mut Interaction) -> bool {
        let mut temp_isect = Interaction::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_isect) {
                hit_anything = true;
                closest_so_far = temp_isect.t;
                *isect = temp_isect.clone();
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;
        self.objects
            .iter()
            .map(|obj| weight * obj.pdf_value(origin, direction))
            .sum()
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        let int_size = self.objects.len();
        if int_size == 0 {
            return Vec3::new(1.0, 0.0, 0.0);
        }

        let index = random_int_range(0, (int_size - 1) as i32) as usize;
        self.objects[index].random(origin)
    }
}
