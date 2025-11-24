use std::path::PathBuf;
use image::{RgbImage, Rgb};
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use ultraviolet::Vec3;
use itertools::iproduct;
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::hittable::{Hittable, HittableList};
use crate::ray::Ray;
use rayon::prelude::*;

pub struct Camera {
    width: u32,
    height: u32,
    aspect_ratio: f32,
    px_samples: u32,
    px_samples_scale: f32,
    max_ray_depth: u32,
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

    pub fn init(height: u32, aspect: f32, samples_per_px: u32, max_depth: u32) -> Self {

        // camera setup
        let width = (height as f32 * aspect) as u32;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect;
        let focal = 1.0;

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
            max_ray_depth: max_depth,
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

            let mut col: Vec3 = (0..self.px_samples).into_par_iter() // for each pixel sample
                .map(|_| Camera::ray_colour(&self.get_ray(x, y), self.max_ray_depth, world)) // calc the pixel colour
                .sum::<Vec3>() * self.px_samples_scale;

            col.apply(|c| c.sqrt().clamp( 0.000, 0.999) * 255.999 );

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
        pg_bar.set_message("rendering image...");

        pg_bar

    }

    fn ray_colour(ray: &Ray, depth: u32, world: &HittableList) -> Vec3 {

        // max depth termination
        if depth <= 0 { return Vec3::new(0.0, 0.0, 0.0); }

        let rec_option = world.hit(ray, 0.001, f32::INFINITY) ;

        if rec_option.is_some() {
            // calculate the surface colour of the sphere if it's hit
            let rec = rec_option.unwrap();
            let mat_option = rec.material.scatter(ray, &rec);

            return if mat_option.is_some() {
                let (scatter_dir, attenuation) = mat_option.unwrap();
                attenuation * Self::ray_colour(&scatter_dir, depth - 1, world)
            } else {
                Vec3::new(0.0, 0.0, 0.0)
            }

        }

        let unit_dir = ray.direction.normalized();
        let a = 0.5 * (unit_dir.y + 1.0);
        let out = (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0);

        out

    }

    fn get_ray(&self, u: u32, v: u32) -> Ray {
        // generates a ray originating from the camera center directed at a randomly sampled
        // point centered at pixel i j

        let mut rng = ThreadRng::default();

        let offset: (f32, f32) = (
            rng.random_range(-0.5..0.5),
            rng.random_range(-0.5..0.5));

        let px_sample = self.px_loc_100
            + ( (u as f32 + offset.0) * self.px_delta_u)
            + ( (v as f32 + offset.1) * self.px_delta_v);

        Ray::new(&self.origin, &(px_sample - self.origin))

    }

    fn random_on_hemisphere(normal: &Vec3) -> Vec3 {

        let rand_vec = random_unit_vec();
        if normal.dot(rand_vec) > 0.0 { rand_vec } else { -rand_vec }

    }

}

pub fn random_unit_vec() -> Vec3 {

    let mut rng = ThreadRng::default();

    // loop {
    //     let rand_vec = Vec3::new(
    //         rng.random_range(-1.0..1.0) as f32,
    //         rng.random_range(-1.0..1.0) as f32,
    //         rng.random_range(-1.0..1.0) as f32);
    //
    //     // let len_sq = rand_vec.mag_sq();
    //     //
    //     // if 1e-160 < len_sq && len_sq < 1.0 {
    //     //     return rand_vec.normalized();
    //     // }
    //
    //     return rand_vec.normalized();
    // }

    Vec3::new(
        rng.random_range(-1.0..1.0) as f32,
        rng.random_range(-1.0..1.0) as f32,
        rng.random_range(-1.0..1.0) as f32)
        .normalized()


}


