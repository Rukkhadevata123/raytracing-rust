use crate::core::camera::Camera;
use crate::core::vec3::{Color, Point3, Vec3, Vec3Ext};
use crate::geometry::bvh::BvhNode;
use crate::geometry::constant_medium::ConstantMedium;
use crate::geometry::hittable_list::HittableList;
use crate::geometry::quad;
use crate::geometry::quad::Quad;
use crate::geometry::sphere::Sphere;
use crate::geometry::transforms::rotate::RotateY;
use crate::geometry::transforms::translate::Translate;
use crate::materials::dielectric::Dielectric;
use crate::materials::diffuse_light::DiffuseLight;
use crate::materials::lambertian::Lambertian;
use crate::materials::metal::Metal;
use crate::sampling::random::random_double_range;
use crate::textures::image::ImageTexture;
use crate::textures::noise::NoiseTexture;
use crate::textures::solid_color::SolidColor;
use std::sync::Arc;

pub fn build_final_scene(
    image_width: u32,
    samples: u32,
    max_depth: u32,
) -> (Arc<HittableList>, Arc<HittableList>, Camera) {
    let mut world = HittableList::new();
    let mut lights = HittableList::new();

    // Ground Boxes
    let ground = Arc::new(Lambertian::new(Arc::new(SolidColor::new_rgb(
        0.48, 0.83, 0.53,
    ))));
    let mut boxes1 = HittableList::new();
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_range(1.0, 101.0);
            let z1 = z0 + w;

            let box_instance = quad::box_new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            );
            boxes1.add(Arc::new(box_instance));
        }
    }
    world.add(Arc::new(BvhNode::new(&boxes1)));

    // Light
    let light_mat = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new_rgb(
        7.0, 7.0, 7.0,
    ))));
    let light = Arc::new(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light_mat,
    ));
    world.add(light.clone());
    lights.add(light);

    // Moving Sphere
    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_mat = Arc::new(Lambertian::new(Arc::new(SolidColor::new_rgb(
        0.7, 0.3, 0.1,
    ))));
    world.add(Arc::new(Sphere::new_moving(
        center1, center2, 50.0, sphere_mat,
    )));

    // Glass and Metal Spheres
    world.add(Arc::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    // Subsurface Reflection (Blue glass sphere with volume)
    let boundary = Arc::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(boundary.clone());
    world.add(Arc::new(ConstantMedium::new(
        boundary,
        0.2,
        Arc::new(SolidColor::new_rgb(0.2, 0.4, 0.9)),
    )));

    // Global Fog
    let boundary2 = Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(Arc::new(ConstantMedium::new(
        boundary2,
        0.0001,
        Arc::new(SolidColor::new_rgb(1.0, 1.0, 1.0)),
    )));

    // Earth
    let earth_mat = Arc::new(Lambertian::new(Arc::new(ImageTexture::new("earthmap.jpg"))));
    world.add(Arc::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        earth_mat,
    )));

    // Noise
    let pertext = Arc::new(NoiseTexture::new(0.2));
    world.add(Arc::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::new(pertext)),
    )));

    // Cluster of spheres
    let mut boxes2 = HittableList::new();
    let white = Arc::new(Lambertian::new(Arc::new(SolidColor::new_rgb(
        0.73, 0.73, 0.73,
    ))));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Arc::new(Sphere::new(
            Vec3::random_range(0.0, 165.0).into(),
            10.0,
            white.clone(),
        )));
    }

    let boxes2_rot = Arc::new(RotateY::new(Arc::new(BvhNode::new(&boxes2)), 15.0));
    let boxes2_trans = Arc::new(Translate::new(boxes2_rot, Vec3::new(-100.0, 270.0, 395.0)));
    world.add(boxes2_trans);

    let mut cam = Camera::new(image_width, 1.0);
    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(478.0, 278.0, -600.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.samples_per_pixel = samples;
    cam.max_depth = max_depth;
    cam.initialize();

    (Arc::new(world), Arc::new(lights), cam)
}
