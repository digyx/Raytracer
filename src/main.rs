use std::env;
use std::sync::Arc;

use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

mod vec3;
mod ray;
mod objects;

use crate::vec3::{Colour};
use crate::objects::World;
use crate::objects::sphere;

const IMAGE_WIDTH: i32 = 640;
const IMAGE_HEIGHT: i32 = 360;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&String::from("test")) {
        image_gen_test();
    }

    // World
    let world: World = World(
        vec![
            Arc::new(sphere::new(vec3::new(0.0,    0.0, -1.0), 0.5)),
            Arc::new(sphere::new(vec3::new(1.0,    0.0, -1.0), 0.25)),
            Arc::new(sphere::new(vec3::new(0.0, -100.5, -1.0), 100.0))
        ]
    );

    // Camera
    const ASPECT_RATIO: f32 = IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32;

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = vec3::new(0.0, 0.0,0.0);
    let horizontal = vec3::new(viewport_width, 0.0, 0.0);
    let vertical = vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - vec3::new(0.0, 0.0, focal_length);

    // Render
    let mut f = File::create("target/out.ppm").unwrap();
    let res = writeln!(f, "P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    if res.is_err() {
        println!("error: {}", res.unwrap_err());
        exit(1);
    }

    for j in (0..IMAGE_HEIGHT).rev() {
        print!("\rCasting...{:.2}%", (1.0 - j as f32 / IMAGE_HEIGHT as f32) * 100.0);
        for i in 0..IMAGE_WIDTH {
            let u = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let v = j as f32 / (IMAGE_HEIGHT - 1) as f32;

            let r = ray::new(
                origin, 
                lower_left_corner + u * horizontal + v * vertical - origin
            );

            r.colour(world.clone()).write(&mut f);
        }
    }

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

            let c: Colour = vec3::new(r,g,b);
            c.write(&mut f);
        }
    }

    println!("\nDone.");
}
