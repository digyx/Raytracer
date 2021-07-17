use super::vec3::{Point3};

pub struct Ray {
    origin: Point3,
    direction: Point3
}

pub fn new(origin: Point3, direction: Point3) -> Ray {
    Ray{
        origin,
        direction
    }
}

impl Ray {
    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Point3 {
        self.direction
    }
}
