use super::Texture;
use crate::{color::Color, Float, Vec3, PI};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Deserialize, Serialize, Debug)]
// A standard checkered texture.
pub struct Checkered {
    // TODO: get recursive textures back, maybe?
    #[serde(default = "default_even")]
    even: Color,
    #[serde(default = "default_odd")]
    odd: Color,
    #[serde(default = "default_density")]
    /// Controls the density of the checkered pattern. Default value is 1.0, which corresponds to filling a 1.0 unit square in the coordinate system with one color of the pattern.
    density: Float,
}

fn default_even() -> Color {
    Color::new(0.82, 0.82, 0.82)
}

fn default_odd() -> Color {
    Color::new(0.18, 0.18, 0.18)
}

fn default_density() -> Float {
    1.0
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
        // TODO: convert ahead-of-time. NOTE: take into account serde-i-fication; not enough to do in `new` alone
        let density = self.density * PI;
        let sines = 1.0 // cosmetic 1 for readability of following lines :)
            * (density * position.x).sin()
            * (density * position.y).sin()
            * (density * position.z).sin();
        if sines < 0.0 {
            // return odd.color(u, v, position); TODO:
            self.odd
        } else {
            // return even.color(u, v, position); TODO:
            self.even
        }
    }
}
