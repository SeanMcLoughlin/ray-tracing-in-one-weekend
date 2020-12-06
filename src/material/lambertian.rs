use crate::hittable::HitRecord;
use crate::material::{Material, Scatter};
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        let ray = Ray::new(hit_record.p, scatter_direction);
        let attenuation = self.albedo;
        Some(Scatter { ray, attenuation })
    }
}
