use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

use super::vec3::Colour;

pub fn write_colour(f: &mut File, colour: Colour) {
    let res = f.write(format!(
        "{} {} {}\n", 
        i32_265(colour.x()), i32_265(colour.y()), i32_265(colour.z())
    ).as_bytes());

    if res.is_err() {
        println!("error: {}", res.unwrap_err());
        exit(1);
    }
}

fn i32_265(f: f32) -> i32 {
    (256.0 * f) as i32
}
