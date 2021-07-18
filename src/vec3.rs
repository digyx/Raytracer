use std::ops::{Add, AddAssign, Div, Mul, Sub, Neg};
use std::fmt::Display;

use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

#[derive(Debug, Clone, Copy)]
pub struct Vec3(pub f32, pub f32, pub f32);

pub use Vec3 as Point3;  // 3D Point
pub use Vec3 as Colour;  // RGB Colour


pub fn dot(a: Vec3, b: Vec3) -> f32 {
    a.0 * b.0 +
    a.1 * b.1 +
    a.2 * b.2
}

impl Vec3 {
    pub fn x(&self) -> f32 {
        self.0.clone()
    }

    pub fn y(&self) -> f32 {
        self.1.clone()
    }

    pub fn z(&self) -> f32 {
        self.2.clone()
    }

    pub fn len(&self) -> f32 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.len()
    }
}

// Display
impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

// Math Functions
impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
        )
    }
}

impl Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f32) -> Self::Output {
        Vec3(
            self.0 + rhs,
            self.1 + rhs,
            self.2 + rhs,
        )
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3(
            self.0 * rhs,
            self.1 * rhs,
            self.2 * rhs,
        )
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(
            self * rhs.0,
            self * rhs.1,
            self * rhs.2,
        )
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3(
            self.0 / rhs,
            self.1 / rhs,
            self.2 / rhs,
        )
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output{
        Vec3(
            -self.0,
            -self.1,
            -self.2
        )
    }
}

// Colour Only (shh, not enforced)
impl Vec3 {
    pub fn write(&self, f: &mut File) {
        let r = (255.9999 * self.x()) as i32;
        let g = (255.9999 * self.y()) as i32;
        let b = (255.9999 * self.z()) as i32;

        let res = f.write(format!(
            "{} {} {}\n", r, g, b
        ).as_bytes());

        if res.is_err() {
            println!("error: {}", res.unwrap_err());
            exit(1);
        }
    }
}
