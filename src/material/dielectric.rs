use crate::hittable::HitRecord;
use crate::material::{Material, Scatter};
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};
use rand::{thread_rng, Rng};

#[derive(Clone, Copy)]
pub struct Dielectric {
    ir: f64, // Index of Refraction
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Dielectric { ir }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Schlick's approximation for reflectance.
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = Vec3::unit_vector(ray_in.direction);

        let cos_theta = f64::min(-unit_direction.dot(hit_record.normal), 1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let mut rng = thread_rng();
        let schlick_approx =
            Dielectric::reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0, 1.0);

        let direction = if cannot_refract || schlick_approx {
            unit_direction.reflect(hit_record.normal)
        } else {
            unit_direction.refract(hit_record.normal, refraction_ratio)
        };

        let ray = Ray::new(hit_record.p, direction);
        Some(Scatter { ray, attenuation })
    }
}
