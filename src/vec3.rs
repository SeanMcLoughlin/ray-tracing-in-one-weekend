use rand::Rng;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

pub type Color = Vec3;
pub type Point3 = Vec3;

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    #[inline]
    pub fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[inline]
    pub fn cross(self, rhs: Self) -> Self {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    #[inline]
    pub fn unit_vector(self) -> Self {
        self / self.length()
    }

    pub fn random(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        Vec3 {
            x: rng.gen_range(min, max),
            y: rng.gen_range(min, max),
            z: rng.gen_range(min, max),
        }
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        Vec3 {
            x: rng.gen_range(min, max),
            y: rng.gen_range(min, max),
            z: rng.gen_range(min, max),
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Vec3::unit_vector(Vec3::random_in_unit_sphere())
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Self {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        let in_same_hemisphere_as_normal = in_unit_sphere.dot(normal) > 0.0;
        if in_same_hemisphere_as_normal {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn reflect(self, n: Vec3) -> Self {
        self - 2.0 * self.dot(n) * n
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        (1f64 / rhs) * self
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1f64 / rhs;
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            i => panic!("index {} out of bounds", i),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            i => panic!("index {} out of bounds", i),
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(
            Vec3::new(4.0, 4.0, 4.0),
            Vec3::new(2.0, 2.0, 2.0) + Vec3::new(2.0, 2.0, 2.0)
        );
        assert_eq!(
            Vec3::new(3.1, 4.2, 5.3),
            Vec3::new(0.1, 0.2, 0.3) + Vec3::new(3.0, 4.0, 5.0)
        );
    }

    #[test]
    fn test_add_assign() {
        let mut act: Vec3;

        act = Vec3::new(2.0, 2.0, 2.0);
        act += Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(Vec3::new(4.0, 4.0, 4.0), act);

        act = Vec3::new(0.1, 0.2, 0.3);
        act += Vec3::new(3.0, 4.0, 5.0);
        assert_eq!(Vec3::new(3.1, 4.2, 5.3), act);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(
            Vec3::new(4.0, 4.0, 4.0),
            Vec3::new(6.0, 6.0, 6.0) - Vec3::new(2.0, 2.0, 2.0)
        );
        assert_eq!(
            Vec3::new(3.1, 4.2, 5.3),
            Vec3::new(4.0, 5.0, 6.0) - Vec3::new(0.9, 0.8, 0.7)
        );
    }

    #[test]
    fn test_sub_assign() {
        let mut act: Vec3;

        act = Vec3::new(6.0, 6.0, 6.0);
        act -= Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(Vec3::new(4.0, 4.0, 4.0), act);

        act = Vec3::new(3.0, 4.0, 5.0);
        act -= Vec3::new(0.1, 0.2, 0.3);
        assert_eq!(Vec3::new(2.9, 3.8, 4.7), act);
    }

    #[test]
    fn test_multiply_vec3_to_vec3() {
        assert_eq!(
            Vec3::new(4.0, 9.0, 16.0),
            Vec3::new(2.0, 3.0, 4.0) * Vec3::new(2.0, 3.0, 4.0)
        );
        assert_eq!(
            Vec3::new(6.0, 12.0, 20.0),
            Vec3::new(2.0, 3.0, 4.0) * Vec3::new(3.0, 4.0, 5.0)
        );
    }

    #[test]
    fn test_multiply_float_to_vec3() {
        assert_eq!(Vec3::new(4.0, 6.0, 8.0), Vec3::new(2.0, 3.0, 4.0) * 2.0);
        assert_eq!(Vec3::new(10.0, 12.0, 14.0), 2.0 * Vec3::new(5.0, 6.0, 7.0));
    }

    #[test]
    fn test_multiply_assign() {
        let mut act: Vec3;

        act = Vec3::new(6.0, 6.0, 6.0);
        act *= 2.0;
        assert_eq!(Vec3::new(12.0, 12.0, 12.0), act);

        act = Vec3::new(3.0, 4.0, 5.0);
        act *= 5.0;
        assert_eq!(Vec3::new(15.0, 20.0, 25.0), act);
    }

    #[test]
    fn test_divide() {
        assert_eq!(Vec3::new(2.0, 4.0, 6.0), Vec3::new(4.0, 8.0, 12.0) / 2.0);
        assert_eq!(Vec3::new(3.0, 3.0, 3.0), Vec3::new(9.0, 9.0, 9.0) / 3.0);
    }

    #[test]
    fn test_divide_assign() {
        let mut act: Vec3;

        act = Vec3::new(12.0, 12.0, 12.0);
        act /= 2.0;
        assert_eq!(Vec3::new(6.0, 6.0, 6.0), act);

        act = Vec3::new(15.0, 20.0, 25.0);
        act /= 5.0;
        assert_eq!(Vec3::new(3.0, 4.0, 5.0), act);
    }

    #[test]
    fn test_index() {
        let vec = Vec3::new(3.0, 4.0, 5.0);
        assert!(approx_eq!(f64, vec[0], 3.0, ulps = 2));
        assert!(approx_eq!(f64, vec[1], 4.0, ulps = 2));
        assert!(approx_eq!(f64, vec[2], 5.0, ulps = 2));
    }

    #[test]
    fn test_index_mut() {
        #[allow(unused_assignments)]
        let mut vec = Vec3::new(3.0, 4.0, 5.0);

        vec = Vec3::new(4.0, 5.0, 6.0);
        assert!(approx_eq!(f64, vec[0], 4.0, ulps = 2));
        assert!(approx_eq!(f64, vec[1], 5.0, ulps = 2));
        assert!(approx_eq!(f64, vec[2], 6.0, ulps = 2));
    }

    #[test]
    fn test_negate() {
        assert_eq!(Vec3::new(-2.0, -4.0, -6.0), -Vec3::new(2.0, 4.0, 6.0));
        assert_eq!(Vec3::new(-3.0, -3.0, -3.0), -Vec3::new(3.0, 3.0, 3.0));
    }

    #[test]
    fn test_type_aliases() {
        assert_eq!(
            Color {
                x: 3.0,
                y: 3.0,
                z: 3.0
            },
            Vec3 {
                x: 3.0,
                y: 3.0,
                z: 3.0
            }
        );
        assert_eq!(
            Point3 {
                x: 4.0,
                y: 4.0,
                z: 4.0
            },
            Vec3 {
                x: 4.0,
                y: 4.0,
                z: 4.0
            }
        );
    }

    #[test]
    fn test_dot_product() {
        let exp: f64 = 3.0;
        let act = Vec3::new(1.0, 1.0, 1.0).dot(Vec3::new(1.0, 1.0, 1.0));
        assert!(approx_eq!(f64, exp, act, ulps = 2));
    }

    #[test]
    fn test_cross_product() {
        let exp = Vec3::new(-1.0, 2.0, -1.0);
        let act = Vec3::new(2.0, 3.0, 4.0).cross(Vec3::new(3.0, 4.0, 5.0));
        assert_eq!(exp, act);
    }
}
