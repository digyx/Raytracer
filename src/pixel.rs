use rand::random;
use std::sync::Arc;

use crate::{IMAGE_HEIGHT, IMAGE_WIDTH, SAMPLES_PER_PX};

use crate::objects::World;
use crate::ray::Ray;
use crate::vec3::{Colour, Point3, Vec3};


pub struct Pixel {
    x: i32,
    y: i32,
    x_ray: Vec3,
    y_ray: Vec3,
    colour: Colour
}

impl Pixel {
    pub fn new(x: i32, y: i32, x_ray: Vec3, y_ray: Vec3) -> Pixel {
        Pixel{x, y, x_ray, y_ray, colour: Colour::new(0.0, 0.0, 0.0)}
    }

    pub fn render(&mut self, cam_loc: Point3, vport_origin: Vec3, world: Arc<World>) -> Colour {
        for _ in 0..SAMPLES_PER_PX {
            let u = (self.x as f32 + random::<f32>()) / (IMAGE_WIDTH - 1) as f32;
            let v = (self.y as f32 + random::<f32>()) / (IMAGE_HEIGHT - 1) as f32;

            let r = Ray::new(
                cam_loc,
                vport_origin + u * self.x_ray + v * self.y_ray - cam_loc
            );

            self.colour += r.cast(&Arc::clone(&world));
        }

        self.colour
    }
}
