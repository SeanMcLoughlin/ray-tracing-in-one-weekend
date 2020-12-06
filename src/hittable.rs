use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub mod sphere;

pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
    pub t: f64,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        p: Point3,
        normal: Vec3,
        material: &'a dyn Material,
        t: f64,
        front_face: bool,
    ) -> Self {
        HitRecord {
            p,
            normal,
            material,
            t,
            front_face,
        }
    }

    #[inline]
    pub fn build_with_face_normal(
        ray: Ray,
        outward_normal: Vec3,
        material: &'a dyn Material,
        point: Point3,
        t: f64,
    ) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord::new(point, normal, material, t, front_face)
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl<T: AsRef<[Box<dyn Hittable>]>> Hittable for T {
    fn hit(&self, ray: &Ray, t_min: f64, mut t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far: Option<HitRecord> = None;
        for hittable in self.as_ref().iter() {
            if let Some(hit_record) = hittable.hit(ray, t_min, t_max) {
                t_max = hit_record.t;
                closest_so_far = Some(hit_record);
            }
        }
        closest_so_far
    }
}
