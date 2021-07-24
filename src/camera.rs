use std::fs::File;
use std::thread;
use std::sync::Arc;

use crate::{IMAGE_HEIGHT, IMAGE_WIDTH};

use crate::pixel::Pixel;
use crate::objects::World;
use crate::vec3::{Point3, Vec3, Colour};

#[derive(Debug,Clone, Copy)]
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

    pub fn render(self, world: Arc<World>, f: &mut File) {
        let mut thread_vec = Vec::new();

        for j in (0..IMAGE_HEIGHT).rev() {
            let world = Arc::clone(&world);

            let handle = thread::spawn(move|| -> Vec<Colour> {
                let mut line_vec = Vec::new();

                for i in 0..IMAGE_WIDTH {
                    // progress(i, j);

                    let world = Arc::clone(&world);
                    let mut px = Pixel::new(
                        i,
                        j,
                        self.horizontal,
                        self.vertical
                    );

                    let c = px.render(self.origin, self.lower_left_corner, world);
                    line_vec.push(c);
                }

                line_vec
            });

            thread_vec.push(handle);
        }

        for line in thread_vec {
            for px in line.join().unwrap() {
                px.write(f);
            }
        }

    }
}


// fn progress(i: i32, j: i32) {
//     let total = (IMAGE_WIDTH * IMAGE_HEIGHT) as f32;
//     let current = (IMAGE_WIDTH * (IMAGE_HEIGHT - j - 1) + i) as f32;

//     let percent = 100.0 * (current / total);

//     print!("\rCasting...{:.2}%", percent);
// }
