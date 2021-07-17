use std::env;

use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

mod vec3;
mod colour;

const IMAGE_HEIGHT: i32 = 256;
const IMAGE_WIDTH: i32 = 256;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args[1] == "test" {
        image_gen_test();
    }
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

            let c: vec3::Colour = vec3::new(r,g,b);
            colour::write_colour(&mut f, c);
        }
    }

    println!("\nDone.");
}
