use std::env;

use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

mod vec3;
mod colour;
mod ray;

use vec3::{Colour};
use ray::Ray;

const IMAGE_WIDTH: i32 = 640;
const IMAGE_HEIGHT: i32 = 360;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&String::from("test")) {
        image_gen_test();
    }

    const ASPECT_RATIO: f32 = IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32;

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    // Camera
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
        print!("\rScanlines Remaining {}", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let v = j as f32 / (IMAGE_HEIGHT - 1) as f32;

            let r = ray::new(
                origin, 
                lower_left_corner + u * horizontal + v * vertical - origin
            );

            colour::write_colour(&mut f, ray_colour(r));
        }
    }

    println!("\nDone.")
}

fn ray_colour(r: Ray) -> Colour {
    let unit_dir = r.direction().unit();
    let t = 0.5*(unit_dir.y() + 1.0);

    (1.0 - t) * vec3::new(1.0, 1.0, 1.0) + t * vec3::new(0.5, 0.7, 1.0)
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
            colour::write_colour(&mut f, c);
        }
    }

    println!("\nDone.");
}
