use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Default for Camera {
    fn default() -> Self {
        let look_from = Point3::new(3.0, 3.0, 2.0);
        let look_at = Point3::new(0.0, 0.0, -1.0);
        let focal_dist = (look_from - look_at).length();
        Camera::new(
            look_from,
            look_at,
            Vec3::new(0.0, 1.0, 0.0),
            20.0,
            16.0 / 9.0,
            2.0,
            focal_dist,
        )
    }
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        v_up: Vec3,
        vertical_fov_deg: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vertical_fov_deg.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vec3::unit_vector(look_from - look_at);
        let u = Vec3::unit_vector(v_up.cross(w));
        let v = w.cross(u);

        let origin = look_from;

        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;
        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        let origin = self.origin + offset;
        let direction =
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset;
        Ray::new(origin, direction)
    }
}
