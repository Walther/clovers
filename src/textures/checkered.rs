use super::Texture;
use crate::{color::Color, Float, Vec3};

#[derive(Copy, Clone)]
pub struct Checkered {
    // TODO: get recursive textures back, maybe?
    even: Color,
    odd: Color,
    density: Float,
}

impl Checkered {
    pub fn new(color1: Color, color2: Color, density: Float) -> Texture {
        Texture::Checkered(Checkered {
            even: color1,
            odd: color2,
            density,
        })
    }

    pub fn color(self, _u: Float, _v: Float, position: Vec3) -> Color {
        let sines = (self.density * position.x).sin()
            * (self.density * position.y).sin()
            * (self.density * position.z).sin();
        if sines < 0.0 {
            // return odd.color(u, v, position); TODO:
            self.odd
        } else {
            // return even.color(u, v, position); TODO:
            self.even
        }
    }
}
