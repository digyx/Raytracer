use std::ops::{Add, AddAssign, Div, Mul, Sub, Neg};
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Vec3([f32; 3]);

type Point3 = Vec3;  // 3D Point
pub type Colour = Vec3;  // RGB Colour

pub fn new(a: f32, b: f32, c: f32) -> Vec3 {
    Vec3([a,b,c])
}

impl Vec3 {
    pub fn x(&self) -> f32 {
        self.0[0].clone()
    }

    pub fn y(&self) -> f32 {
        self.0[1].clone()
    }

    pub fn z(&self) -> f32 {
        self.0[2].clone()
    }

    pub fn len(&self) -> f32 {
        (self.0[0] * self.0[0] + self.0[1] * self.0[1] + self.0[2] * self.0[2]).sqrt()
    }
}

// Display
impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0[0], self.0[1], self.0[2])
    }
}

// Math Functions
impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ])
    }
}

impl Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f32) -> Self::Output {
        Vec3([
            self.0[0] + rhs,
            self.0[1] + rhs,
            self.0[2] + rhs,
        ])
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
        Vec3([
            self.0[0] * rhs,
            self.0[1] * rhs,
            self.0[2] * rhs,
        ])
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3([
            self.0[0] / rhs,
            self.0[1] / rhs,
            self.0[2] / rhs,
        ])
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output{
        Vec3([
            -self.0[0],
            -self.0[1],
            -self.0[2]
        ])
    }
}
