use crate::vec3::Colour;

#[derive(Debug,Clone, Copy)]
pub struct Material {
    scatter: f32,
    colour: Colour
}

impl Material {
    pub fn new(scatter: f32, colour: Colour) -> Material {
        Material{scatter, colour}
    }

    pub fn scatter(&self) -> f32 {
        self.scatter
    }

    pub fn colour(&self) -> Colour {
        self.colour
    }
}
