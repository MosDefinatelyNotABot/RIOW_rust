use std::ops::Div;
use std::path::PathBuf;
use image::{RgbImage, Rgb};
use indicatif::{ProgressBar, ProgressIterator};
use indicatif::ProgressStyle;
use ultraviolet::Vec3;
use itertools::iproduct;
use rand::{random, random_range};
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use crate::hittable::{Hittable, HittableList};
use crate::ray::{random_in_unit_disk, Ray};
use rayon::prelude::*;

pub struct Camera {
    width: u32,
    height: u32,
    aspect_ratio: f32,
    px_samples: u32,
    px_samples_scale: f32,
    max_depth: u32,
    image: RgbImage,

    defocus_angle: f32,
    viewport_height: f32,
    viewport_width: f32,

    origin: Vec3,
    viewport_u: Vec3,
    viewport_v: Vec3,

    px_delta_u: Vec3,
    px_delta_v: Vec3,
    px_loc_100: Vec3,
    u: Vec3, v: Vec3, w: Vec3,

    defocus_disc_u: Vec3,
    defocus_disc_v: Vec3,

}

impl Camera {

    pub fn init(cam_setup: &CameraSetup) -> Self {

        // camera setup
        let width = (cam_setup.image_height as f32 * cam_setup.aspect_ratio) as u32;
        let h = cam_setup.vfov.div(2.0).tan();

        let viewport_height = 2.0 * h * cam_setup.focus_distance;
        let viewport_width = viewport_height * cam_setup.aspect_ratio;

        let w = (cam_setup.look_from - cam_setup.look_at).normalized();
        let u = cam_setup.vertical_up.cross(w).normalized();
        let v = w.cross(u);

        let viewport_u = viewport_width  *  u;
        let viewport_v = viewport_height * -v;

        let px_delta_u = viewport_u / (width as f32);
        let px_delta_v = viewport_v / (cam_setup.image_height as f32);

        let viewport_up_left = cam_setup.look_from
            - (cam_setup.focus_distance * w)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        let px_loc_100 = viewport_up_left + 0.5 * (px_delta_u + px_delta_v);

        let defocus_radius = cam_setup.focus_distance
            * cam_setup.defocus_angle.div(2.0).tan();

        let defocus_disc_u = u * defocus_radius;
        let defocus_disc_v = v * defocus_radius;

        Camera{

            width,
            height: cam_setup.image_height,
            aspect_ratio: cam_setup.aspect_ratio,
            px_samples: cam_setup.samples_per_px,
            px_samples_scale: 1.0 / (cam_setup.samples_per_px as f32),
            max_depth: cam_setup.max_depth,
            image: RgbImage::new(width, cam_setup.image_height),

            // focal_length: cam_setup.,
            defocus_angle: cam_setup.defocus_angle,
            viewport_height,
            viewport_width,

            origin: cam_setup.look_from,
            viewport_u,
            viewport_v,

            px_delta_u,
            px_delta_v,
            px_loc_100,
            u, v, w,

            defocus_disc_u,
            defocus_disc_v

        }

    }

    pub fn render(&mut self, world: &HittableList) {

        let pg_bar = self.setup_pg_bar(); // setup progress bar

        for (x, y) in iproduct!(
            (0..self.width).into_iter(),    // x-coord as the outer prod
            (0..self.height).into_iter())   // y-coord as the inner prod
            .progress_with(pg_bar) {

            let mut col: Vec3 = (0..self.px_samples).into_par_iter() // for each pixel sample
                // calc the pixel colour
                .map(|_| Camera::ray_colour(&self.get_ray(x, y), self.max_depth, world))
                // then accumulate and scale.
                .sum::<Vec3>() * self.px_samples_scale;

            // gamma correction
            col.apply( |x| if x > 0.0 { x.sqrt() } else { 0.0 } );
            // clamp
            col.apply( |x| x.clamp(0.000, 0.999) );
            // conversion to range 0-255
            col.apply( |x| (x * 255.0).round() );

            self.image.put_pixel(x, y, Rgb([col.x as u8, col.y as u8, col.z as u8]));

        }

    }

