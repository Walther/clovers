use super::Texture;
use crate::{color::Color, Float, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Deserialize, Serialize, Debug)]
pub struct Checkered {
    // TODO: get recursive textures back, maybe?
    #[serde(default = "default_even")]
    even: Color,
    #[serde(default = "default_odd")]
    odd: Color,
    #[serde(default = "default_density")]
    density: Float,
}

fn default_even() -> Color {
    Color::new(0.82, 0.82, 0.82)
}

fn default_odd() -> Color {
    Color::new(0.18, 0.18, 0.18)
}

fn default_density() -> Float {
    // TODO: this density parameter feels odd to intuit and manipulate
    0.1
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
