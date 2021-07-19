use std::fs::File;

use rand::random;

use crate::{IMAGE_HEIGHT, IMAGE_WIDTH, SAMPLES_PER_PX};
use crate::objects::World;
use crate::vec3::{Colour, Point3, Vec3};
use crate::ray::Ray;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio = IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::new(0.0, 0.0,0.0);
        let horizontal = Point3::new(viewport_width, 0.0, 0.0);
        let vertical = Point3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Point3::new(0.0, 0.0, focal_length);

        Camera{origin, lower_left_corner, horizontal, vertical}
    }

    pub fn render(self, world: World, f: &mut File) -> Option<String> {
        for j in (0..IMAGE_HEIGHT).rev() {
            print!("\rCasting...{:.2}%", (1.0 - j as f32 / IMAGE_HEIGHT as f32) * 100.0);
            for i in 0..IMAGE_WIDTH {
                let mut pixel = Colour::new(0.0, 0.0, 0.0);
                for _ in 1..SAMPLES_PER_PX {
                    let u = (i as f32 + random::<f32>()) / (IMAGE_WIDTH - 1) as f32;
                    let v = (j as f32 + random::<f32>()) / (IMAGE_HEIGHT - 1) as f32;
                    
                    let r = Ray::new(
                        self.origin, 
                        self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin
                    );

                    pixel += r.colour(world.clone());
                }
                
                pixel.write(f);
            }
        }

        None
    }
}
