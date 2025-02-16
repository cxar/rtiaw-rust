use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Index;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;

pub type Point3 = Vec3;

#[derive(Copy, Clone)]
pub struct Vec3 {
    e: [f64; 3],
}

#[allow(dead_code)]
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { e: [x, y, z] }
    }
    pub fn default() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
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
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] = self.e[0] + rhs.e[0];
        self.e[1] = self.e[1] + rhs.e[1];
        self.e[2] = self.e[2] + rhs.e[2];
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        )
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &Self::Output {
        &self.e[i]
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.e[0] * rhs.e[0],
            self.e[1] * rhs.e[1],
            self.e[2] * rhs.e[2],
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self::new(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs)
    }
}

// Commutative mul
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] = self.e[0] * rhs;
        self.e[1] = self.e[1] * rhs;
        self.e[2] = self.e[2] * rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

fn dot(l: Vec3, r: Vec3) -> f64 {
    l.e[0] * r.e[0] + l.e[1] * r.e[1] + l.e[2] * r.e[2]
}

fn cross(l: Vec3, r: Vec3) -> Vec3 {
    Vec3::new(
        l.e[1] * r.e[2] - l.e[2] * r.e[1],
        l.e[2] * r.e[0] - l.e[0] * r.e[2],
        l.e[0] * r.e[1] - l.e[1] * r.e[0],
    )
}

fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}
