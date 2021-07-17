use std::fs::File;
use std::io::prelude::*;

const IMAGE_HEIGHT: i32 = 256;
const IMAGE_WIDTH: i32 = 256;

fn main() {
    println!("Generating...");

    let mut contents = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        print!("\rScanlines Remaining {}", j);
        for i in 0..IMAGE_WIDTH {
            let r = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let g = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let b = 0.25;

            let ir: i32 = (256 as f32 * r) as i32;
            let ig: i32 = (256 as f32 * g) as i32;
            let ib: i32 = (256 as f32 * b) as i32;

            contents.push_str(format!("{} {} {}\n", ir, ig, ib).as_str());
        }
    }

    let mut f = File::create("target/out.ppm").unwrap();

    let res = f.write_all(contents.as_bytes());

    match res {
        Err(_) => println!("\nFailed...shit."),
        _ => println!("\nDone.")
    }
}
