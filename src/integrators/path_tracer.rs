use crate::core::camera::Camera;
use crate::core::interaction::Interaction;
use crate::core::interval::Interval;
use crate::core::ray::Ray;
use crate::core::vec3::Color;
use crate::geometry::hittable::Hittable;
use crate::integrators::integrator_trait::Integrator;
use crate::materials::material_trait::ScatterRecord;
use crate::sampling::pdf::{HittablePDF, MixturePDF, PDF};
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::sync::Arc;

pub struct PathTracer {
    output_filename: String,
}

impl PathTracer {
    pub fn new(output_filename: &str) -> Self {
        Self {
            output_filename: output_filename.to_string(),
        }
    }

    /// Li (Incoming Light)
    fn li(
        &self,
        ray: &Ray,
        depth: u32,
        world: &dyn Hittable,
        lights: Option<&Arc<dyn Hittable>>,
        background: &Color,
    ) -> Color {
        // Stop recursion
        if depth == 0 {
            return Color::zeros();
        }

        let mut isect = Interaction::default();

        // Ray intersection test
        if !world.hit(ray, Interval::new(0.001, f64::INFINITY), &mut isect) {
            return *background;
        }

        let material = match &isect.material {
            Some(m) => m,
            None => return Color::new(1.0, 0.0, 1.0),
        };

        let emission = material.emitted(ray, &isect, isect.uv.0, isect.uv.1, &isect.p);

        let mut srec = ScatterRecord::default();
        if !material.scatter(ray, &isect, &mut srec) {
            return emission;
        }

        if srec.skip_pdf {
            return emission
                + srec.attenuation.component_mul(&self.li(
                    &srec.skip_pdf_ray,
                    depth - 1,
                    world,
                    lights,
                    background,
                ));
        }

        let p: Arc<dyn PDF> = if let Some(light_objects) = lights {
            let light_pdf = Arc::new(HittablePDF::new(light_objects.clone(), isect.p));
            let mat_pdf = srec.pdf_ptr.unwrap();
            Arc::new(MixturePDF::new(light_pdf, mat_pdf))
        } else {
            srec.pdf_ptr.unwrap()
        };

        let scattered_direction = p.generate();
        let scattered_ray = Ray::new(isect.p, scattered_direction, ray.time);

        let pdf_val = p.value(&scattered_direction);

        if pdf_val < 1e-5 {
            return emission;
        }

        let scattering_pdf = material.scattering_pdf(ray, &isect, &scattered_ray);

        let sample_color = self.li(&scattered_ray, depth - 1, world, lights, background);

        emission + srec.attenuation.component_mul(&sample_color) * scattering_pdf / pdf_val
    }

    fn calculate_pixel_color(
        &self,
        i: u32,
        j: u32,
        world: &dyn Hittable,
        lights: Option<&Arc<dyn Hittable>>,
        camera: &Camera,
    ) -> Color {
        let mut pixel_color = Color::zeros();
        for _ in 0..camera.samples_per_pixel {
            let r = camera.get_ray(i, j);
            let sample_color = self.li(&r, camera.max_depth, world, lights, &camera.background);

            if sample_color.x.is_finite()
                && sample_color.y.is_finite()
                && sample_color.z.is_finite()
            {
                pixel_color += sample_color;
            }
        }
        pixel_color
    }
}

impl Integrator for PathTracer {
    fn render(&self, world: &dyn Hittable, lights: Option<Arc<dyn Hittable>>, camera: &Camera) {
        let width = camera.image_width;
        let height = camera.image_height;
        let mut img: RgbImage = ImageBuffer::new(width, height);

        println!(
            "Rendering {}x{} image with {} SPP...",
            width, height, camera.samples_per_pixel
        );

        let progress_bar = ProgressBar::new((width * height) as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
                )
                .unwrap()
                .progress_chars("#>-"),
        );

        // Tile-based rendering for parallel efficiency and progress updates
        let tile_size = 16;
        let num_tiles_x = width.div_ceil(tile_size);
        let num_tiles_y = height.div_ceil(tile_size);
        let total_tiles = num_tiles_x * num_tiles_y;

        let start_time = std::time::Instant::now();

        let render_results: Vec<(u32, u32, Rgb<u8>)> = (0..total_tiles)
            .into_par_iter()
            .flat_map(|tile_idx| {
                let tile_x = (tile_idx % num_tiles_x) * tile_size;
                let tile_y = (tile_idx / num_tiles_x) * tile_size;

                let mut tile_pixels = Vec::new();

                for j in tile_y..std::cmp::min(tile_y + tile_size, height) {
                    for i in tile_x..std::cmp::min(tile_x + tile_size, width) {
                        let color =
                            self.calculate_pixel_color(i, j, world, lights.as_ref(), camera);
                        tile_pixels.push((i, j, color_to_rgb(color, camera.samples_per_pixel)));
                        progress_bar.inc(1);
                    }
                }
                tile_pixels
            })
            .collect();

        progress_bar.finish_with_message("Done");
        println!("Render complete in {:.2?}", start_time.elapsed());

        for (i, j, pixel) in render_results {
            img.put_pixel(i, j, pixel);
        }

        match img.save(&self.output_filename) {
            Ok(_) => println!("Image saved to {}", self.output_filename),
            Err(e) => eprintln!("Error saving image: {}", e),
        }
    }
}

fn color_to_rgb(color: Color, samples_per_pixel: u32) -> Rgb<u8> {
    let scale = 1.0 / samples_per_pixel as f64;
    let r = (linear_to_gamma(color.x * scale)).clamp(0.0, 0.999);
    let g = (linear_to_gamma(color.y * scale)).clamp(0.0, 0.999);
    let b = (linear_to_gamma(color.z * scale)).clamp(0.0, 0.999);

    Rgb([(r * 256.0) as u8, (g * 256.0) as u8, (b * 256.0) as u8])
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}
