use ultraviolet::Vec3;
use crate::ray::Ray;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool
}



pub trait Hittable {

    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;

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
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.vec.iter()
            .filter_map(|x| x.hit(ray, t_min, t_max))
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
    }

}