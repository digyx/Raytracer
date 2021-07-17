use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

use super::vec3::Colour;

pub fn write_colour(f: &mut File, colour: Colour) {
    let res = f.write(format!(
        "{} {} {}\n", 
        i32_256(colour.x()), i32_256(colour.y()), i32_256(colour.z())
    ).as_bytes());

    if res.is_err() {
        println!("error: {}", res.unwrap_err());
        exit(1);
    }
}

fn i32_256(f: f32) -> i32 {
    (255.9999 * f) as i32
}
