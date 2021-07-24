use std::sync::Arc;

use crate::MAX_REFLECTIONS;

use crate::vec3::{Colour, Point3, Vec3, dot};
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
                let res = reflect(
                    self.direction,
                    rec.normal(),
                    rec.scatter()
                );

                if res.is_none() {
                    return Colour::new(0.0, 0.0, 0.0);
                }

                let target = res.unwrap();

                let child = Self::child(
                    rec.point(),
                    target - rec.point(),
                    self.depth + 1
                );

                rec.colour() * child.cast(world)
            },
            None => {
                let unit_dir = self.direction().unit();
                let t = 0.5*(unit_dir.y() + 1.0);

                (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
            }
        }
    }
}


fn reflect(input: Vec3, normal: Vec3, scatter: f32) -> Option<Vec3> {
    let reflection_vector = input - 2.0 * dot(input, normal) * normal;

    if scatter == 0.0 && dot(reflection_vector, normal) > 0.0 {
        return Some(reflection_vector)
    } else if scatter == 0.0 {
        return None
    }

    let mut v: Vec3;

    if dot(reflection_vector, normal) <= 0.0 {
        return None
    }

    loop {
        let x = 2.0 * rand::random::<f32>() - 1.0;
        let y = 2.0 * rand::random::<f32>() - 1.0;
        let z = 2.0 * rand::random::<f32>() - 1.0;

        v = Vec3::new(x, y, z);

        if dot(v, v) < 1.0 {break}
    }

    Some((scatter * (normal + v) + (1.0 - scatter) * reflection_vector).unit())
}
