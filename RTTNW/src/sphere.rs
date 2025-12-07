use std::ops::Range;
use std::sync::Arc;
use ultraviolet::Vec3;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;

pub struct Sphere {
    centre: Vec3,
    radius: f32,
    material: Arc<dyn Material>
}

impl Sphere {
    pub fn new(centre: Vec3, radius: f32, material: Arc<dyn Material>) -> Self {
        Sphere { centre, radius: radius.max(0.0), material }
    }

    pub fn get_face_normal(r: &Ray, out_norm: Vec3) -> (bool, Vec3) {
        let front_face = r.direction.dot(out_norm) < 0.0;
        let normal = if front_face { out_norm } else { -out_norm };

        (front_face, normal)
    }

}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_interval: Range<f32>) -> Option<HitRecord> {

        let oc = self.centre - ray.origin;
        let a = ray.direction.mag_sq();
        let h = ray.direction.dot(oc);
        let c = oc.mag_sq() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 { return None }

        let discriminant_sqrt = discriminant.sqrt();
        let mut root = (h - discriminant_sqrt) / a;
        if !t_interval.contains(&root) {
            root = ( h + discriminant_sqrt ) / a;
            if !t_interval.contains(&root) {
                return None
            }
        }

        let p = ray.at(root);
        let norm = ( p - self.centre ) / self.radius;
        let (front_face, out_norm) = Sphere::get_face_normal(ray, norm);

        Some(HitRecord{
            point: p,
            normal: out_norm,
            time: root,
            front_face,
            material: self.material.clone()
        })

    }
}

pub struct MovingSphere {
    center: Ray,
    radius: f32,
    material: Arc<dyn Material>
}

impl MovingSphere {
    pub(crate) fn new(centre_0: Vec3, centre_1: Vec3, radius: f32, material: Arc<dyn Material>) -> Self {
        MovingSphere {
            center: Ray::new(centre_0, centre_1 - centre_0, 0.0),
            radius: radius.max(0.0),
            material
        }
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_interval: Range<f32>) -> Option<HitRecord> {

        let curr_centre = self.center.at(ray.time);
        let oc = curr_centre - ray.origin;
        let a = ray.direction.mag_sq();
        let h = ray.direction.dot(oc);
        let c = oc.mag_sq() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 { return None }

        let discriminant_sqrt = discriminant.sqrt();
        let mut root = (h - discriminant_sqrt) / a;
        if !t_interval.contains(&root) {
            root = ( h + discriminant_sqrt ) / a;
            if !t_interval.contains(&root) {
                return None
            }
        }

        let p = ray.at(root);
        let norm = ( p - curr_centre ) / self.radius;
        let (front_face, out_norm) = Sphere::get_face_normal(ray, norm);

        Some(HitRecord{
            point: p,
            normal: out_norm,
            time: root,
            front_face,
            material: self.material.clone()
        })
    }
}