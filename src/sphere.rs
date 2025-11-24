use std::sync::Arc;
use ultraviolet::Vec3;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;

pub struct Sphere {
    centre: Vec3,
    radius: f32,
    mat: Arc<dyn Material>
}

impl Sphere {
    pub fn new(centre: Vec3, radius: f32, mat: Arc<dyn Material>) -> Self {
        Sphere { centre, radius: f32::max(0.0, radius),  mat }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {

        let oc = self.centre.clone() - ray.origin;
        let a = ray.direction.mag_sq();
        let h = ray.direction.dot(oc);
        let c = oc.mag_sq() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant > 0.0 {
            let discriminant_sqrt = discriminant.sqrt();
            let root = (h - discriminant_sqrt) / a;

            if root < t_max && root > t_min {

                let pnt = ray.at(root);
                return Some(HitRecord{
                    point: pnt,
                    normal: (pnt - self.centre) / self.radius,
                    material: self.mat.clone(),
                    t: root,
                    front_face: true
                });

            }

            let root = (h + discriminant_sqrt) / a;
            if root < t_max && root > t_min {

                let pnt = ray.at(root);
                return Some(HitRecord{
                    point: pnt,
                    normal: (pnt - self.centre) / self.radius,
                    material: self.mat.clone(),
                    t: root,
                    front_face: false
                });

            }

        }

        None

    }
}