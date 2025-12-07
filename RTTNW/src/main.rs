mod camera;
mod ray;
mod hittable;
mod sphere;
mod material;

use crate::camera::{random_unit_vec, Camera, CameraSetup};
use crate::hittable::HittableList;
use crate::material::{Lambertian, Metal, Dielectric, Material};
use crate::sphere::{MovingSphere, Sphere};
use rand::{random, random_range, SeedableRng};
use std::ops::{Div, Mul};
use std::sync::Arc;
use itertools::iproduct;
use ultraviolet::Vec3;

fn main() {

    println!("Ray Tracing The Next Weekend.\n\
              =============================");

    // scene setup
    let world = final_render_scene();

    // camera setup
    let camera_setup = CameraSetup::new(
        480,                        // image height
        16.0 / 9.0,                 // image aspect ratio
        128,                        // samples per pixel
        32,                         // max ray bounce depth
        20.0_f32.to_radians(),      // vertical field of view
        Vec3::new(13.0, 2.0, 3.0),  // look from position
        Vec3::new(0.0, 0.0, 0.0),   // look at position
        Vec3::new(0.0, 1.0, 0.0),   // vertical up Vec
        0.6_f32.to_radians(),       // defocus angle
        10.0                        // focus distance
    );

    let mut camera_obj = Camera::init(&camera_setup);

    // render scene
    camera_obj.render(&world);

    // save rendered image to file
    camera_obj.save(Some("test.png"));

}

fn final_render_scene() -> HittableList {
    // setup for the final render scene
    let mut scene = HittableList::new();

    let ground_mat = Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    scene.add(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat)));

    for (a, b) in iproduct!(-11..11, -11..11) {

        let choose_mat: f32 = random();
        let center = Vec3::new(
            (a as f32) + 0.9 * random::<f32>(),
            0.2,
            (b as f32) + 0.9 * random::<f32>());

        let sphere_material: Arc<dyn Material> = if choose_mat < 0.8 {
            // diffuse/matte material
            let colour = random_unit_vec().mul(random_unit_vec());
            Arc::new(Lambertian::new(colour))

        } else if choose_mat < 0.95 {
            // metal
            let colour = Vec3::new(
                random_range(0.5..1.0),
                random_range(0.5..1.0),
                random_range(0.5..1.0)
            );
            let fuzz = random_range(0.0..0.5);
            Arc::new(Metal::new(colour, fuzz))

        } else {
            // dielectric material
            Arc::new(Dielectric::new(1.5))
        };

        if choose_mat < 0.8 {
            let center_1 = center + Vec3::new(0.0, random_range(0.0..0.5), 0.0);
            scene.add(Box::new(MovingSphere::new(center, center_1, 0.2, sphere_material)))

        } else {
            scene.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
        }

    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    let mat2 = Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    let mat3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));

    scene.add(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, mat1)));
    scene.add(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, mat2)));
    scene.add(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, mat3)));

    scene

}

