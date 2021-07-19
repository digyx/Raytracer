use std::env;
use std::sync::Arc;

use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

mod vec3;
mod ray;
mod pixel;
mod objects;
mod camera;

use crate::vec3::{Colour, Point3};
use crate::objects::World;
use crate::objects::sphere::Sphere;
use crate::camera::Camera;

const IMAGE_WIDTH: i32 = 640;
const IMAGE_HEIGHT: i32 = 360;
const SAMPLES_PER_PX: i32 = 10;
const MAX_REFLECTIONS: i32 = 10;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&String::from("test")) {
        image_gen_test();
        exit(0);
    }

    // World
    let world  = World::new(
        vec![
            Arc::new(Sphere::new(Point3::new(0.0,    0.0, -1.0), 0.5)),
            Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0))
        ]
    );

    // Render
    let mut f = File::create("target/out.ppm").unwrap();
    let res = writeln!(f, "P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    if res.is_err() {
        println!("error: {}", res.unwrap_err());
        exit(1);
    }

    let cam = Camera::new();
    cam.render(&world, &mut f);

    println!("\nDone.")
}

fn image_gen_test() {
    println!("Generating...");

    let mut f = File::create("target/out.ppm").unwrap();
    let res = writeln!(f, "P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    if res.is_err() {
        println!("error: {}", res.unwrap_err());
        exit(1);
    }

    for j in (0..IMAGE_HEIGHT).rev() {
        print!("\rScanlines Remaining {}", j);
        for i in 0..IMAGE_WIDTH {
            let r = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let g = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let b = 0.25;

            let c = Colour::new(r,g,b);

            c.write(&mut f);
        }
    }

    println!("\nDone.");
}
