use std::sync::Arc;

use crate::ray::Ray; 
use crate::vec3::{Point3, Vec3};

pub mod sphere;

#[derive(Debug,Copy,Clone)]
pub struct HitRecord {
    point: Point3,
    normal: Vec3,
    t: f32,
    front_face: bool
}

impl HitRecord {
    pub fn point(&self) -> Vec3 {
        self.point
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    fn t(&self) -> f32 {
        self.t
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[derive(Clone)]
pub struct World(Vec<Arc<dyn Hittable>>);

impl World {
    pub fn new(v: Vec<Arc<dyn Hittable>>) -> World{
        World(v)
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut res:Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for obj in self.0.iter() {
            match obj.hit(r, t_min, closest_so_far) {
                Some(rec) => {
                    closest_so_far = rec.t();
                    res = Some(rec);
                },
                None => continue
            }
        }

        res
    }
}
