use std::ops::Range;
use std::sync::Arc;
use ultraviolet::Vec3;
use crate::material::Material;
use crate::ray::Ray;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub time: f32,
    pub front_face: bool
}

pub trait Hittable: Sync + Send {

    fn hit(&self, ray: &Ray, t_interval: Range<f32>) -> Option<HitRecord>;

}

pub struct HittableList {
    vec: Vec<Box<dyn Hittable>>,
}

impl HittableList {

    pub fn new() -> Self {
        HittableList { vec: Vec::new() }
    }

    pub fn add(&mut self, hittable: Box<dyn Hittable>) {
        self.vec.push(hittable);
    }

    pub fn _clear(&mut self) {
        self.vec.clear();
    }

}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_interval: Range<f32>) -> Option<HitRecord> {
        self.vec.iter()
            .filter_map(|x| x.hit(ray, t_interval.clone()))
            .min_by(|a, b| a.time.partial_cmp(&b.time).unwrap())
    }

}