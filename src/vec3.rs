use std::ops::{Add, AddAssign, Div, Mul, Sub};

use rand::Rng;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
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

    pub fn random(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn random_0_1() -> Vec3 {
        Vec3::random(0.0, 1.1)
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        unit_vector(Vec3::random_in_unit_sphere())
    }

    pub fn random_in_hemishpere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if dot(&in_unit_sphere, normal) > 0.0 {
            // In the same hemisphere as the normal
            return in_unit_sphere;
        } else {
            return -1.0 * in_unit_sphere;
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x().abs() < s && self.y().abs() < s && self.z().abs() < s
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - 2.0 * dot(v, n) * (*n)
    }

    pub fn refract(uv: &Vec3, n: &Vec3, eta_over_eta_prime: f64) -> Vec3 {
        let cos_theta = 1.0_f64.min(dot(&(-1.0 * *uv), n));
        let r_out_perpendicular = eta_over_eta_prime * (*uv + cos_theta * *n);
        let r_out_parallel = -((1.0 - r_out_perpendicular.length_squared()).abs().sqrt()) * *n;

        r_out_parallel + r_out_perpendicular
    }
}

// Utility functions
pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3 {
        e: [
            u.e[1] * v.e[2] - u.e[2] * v.e[1],
            u.e[2] * v.e[0] - u.e[0] * v.e[2],
            u.e[0] * v.e[1] - u.e[1] * v.e[0],
        ],
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

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
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

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
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
fn mul_vector_test() {
    let v = Vec3::new(1.0, -1.0, 0.1);
    let u = Vec3::new(2.0, 3.0, -1.0);
    assert_eq!(v * u, Vec3::new(2.0, -3.0, -0.1));
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

#[test]
fn add_assign_test() {
    let mut u = Vec3::new(1.0, 1.0, 1.0);
    let v = Vec3::new(2.0, 2.0, 2.0);

    u += v;
    assert_eq!(u, Vec3::new(3.0, 3.0, 3.0));
}

#[test]
fn near_zero_test() {
    let v1 = Vec3::new(1e-9, 1e-9, 1e-9);
    let v2 = Vec3::new(1e-7, 1e-7, 1e-7);

    assert_eq!(v1.near_zero(), true);
    assert_eq!(v2.near_zero(), false);
}
