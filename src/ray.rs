use crate::vec3::{self, Vec3};
use crate::vec3::{Point3, Colour};
use crate::objects::{World};

#[derive(Debug,Copy,Clone)]
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

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin() + t * self.direction()
    }

    pub fn colour(&self, world: World) -> Colour {
        match world.hit(self, -1.0, 1.0) {
            Some(rec) => {
                0.5 * (rec.normal() + vec3::new(1.0, 1.0, 1.0))
            },
            None => {
                let unit_dir = self.direction().unit();
                let t = 0.5*(unit_dir.y() + 1.0);
        
                (1.0 - t) * vec3::new(1.0, 1.0, 1.0) + t * vec3::new(0.5, 0.7, 1.0)
            }
        }
    }
}
