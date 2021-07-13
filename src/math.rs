use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign};
use super::{random, random_range};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const ONE: Self = Self {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn squared_norm(self) -> f64 {
        self.dot(self)
    }

    pub fn norm(self) -> f64 {
        self.squared_norm().sqrt()
    }

    pub fn normalize(self) -> Self {
        self / self.norm()
    }

    pub fn normalized(&mut self) {
        *self = self.normalize();
    }

    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    pub fn clamp(self, min: f64, max: f64) -> Self {
        Self {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
            z: self.z.clamp(min, max),
        }
    }

    pub fn reflect(&self, n: Self) -> Self {
        *self - 2.0 * n * self.dot(n)
    }

    pub fn refract(self, n: Self, eta1: f64, eta2: f64) -> Self {
        let cos_theta = -self.dot(n).min(1.0);
        let perp = (eta1 / eta2) * (self + cos_theta * n);
        let parallel = -(1.0 - perp.squared_norm()).abs().sqrt() * n;
        perp + parallel
    }

    pub fn random(min: f64, max: f64) -> Self {
        Self {
            x: random_range(min , max),
            y: random_range(min , max),
            z: random_range(min , max),
        }
    }

    pub fn random_unit_sphere() -> Self {
        Self::random(-1.0, 1.0).normalize()
    }

    pub fn random_unit_disk() -> Self {
        let r = random();
        let theta = random_range(0.0, std::f64::consts::PI * 2.0);
        r * vec3(theta.cos(), theta.sin(), 0.0)
    }

    pub fn is_close(&self, v: Self) -> bool {
        (self.x - v.x).abs() < f64::EPSILON
            && (self.y - v.y).abs() < f64::EPSILON
            && (self.z - v.z).abs() < f64::EPSILON
    }

    pub fn is_close_scalar(&self, v: f64) -> Self {
        Self {
            x: if (self.x - v).abs() < f64::EPSILON { 1.0 } else { 0.0 },
            y: if (self.y - v).abs() < f64::EPSILON { 1.0 } else { 0.0 },
            z: if (self.z - v).abs() < f64::EPSILON { 1.0 } else { 0.0 },
        }
    }

    pub fn is_nan(&self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }

    pub fn is_infinite(&self) -> bool {
        self.x.is_infinite() || self.y.is_infinite() || self.z.is_infinite()
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs.mul(self)
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of range"),
        }
    }
}

pub fn vec3(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3 { x, y, z }
}

pub type Point3 = Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(o: Point3, d: Vec3, t: f64) -> Self {
        Self { origin: o, dir: d, time: t, }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.dir * t
    }
}

pub fn ray(origin: Point3, dir: Vec3, time: f64) -> Ray {
    Ray { origin, dir, time }
}

pub fn is_campled(v: f64, min: f64, max: f64) -> bool {
    v >= min && v <= max
}
