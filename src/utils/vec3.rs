use rand::{self, Rng};
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec3(f64, f64, f64);

impl Vec3 {
    pub fn new() -> Self {
        Vec3(0., 0., 0.)
    }

    pub fn from(a: f64, b: f64, c: f64) -> Self {
        Vec3(a, b, c)
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vec3(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn unit_vector(self) -> Self {
        self / self.length()
    }

    pub fn to_rgb(&self, samples: u32) -> (u8, u8, u8) {
        // gamma set to 2

        let color = *self * (1. / samples as f64);
        let r = color.0.sqrt().clamp(0., 1.) * 255.;
        let g = color.1.sqrt().clamp(0., 1.) * 255.;
        let b = color.2.sqrt().clamp(0., 1.) * 255.;
        (r as u8, g as u8, b as u8)
    }

    pub fn near_zero(&self) -> bool {
        let epsilon = 1e-8;
        return (self.0.abs() < epsilon) && (self.1.abs() < epsilon) && (self.2.abs() < epsilon);
    }
}

impl Vec3 {
    pub fn random() -> Self {
        Vec3::from(rand::random(), rand::random(), rand::random())
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Vec3::from(
            rand::thread_rng().gen_range(min..max),
            rand::thread_rng().gen_range(min..max),
            rand::thread_rng().gen_range(min..max),
        )
    }

    pub fn random_unit_sphere() -> Self {
        loop {
            let p = Self::random_range(-1., 1.);
            if p.length_squared() < 1. {
                return p;
            }
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl Add<Self> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub<Self> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Mul<Self> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Div<Self> for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

impl AddAssign<Self> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl SubAssign<Self> for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}
impl MulAssign<Self> for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}
impl DivAssign<Self> for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
        self.2 /= rhs.2;
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        Vec3(self.0 + rhs, self.1 + rhs, self.2 + rhs)
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        Vec3(self.0 - rhs, self.1 - rhs, self.2 - rhs)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        self.0 += rhs;
        self.1 += rhs;
        self.2 += rhs;
    }
}

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, rhs: f64) {
        self.0 -= rhs;
        self.1 -= rhs;
        self.2 -= rhs;
    }
}
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn test_ops() {
        let a = Vec3::from(2., 1., 9.);
        let b = Vec3::from(1., 2., 3.);

        assert_eq!(a + b, Vec3::from(3., 3., 12.));
        assert_eq!(a - b, Vec3::from(1., -1., 6.));
        assert_eq!(a * b, Vec3::from(2., 2., 27.));
        assert_eq!(a / b, Vec3::from(2., 0.5, 3.));
    }

    #[test]
    fn test_length() {
        let a = Vec3::from(3., 4., 0.);

        assert_eq!(a.length_squared(), 25.);
        assert_eq!(a.length(), 5.);
    }

    #[test]
    fn test_unit_vector() {
        let a = Vec3::from(1., 1., 1.);
        let unit = a.unit_vector();

        assert_eq!(unit.length(), 1.);
        assert_eq!(
            unit,
            Vec3::from(1f64 / 3f64.sqrt(), 1f64 / 3f64.sqrt(), 1f64 / 3f64.sqrt())
        );
    }
}
