use std::sync::Arc;

use crate::MAX_REFLECTIONS;

use crate::vec3::{Point3, Colour, Vec3};
use crate::objects::{World};

pub struct Ray {
    origin: Point3,
    direction: Vec3,
    depth: i32
}

impl Ray {
    pub fn new(origin: Point3, direction: Point3) -> Ray {
        Ray{origin, direction , depth: 0}
    }

    fn child(origin: Point3, direction: Point3, depth:i32) -> Ray {
        Ray{origin, direction, depth}
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

    pub fn cast(&self, world: &Arc<World>) -> Colour {
        if self.depth >= MAX_REFLECTIONS {
            return Colour::new(0.0, 0.0, 0.0)
        }

        match world.hit(self, 0.001, f32::INFINITY) {
            Some(rec) => {
                let target = rec.point() + rec.normal() + Vec3::new_rand();
                let child = Self::child(
                    rec.point(), 
                    target - rec.point(), 
                    self.depth + 1,

                );

                0.5 * child.cast(world)
            },
            None => {
                let unit_dir = self.direction().unit();
                let t = 0.5*(unit_dir.y() + 1.0);
        
                (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
            }
        }
    }
}
