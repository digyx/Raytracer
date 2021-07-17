use crate::vec3;
use crate::vec3::{Point3, Colour};

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

    pub fn colour(&self) -> Colour {
        let unit_dir = self.direction().unit();
        let t = 0.5*(unit_dir.y() + 1.0);

        (1.0 - t) * vec3::new(1.0, 1.0, 1.0) + t * vec3::new(0.5, 0.7, 1.0)
    }
}
