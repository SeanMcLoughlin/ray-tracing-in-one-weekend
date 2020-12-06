pub mod lambertian;
pub mod metal;

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Color;

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Color,
}

pub trait Material {
    fn scatter(&self, ray_in: Ray, hit_record: &HitRecord) -> Option<Scatter>;
}
