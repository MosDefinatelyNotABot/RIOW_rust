use std::path::PathBuf;
use image::{RgbImage, Rgb};
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use ultraviolet::Vec3;
use itertools::iproduct;
use crate::hittable::{Hittable, HittableList};
use crate::ray::Ray;

pub struct Camera {
    width: u32,
    height: u32,
    aspect_ratio: f32,
    px_samples: u32,
    px_samples_scale: f32,
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
}

impl Camera {

    pub fn init(height: u32, aspect: f32, samples_per_px: u32, focal: f32) -> Self {

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

        Camera{

            width,
            height,
            aspect_ratio: aspect,
            px_samples: samples_per_px,
            px_samples_scale: 1.0 / (samples_per_px as f32),
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

        }

    }

    pub fn render(&mut self, world: &HittableList) {

        let pg_bar = self.setup_pg_bar(); // setup progress bar

        for (x, y) in iproduct!((0..self.width).into_iter(), (0..self.height).into_iter()) {

            let col: Vec3 = (0..self.px_samples)                       // for each pixel sample
                .map(|_| Camera::ray_colour(&self.get_ray(x, y), world)) // calc the pixel colour
                .sum::<Vec3>() * self.px_samples_scale;                  // then accumulate and scale.

            self.image.put_pixel(x, y, Rgb([col.x as u8, col.y as u8, col.z as u8]));
            pg_bar.inc(1);

        }

        pg_bar.finish_with_message("done!");

    }

    pub fn save(&self, filename: &str) {
        // saves the previously rendered image

        let mut out_dir = PathBuf::new();
        out_dir.push("output_images");

        if !out_dir.exists() {
            let res = std::fs::create_dir(&out_dir);
            res.unwrap_or_else(|e| println!("Error creating output directory: {e}"));
        }

        out_dir = out_dir.join(filename);
        out_dir.set_extension("png");

        self.image.save(&out_dir).unwrap_or_else(|e| println!("Error saving image: {e}"));

        println!("Saved rendered image to {}", &out_dir.to_str().unwrap());

    }

    fn setup_pg_bar(&self) -> ProgressBar {
        // set up the progress bar...

        let pg_bar = ProgressBar::new(self.height as u64 * self.width as u64);
        pg_bar.set_style(
            ProgressStyle::with_template("elapsed: [{elapsed}] eta: [{eta_precise}] {bar:50.cyan/blue} {pos:>7}/{len:7} {msg}")
                .unwrap()
                .progress_chars("##-"));
        pg_bar.set_message("rendering frame...");

        pg_bar

    }

    fn ray_colour(ray: &Ray, world: &HittableList) -> Vec3 {

        let t = world.hit(ray, 0.0, f32::INFINITY) ;

        if t.is_some() {
            // calculate the surface normal of the sphere if it's hit

            let normal: Vec3  = 0.5 * (t.unwrap().normal + Vec3::new(1.0, 1.0, 1.0));
            return 255.0 * normal; // convert to rgb values

        }

        let unit_dir = ray.direction.normalized();
        let a = 0.5 * (unit_dir.y + 1.0);

        let out = (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0);

        255.0 * out

    }

    fn get_ray(&self, u: u32, v: u32) -> Ray {
        // generates a ray originating from the camera center directed at a randomly sampled
        // point centered at pixel i j

        let offset: (f32, f32) = rand::random();

        let px_sample = self.px_loc_100
            + ( (u as f32 + offset.0) * self.px_delta_u)
            + ( (v as f32 + offset.1) * self.px_delta_v);

        Ray::new(&self.origin, &(px_sample - self.origin))

    }

}


