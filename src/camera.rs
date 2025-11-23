use std::path::PathBuf;
use image::{RgbImage, Rgb};
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use ultraviolet::Vec3;
use itertools::iproduct;
use crate::hittable::{Hittable, HittableList};
use crate::ray::Ray;
use crate::sphere::Sphere;

pub struct Camera {
    width: u32,
    height: u32,
    aspect_ratio: f32,
    image: RgbImage,

    focal_length: f32,
    viewport_height: f32,
    viewport_width: f32,

    origin: Vec3,
    viewport_u: Vec3,
    viewport_v: Vec3,

    px_delta_u: Vec3,
    px_delta_v: Vec3,
    px_loc_100: Vec3,

    world: HittableList
}

impl Camera {

    pub fn init(height: u32, aspect: f32, focal: f32) -> Self {

        // camera setup
        let width = (height as f32 * aspect) as u32;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect;

        let origin = Vec3::new(0.0, 0.0, 0.0);
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let px_delta_u = viewport_u / (width as f32);
        let px_delta_v = viewport_v / (height as f32);

        let viewport_up_left = origin
            - Vec3::new(0.0, 0.0, focal)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        let px_loc_100 = viewport_up_left + 0.5 * (px_delta_u + px_delta_v);

        // Setup the scene
        let mut world = HittableList::new();

        world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
        world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

        Camera{
            width,
            height,
            aspect_ratio: aspect,
            image: image::RgbImage::new(width, height),
            focal_length: focal,

            viewport_height,
            viewport_width,

            origin: Vec3::new(0.0, 0.0, 0.0),
            viewport_u,
            viewport_v,

            px_delta_u,
            px_delta_v,
            px_loc_100,

            world,
        }

    }

    pub fn render(&mut self) {

        let pg_bar = ProgressBar::new(self.height as u64 * self.width as u64);
        pg_bar.set_style(
            ProgressStyle::with_template("elapsed: [{elapsed}] eta: [{eta_precise}] {bar:50.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("##-"));
        pg_bar.set_message("rendering frame...");

        for (x, y) in iproduct!((0..self.width).into_iter(), (0..self.height).into_iter()) {

            let px_center = self.px_loc_100 + (self.px_delta_u * x as f32) + (self.px_delta_v * y as f32);
            let ray_dir = px_center - self.origin;
            let ray: Ray = Ray::new(&self.origin, &ray_dir);

            let col: Rgb<u8> = ray_colour(&ray, &self.world);

            self.image.put_pixel(x, y, col);
            pg_bar.inc(1);

        }

        pg_bar.finish_with_message("done!");

    }

    pub fn save(&self, filename: &str) {

        let mut out_dir = PathBuf::new();
        out_dir.push("output_images");

        if !out_dir.exists() {
            std::fs::create_dir(&out_dir).unwrap();
        }

        out_dir = out_dir.join(filename);
        out_dir.set_extension("png");

        self.image.save(&out_dir).unwrap_or_else(|e| println!("Error saving image {e}"));

        println!("Saved rendered image to {}", &out_dir.to_str().unwrap());

    }

}


pub fn ray_colour(ray: &Ray, world: &HittableList) -> Rgb<u8> {

    let t = world.hit(ray, 0.0, f32::INFINITY) ;

    if t.is_some() {
        // calculate the surface normal of the sphere if it's hit

        let mut normal: Vec3  = 0.5 * (t.unwrap().normal + Vec3::new(1.0, 1.0, 1.0));
        normal = 255.0 * normal; // convert to rgb values

        return Rgb([normal.x as u8, normal.y as u8, normal.z as u8]);

    }

    let unit_dir = ray.direction.normalized();
    let a = 0.5 * (unit_dir.y + 1.0);

    let out = (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0);

    Rgb([(out.x * 255.0) as u8, (out.y * 255.0) as u8, (out.z * 255.0) as u8])

}