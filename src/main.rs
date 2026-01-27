mod core;
mod geometry;
mod integrators;
mod materials;
mod sampling;
mod scenes;
mod textures;

use crate::geometry::hittable::Hittable;
use crate::integrators::integrator_trait::Integrator;
use crate::integrators::path_tracer::PathTracer;
use crate::scenes::{cornell_box, final_scene, many_balls};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let scene_name = args.get(1).map(String::as_str).unwrap_or("many_balls");

    let (world, lights, camera) = match scene_name {
        "many_balls" => {
            println!("Loading Book 1 Final Scene (Random Spheres)...");
            many_balls::build_many_balls(1200, 10000, 75)
        }
        "cornell_box" => {
            println!("Loading Book 3 Cornell Box (Glass Sphere)...");
            cornell_box::build_cornell_box(1200, 10000, 75)
        }
        "final_scene" => {
            println!("Loading Book 2 Final Scene...");
            // High resolution render settings from book
            final_scene::build_final_scene(1200, 10000, 75)
        }
        _ => {
            eprintln!(
                "Unknown scene '{}'. Available: many_balls, cornell_box, final_scene",
                scene_name
            );
            return;
        }
    };

    let filename = format!("{}.png", scene_name);
    let integrator = PathTracer::new(&filename);

    let lights_opt = if lights.objects.is_empty() {
        None
    } else {
        Some(lights as std::sync::Arc<dyn Hittable>)
    };

    integrator.render(&*world, lights_opt, &camera);
}
