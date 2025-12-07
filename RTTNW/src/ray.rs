use rand::random_range;
use ultraviolet::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {

    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

}

pub fn random_unit_vec() -> Vec3 {
    loop {
        let out = Vec3::new(
            random_range(-1.0..1.0),
            random_range(-1.0..1.0),
            random_range(-1.0..1.0),
        );

        if 1e-160 < out.mag_sq() && out.mag_sq() < 1.0 {
            return out.normalized();
        }

    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(
            random_range(-1.0..1.0),
            random_range(-1.0..1.0),
            0.0
        );
        if p.mag_sq() < 1.0 { return p; }
    }
}

pub fn near_zero(v: &Vec3) -> bool {
    let thresh = 1e-8;
    let w = v.abs();

    w.x <= thresh && w.y <= thresh && w.z <= thresh
}