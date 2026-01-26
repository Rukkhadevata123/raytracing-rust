use crate::core::aabb::Aabb;
use crate::core::interaction::Interaction;
use crate::core::interval::Interval;
use crate::core::ray::Ray;
use crate::geometry::hittable::Hittable;
use crate::geometry::hittable_list::HittableList;
use crate::sampling::random::random_int_range;
use std::cmp::Ordering;
use std::sync::Arc;

#[derive(Debug)]
pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(list: &HittableList) -> Self {
        Self::new_from_objects(list.objects.clone())
    }

    pub fn new_from_objects(mut objects: Vec<Arc<dyn Hittable>>) -> Self {
        let axis = random_int_range(0, 2) as usize;
        let comparator =
            |a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>| Self::box_compare(a, b, axis);

        let object_span = objects.len();

        let (left, right) = if object_span == 1 {
            (objects[0].clone(), objects[0].clone())
        } else if object_span == 2 {
            if comparator(&objects[0], &objects[1]) == Ordering::Less {
                (objects[0].clone(), objects[1].clone())
            } else {
                (objects[1].clone(), objects[0].clone())
            }
        } else {
            objects.sort_by(comparator);
            let mid = object_span / 2;
            let (left_objs, right_objs) = objects.split_at(mid);
            (
                Arc::new(Self::new_from_objects(left_objs.to_vec())) as Arc<dyn Hittable>,
                Arc::new(Self::new_from_objects(right_objs.to_vec())) as Arc<dyn Hittable>,
            )
        };

        let bbox = left.bounding_box().merge(&right.bounding_box());

        Self { left, right, bbox }
    }

    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> Ordering {
        let box_a = a.bounding_box();
        let box_b = b.bounding_box();

        box_a
            .axis_interval(axis)
            .min
            .partial_cmp(&box_b.axis_interval(axis).min)
            .unwrap_or(Ordering::Equal)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: Interval, isect: &mut Interaction) -> bool {
        if !self.bbox.hit(r, ray_t) {
            return false;
        }

        let hit_left = self.left.hit(r, ray_t, isect);

        let t_max = if hit_left { isect.t } else { ray_t.max };
        let right_interval = Interval::new(ray_t.min, t_max);

        let mut right_isect = Interaction::default();
        let hit_right = self.right.hit(r, right_interval, &mut right_isect);

        if hit_right {
            *isect = right_isect;
            return true;
        }

        hit_left
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
