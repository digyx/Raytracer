use crate::vec3::{self, Vec3};
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
    fn origin(&self) -> Point3 {
        self.origin
    }

    fn direction(&self) -> Point3 {
        self.direction
    }

    fn at(&self, t: f32) -> Vec3 {
        self.origin() + t * self.direction()
    }

    pub fn colour(&self) -> Colour {
        let mut t = hits_sphere(vec3::new(0.0, 0.0, -1.0), 0.5, self);

        if t > 0.0 {
            let n = (self.at(t) - vec3::new(0.0, 0.0, -1.0)).unit();
            return 0.5 * (n + 1.0)
        }

        let unit_dir = self.direction().unit();
        
        t = 0.5*(unit_dir.y() + 1.0);

        (1.0 - t) * vec3::new(1.0, 1.0, 1.0) + t * vec3::new(0.5, 0.7, 1.0)
    }
}

fn hits_sphere(center: Point3, radius: f32, r: &Ray) -> f32 {
    let oc = r.origin() - center;

    let a = r.direction().len() * r.direction().len();
    let half_b = vec3::dot(oc, r.direction());
    let c = oc.len() * oc.len() - radius * radius;

    let discriminant = half_b * half_b - a * c;
    
    if discriminant < 0.0 {
        return -1.0
    }

    (-half_b - discriminant.sqrt()) / a
}
