mod camera;
mod ray;
mod hittable;
mod sphere;
mod material;

use std::sync::Arc;
use ultraviolet::Vec3;
use crate::camera::{Camera};
use crate::hittable::HittableList;
use crate::sphere::Sphere;

fn main() {

    println!("Ray Tracing in One Weekend.\n\
              ===========================");

    // camera setup
    let mut frame_obj = Camera::init(
        480,            // height 720p
        1.7777778,     // aspect ratio 16:9
        32,
        32
    );

    // scene setup
    let mut world = HittableList::new();

    let mat_ground = Arc::new(material::Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let mat_center = Arc::new(material::Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let mat_left = Arc::new(material::Dielectric::new(1.5 ));
    let mat_right = Arc::new(material::Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0));

    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, mat_ground)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, mat_center)));
    world.add(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, mat_left)));
    world.add(Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, mat_right)));

    // render scene
    frame_obj.render(&world);

    // save rendered image to file
    frame_obj.save("chap11.2.png");

}

