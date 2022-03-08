use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    pub e: [f64; 3],
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }
    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        return self.e[0].powf(2.0) + self.e[1].powf(2.0) + self.e[2].powf(2.0);
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];

        self
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self {
        self.e[0] -= rhs.e[0];
        self.e[1] -= rhs.e[1];
        self.e[2] -= rhs.e[2];

        self
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(mut self, rhs: f64) -> Self {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;

        self
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(mut self, rhs: f64) -> Self {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
        self.e[2] /= rhs;

        self
    }
}

#[test]
fn x_test() {
    let v = Vec3::new(-1.11, 0.0, 0.0);
    assert_eq!(v.x(), -1.11);
}

#[test]
fn y_test() {
    let v = Vec3::new(0.0, 99.9, 0.0);
    assert_eq!(v.y(), 99.9);
}

#[test]
fn z_test() {
    let v = Vec3::new(0.0, 0.0, 0.0001);
    assert_eq!(v.z(), 0.0001);
}

#[test]
fn length_test() {
    let v = Vec3::new(0.0, 3.0, 4.0);
    assert_eq!(v.length(), 5.0);
}

#[test]
fn length_squared_test() {
    let v = Vec3::new(0.0, 3.0, 4.0);
    assert_eq!(v.length_squared(), 25.0);
}

#[test]
fn add_test() {
    let lhs = Vec3::new(1.0, -1.0, 0.0001);
    let rhs = Vec3::new(2.0, 1.0, 0.0001);
    assert_eq!(lhs + rhs, Vec3::new(3.0, 0.0, 0.0002));
}

#[test]
fn sub_test() {
    let lhs = Vec3::new(1.0, -1.0, 0.0001);
    let rhs = Vec3::new(2.0, 1.0, 0.0001);
    assert_eq!(lhs - rhs, Vec3::new(-1.0, -2.0, 0.0));
}

#[test]
fn mul_test() {
    let v = Vec3::new(1.0, -1.0, 0.0001);
    let f = 10_f64;
    assert_eq!(v * f, Vec3::new(10.0, -10.0, 0.001));
}

#[test]
fn mul_test_reverse() {
    let v = Vec3::new(1.0, -1.0, 0.0001);
    let f = 10_f64;
    assert_eq!(f * v, Vec3::new(10.0, -10.0, 0.001));
}

#[test]
fn div_test() {
    let v = Vec3::new(1.0, -1.0, 0.0001);
    let f = 10_f64;
    assert_eq!(v / f, Vec3::new(0.1, -0.1, 0.00001));
}

// Utility functions
pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

#[test]
fn unit_vector_test() {
    let v = Vec3::new(3.0, 4.0, 0.0);
    assert_eq!(unit_vector(v), Vec3::new(0.6, 0.8, 0.0));
}

#[test]
fn dot_test() {
    let u = Vec3::new(1.0, 1.0, 1.0);
    let v = Vec3::new(2.0, 2.0, 2.0);
    assert_eq!(dot(&u, &v), 6.0);
}
