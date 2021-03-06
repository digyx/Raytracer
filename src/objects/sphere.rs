use crate::vec3::{self, Colour};
use crate::vec3::Point3;

use super::{HitRecord, Hittable};
use super::material::Material;

pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Material
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, scatter: f32, colour: Colour) -> Sphere {
        Sphere{center, radius, material: Material::new(scatter, colour)}
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center;

        let a = vec3::dot(r.direction(), r.direction());
        let half_b = vec3::dot(oc, r.direction());
        let c = vec3::dot(oc, oc) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None
        }

        // Object may be hit
        let sqrt_d = discriminant.sqrt();
        let mut root = (-half_b - sqrt_d) / a;

        if root < t_min || root > t_max {
            root = (-half_b + sqrt_d) / a;

            if root < t_min || root > t_max {
                return None
            }
        }

        // Update the Hit Record, object is hit
        let point = r.at(root);
        let normal = (point - self.center) / self.radius;

        let front_face = vec3::dot(r.direction(), normal) < 0.0;

        let rec = HitRecord{
            point,
            normal: if front_face{normal} else{-normal},
            t: root,
            front_face,
            material: self.material
        };

        Some(rec)
    }
}
