use ultraviolet::Vec3;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::camera::random_unit_vec;

pub trait Material: Sync + Send {

    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)>;

}

pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {

    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let mut direction = rec.normal + random_unit_vec();

        let s = 1e-8;
        if direction.x.abs() < s && direction.y.abs() < s && direction.z.abs() < s {
            direction = rec.normal;
        }

        let scattered = Ray::new(&rec.point, &direction);

        Some((scattered, self.albedo))

    }

}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        Metal { albedo, fuzz: fuzz.clamp(0.0, 1.0) }
    }
}

impl Material for Metal {

    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {

        let mut reflected = ray_in.direction.reflected(rec.normal);
        reflected = reflected.normalized() + (self.fuzz * random_unit_vec());
        let scattered = Ray::new(&rec.point, &reflected);
        let attenuation = self.albedo;

        if scattered.direction.dot(rec.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }

    }

}