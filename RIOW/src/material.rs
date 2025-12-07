use rand::random;
use ultraviolet::Vec3;
use crate::hittable::HitRecord;
use crate::ray::{near_zero, Ray};
use crate::ray::random_unit_vec;

pub trait Material: Sync + Send {

    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> { None }

}

pub struct Lambertian {
    colour: Vec3
}

impl Lambertian {
    pub fn new(colour: Vec3) -> Self {
        Lambertian { colour }
    }
}

impl Material for Lambertian {

    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let mut direction = rec.normal + random_unit_vec();

        let s = 1e-8;
        if near_zero(&direction) { direction = rec.normal }

        Some((Ray::new(rec.point, direction), self.colour))

    }

}

pub struct Metal {
    colour: Vec3,
    fuzz: f32
}

impl Metal {
    pub fn new(colour: Vec3, fuzz: f32) -> Self {
        Metal { colour, fuzz: fuzz.clamp(0.0, 1.0) }
    }
}

impl Material for Metal {

    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {

        let reflected = ray_in.direction.reflected(rec.normal).normalized()
            + (self.fuzz * random_unit_vec());
        let scattered = Ray::new(rec.point, reflected);

        if scattered.direction.dot(rec.normal) > 0.0 {
            Some((scattered,  self.colour))
        } else {
            None
        }

    }

}

pub struct Dielectric {
    refract_idx: f32
}


impl Dielectric {
    pub fn new(refract_idx: f32) -> Dielectric { Dielectric{refract_idx} }

    pub fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0*r0;
        r0 + (1.0-r0)*f32::powi(1.0-cosine, 5)
    }

}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let col = Vec3::one();
        let ri = if rec.front_face { 1.0 / self.refract_idx } else { self.refract_idx };

        let unit_dir = ray_in.direction.normalized();
        let cos_theta = rec.normal.dot(-unit_dir).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;

        let dir = if cannot_refract ||
            Dielectric::reflectance(cos_theta, ri) > random() {
            // reflect
            unit_dir.reflected(rec.normal)

        } else {
            // refract
            unit_dir.refracted(rec.normal, ri)
        };


        Some((Ray::new(rec.point, dir), col))
    }
}