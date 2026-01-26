use crate::core::camera::Camera;
use crate::core::vec3::{Color, Point3, Vec3};
use crate::geometry::hittable_list::HittableList;
use crate::geometry::quad;
use crate::geometry::quad::Quad;
use crate::geometry::sphere::Sphere;
use crate::geometry::transforms::rotate::RotateY;
use crate::geometry::transforms::translate::Translate;
use crate::materials::dielectric::Dielectric;
use crate::materials::diffuse_light::DiffuseLight;
use crate::materials::lambertian::Lambertian;
use crate::textures::solid_color::SolidColor;
use std::sync::Arc;

pub fn build_cornell_box(
    image_width: u32,
    samples: u32,
    max_depth: u32,
) -> (Arc<HittableList>, Arc<HittableList>, Camera) {
    let mut world = HittableList::new();
    let mut lights = HittableList::new();

    // Materials
    let red_mat = Arc::new(Lambertian::new(Arc::new(SolidColor::new_rgb(
        0.65, 0.05, 0.05,
    ))));
    let white_mat = Arc::new(Lambertian::new(Arc::new(SolidColor::new_rgb(
        0.73, 0.73, 0.73,
    ))));
    let green_mat = Arc::new(Lambertian::new(Arc::new(SolidColor::new_rgb(
        0.12, 0.45, 0.15,
    ))));
    let light_mat = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new_rgb(
        15.0, 15.0, 15.0,
    ))));

    // Cornell Box Walls
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 555.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(-555.0, 0.0, 0.0),
        white_mat.clone(),
    ))); // Back (Green side in C++ logic? No, check coords)
    // C++: green(555,0,0) -> (0,0,555) -> (0,555,0).
    // Rust Quads here match original implementation, assuming they form a box.
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        red_mat.clone(),
    ))); // Right
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 555.0),
        Vec3::new(0.0, 0.0, -555.0),
        Vec3::new(0.0, 555.0, 0.0),
        green_mat.clone(),
    ))); // Left
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 555.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white_mat.clone(),
    ))); // Top
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white_mat.clone(),
    ))); // Bottom

    // Light
    let light = Arc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light_mat.clone(),
    ));
    world.add(light.clone());
    lights.add(light.clone());

    // Objects Match Book 3 "Cornell Box with Glass Sphere" cover

    // Box 1 (Rotated & Translated)
    let box1 = quad::box_new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white_mat.clone(),
    );
    let box1_rot = Arc::new(RotateY::new(Arc::new(box1), 15.0));
    let box1_trans = Arc::new(Translate::new(box1_rot, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1_trans);

    // Glass Sphere
    let glass_mat = Arc::new(Dielectric::new(1.5));
    let glass_sphere = Arc::new(Sphere::new(
        Point3::new(190.0, 90.0, 190.0),
        90.0,
        glass_mat,
    ));
    world.add(glass_sphere.clone());

    // Add glass sphere to lights for importance sampling (Book 3 technique for caustics)
    lights.add(glass_sphere);

    // Camera Setup
    let mut cam = Camera::new(image_width, 1.0);
    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    cam.samples_per_pixel = samples;
    cam.max_depth = max_depth;
    cam.background = Color::zeros();

    cam.initialize();

    (Arc::new(world), Arc::new(lights), cam)
}