    fn ray_colour(ray: &Ray, depth: u32, world: &HittableList) -> Vec3 {

        if depth <= 0 { return Vec3::zero() }

        let rec_option = world.hit(ray, 0.001..f32::INFINITY) ;

        if rec_option.is_some() {
            // calculate the surface normal of the sphere if it's hit
            let rec = rec_option.unwrap();

            let scatter = rec.material.scatter(ray, &rec);

            return if scatter.is_some() {
                let (scattered, col) = scatter.unwrap();
                col * Self::ray_colour(&scattered, depth - 1, world)
            } else {
                Vec3::zero()
            }

        }

        let unit_dir = ray.direction.normalized();
        let a = 0.5 * (unit_dir.y + 1.0);

        let out = (1.0 - a) * Vec3::one() + a * Vec3::new(0.5, 0.7, 1.0);

        out

    }

    fn get_ray(&self, u: u32, v: u32) -> Ray {
        // generates a ray originating from the camera center directed at a randomly sampled
        // point centered at pixel i j

        let offset_x = random_range(-0.5..0.5);
        let offset_y = random_range(-0.5..0.5);

        let px_sample = self.px_loc_100
            + ( (u as f32 + offset_x) * self.px_delta_u)
            + ( (v as f32 + offset_y) * self.px_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.origin
        } else { self.defocus_disc_sample() };
        let ray_direction = (px_sample - ray_origin).normalized();

        let ray_time = random();

        Ray::new(ray_origin, ray_direction, ray_time)

    }

    pub fn save(&self, filename: Option<&str>) {
        // saves the previously rendered image

        let mut out_dir = PathBuf::new();
        out_dir.push("../RTTNW/output_images");

        if !out_dir.exists() {
            let res = std::fs::create_dir(&out_dir);
            res.unwrap_or_else(|e| println!("Error creating output directory: {e}"));
        }

        out_dir = out_dir.join(filename.unwrap_or("output"));
        out_dir.set_extension("png");

        self.image.save(&out_dir).unwrap_or_else(|e| println!("Error saving image: {e}"));

        println!("Saved rendered image to {}", &out_dir.to_str().unwrap());

    }

    fn setup_pg_bar(&self) -> ProgressBar {
        // set up the progress bar...

        let pg_bar = ProgressBar::new(self.height as u64 * self.width as u64);
        pg_bar.set_style(
            ProgressStyle::with_template("elapsed: [{elapsed}] {bar:50.cyan/blue} {percent:.bold.cyan/blue}% {msg}")
                .unwrap()
                .progress_chars("##-"));

        pg_bar.set_message("rendering frame...");

        pg_bar

    }

    fn random_on_hemisphere(normal: &Vec3) -> Vec3 {

        let rand_vec = random_unit_vec();
        if normal.dot(rand_vec) > 0.0 { rand_vec } else { -rand_vec }

    }

    fn defocus_disc_sample(&self) -> Vec3 {
        let offset = random_in_unit_disk();
        self.origin + ( offset.x * self.defocus_disc_u ) + ( offset.y * self.defocus_disc_v )
    }

}

pub fn random_unit_vec() -> Vec3 {

    loop {
        let rand_vec = Vec3::new(
            random_range(-1.0..1.0) as f32,
            random_range(-1.0..1.0) as f32,
            random_range(-1.0..1.0) as f32);

        let len_sq = rand_vec.mag_sq();

        if 1e-160 < len_sq && len_sq < 1.0 {
            return rand_vec.normalized();
        }

    }

}

pub struct CameraSetup {
    image_height: u32,
    aspect_ratio: f32,
    samples_per_px: u32,
    max_depth: u32,
    // focal_length: f32,
    vfov: f32, // in radians
    look_from: Vec3,
    look_at: Vec3,
    vertical_up: Vec3,
    defocus_angle: f32,
    focus_distance: f32
}

impl CameraSetup {
    // helper struct for initializing camera objects
    pub fn new(
        image_height: u32,
        aspect_ratio: f32,
        samples_per_px: u32,
        max_depth: u32,
        vertical_field_of_view: f32,
        look_from: Vec3,
        look_at: Vec3,
        vertical_up: Vec3,
        defocus_angle: f32,
        focus_distance: f32
    ) -> Self {
        CameraSetup {
            image_height,
            aspect_ratio,
            samples_per_px,
            max_depth,
            vfov: vertical_field_of_view,
            look_from,
            look_at,
            vertical_up,
            defocus_angle,
            focus_distance
        }
    }

    pub fn default() -> Self {
        CameraSetup {
            image_height: 720,
            aspect_ratio: 16.0 / 9.0,
            samples_per_px: 32,
            max_depth: 64,
            vfov: std::f32::consts::PI / 2.0,
            look_from: Vec3::new(0.0, 0.0, 0.0),
            look_at: Vec3::new(0.0, 0.0, -1.0),
            vertical_up: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_distance: 1.0
        }
    }
}