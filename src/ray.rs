use crate::vec3::{Point3, Vec3};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ray = Ray::new(Point3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0));
        assert_eq!(ray.origin, Point3::new(1.0, 2.0, 3.0));
        assert_eq!(ray.direction, Vec3::new(4.0, 5.0, 6.0));
    }

    #[test]
    fn test_at() {
        let ray = Ray::new(Point3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0));
        assert_eq!(Point3::new(13.0, 17.0, 21.0), ray.at(3.0));
    }
}
