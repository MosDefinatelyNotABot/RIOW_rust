mod camera;
mod ray;
mod hittable;
mod sphere;
mod material;

use std::f32::consts::PI;
use std::sync::Arc;
use ultraviolet::Vec3;
use crate::camera::{Camera, CameraSetup};
use crate::hittable::HittableList;
use crate::sphere::Sphere;

fn main() {

    println!("Ray Tracing in One Weekend.\n\
              ===========================");

    // scene setup
    let mut world = HittableList::new();

    let mat_ground = Arc::new(material::Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let mat_center = Arc::new(material::Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let mat_left = Arc::new(material::Dielectric::new(1.50 ));
    let mat_bubble = Arc::new(material::Dielectric::new(1.00 / 1.50 ));
    let mat_right = Arc::new(material::Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0));

    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, mat_ground)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, mat_center)));
    world.add(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, mat_left)));
    world.add(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.4, mat_bubble)));
    world.add(Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, mat_right)));

    // camera setup
    let camera_setup = CameraSetup::new(
        480,                            // image height
        16.0 / 9.0,                     // image aspect ratio
        32,                             // samples per pixel
        32,                             // max ray bounce depth
        1.0,                            // focal length
        20.0_f32.to_radians(),                       // vertical field of view
        Vec3::new(-2.0, 2.0, 1.0), // look from position
        Vec3::new(0.0, 0.0, -1.0),  // look at position
        Vec3::new(0.0, 1.0, 0.0),  // vertical up Vec
    );

    let mut camera_obj = Camera::init(&camera_setup);

    // render scene
    camera_obj.render(&world);

    // save rendered image to file
    camera_obj.save(Some("chap12.2.2.png"));

}

