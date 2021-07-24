use std::sync::Arc;

use crate::ray::Ray; 
use crate::vec3::{Colour, Point3, Vec3};
use material::Material;

pub mod sphere;
pub mod material;

#[derive(Debug,Copy,Clone)]
pub struct HitRecord {
    point: Point3,
    normal: Vec3,
    t: f32,
    front_face: bool,
    material: Material
}

impl HitRecord {
    pub fn point(&self) -> Vec3 {
        self.point
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn scatter(&self) -> f32 {
        self.material.scatter()
    }

    pub fn colour(&self) -> Colour {
        self.material.colour()
    }

    fn t(&self) -> f32 {
        self.t
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct World(Vec<Arc<dyn Hittable + Send + Sync>>);

impl World {
    pub fn new(v: Vec<Arc<dyn Hittable + Send + Sync>>) -> World{
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
