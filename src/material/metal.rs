use crate::hittable::HitRecord;
use crate::material::{Material, Scatter};
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

#[derive(Clone, Copy)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let reflected = Vec3::unit_vector(ray_in.direction).reflect(hit_record.normal);
        let ray = Ray::new(
            hit_record.p,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );
        let attenuation = self.albedo;
        if ray.direction.dot(hit_record.normal) > 0.0 {
            Some(Scatter { ray, attenuation })
        } else {
            None
        }
    }
}
