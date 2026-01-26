use crate::core::camera::Camera;
use crate::geometry::hittable::Hittable;
use std::sync::Arc;

pub trait Integrator: Send + Sync {
    /// Renders the scene.
    fn render(&self, world: &dyn Hittable, lights: Option<Arc<dyn Hittable>>, camera: &Camera);
}
