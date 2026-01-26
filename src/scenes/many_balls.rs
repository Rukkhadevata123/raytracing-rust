use crate::core::camera::Camera;
use crate::core::vec3::{Color, Point3, Vec3, Vec3Ext};
use crate::geometry::hittable_list::HittableList;
use crate::geometry::sphere::Sphere;
use crate::materials::dielectric::Dielectric;
use crate::materials::lambertian::Lambertian;
use crate::materials::metal::Metal;
use crate::sampling::random::random_double;
use crate::textures::solid_color;
use std::sync::Arc;

pub fn build_many_balls(
    image_width: u32,
    samples: u32,
    max_depth: u32,
) -> (Arc<HittableList>, Arc<HittableList>, Camera) {
    let mut world = HittableList::new();
    let lights = HittableList::new(); // empty for this scene

    let ground_material = Arc::new(Lambertian::new(Arc::new(solid_color::SolidColor::new_rgb(
        0.5, 0.5, 0.5,
    ))));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    // Use component_mul for element-wise multiplication
                    let albedo = Color::random().component_mul(&Color::random());
                    let sphere_material = Arc::new(Lambertian::new(Arc::new(
                        solid_color::SolidColor::new(albedo),
                    )));
                    let _center2 = center + Vec3::new(0.0, random_double() * 0.5, 0.0);
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_double() * 0.5;
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // Glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Arc::new(solid_color::SolidColor::new_rgb(
        0.4, 0.2, 0.1,
    ))));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let mut cam = Camera::new(image_width, 16.0 / 9.0);
    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;
    cam.samples_per_pixel = samples;
    cam.max_depth = max_depth;
    cam.background = Color::new(0.70, 0.80, 1.00);

    cam.initialize();

    (Arc::new(world), Arc::new(lights), cam)
}
