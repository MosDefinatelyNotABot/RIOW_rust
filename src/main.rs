mod camera;
mod ray;
mod hittable;
mod sphere;

use crate::camera::{Camera};

fn main() {

    println!("Hello, world!");

    let mut frame_obj = Camera::init(
        720,            // height 720p
        1.7777778,     // aspect ratio 16:9
        1.0             // focal length 1.0 in camera space
    );

    frame_obj.render();

    frame_obj.save("test.png");

}

