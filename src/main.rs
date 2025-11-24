mod camera;
mod ray;
mod hittable;
mod sphere;

use ultraviolet::Vec3;
use crate::camera::{Camera};
use crate::hittable::HittableList;
use crate::sphere::Sphere;

fn main() {

    println!("Ray Tracing in One Weekend.\n\
              ===========================\n");

    // camera setup
    let mut frame_obj = Camera::init(
        720,            // height 720p
        1.7777778,     // aspect ratio 16:9
        64,
        1.0             // focal length 1.0 in camera space
    );

    // scene setup
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    // render scene
    frame_obj.render(&world);

    // save rendered image to file
    frame_obj.save("test.png");

}

