use crate::vec3::{Point3, Colour, Vec3};
use crate::objects::{World};

#[derive(Debug,Copy,Clone)]
pub struct Ray {
    origin: Point3,
    direction: Vec3
}

impl Ray {
    pub fn new(origin: Point3, direction: Point3) -> Ray {
        Ray{origin, direction}
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin() + t * self.direction()
    }

    pub fn colour(&self, world: World) -> Colour {
        match world.hit(self, 0.0, f32::INFINITY) {
            Some(rec) => {
                0.5 * (rec.normal() + Colour::new(1.0, 1.0, 1.0))
            },
            None => {
                let unit_dir = self.direction().unit();
                let t = 0.5*(unit_dir.y() + 1.0);
        
                (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
            }
        }
    }
}
