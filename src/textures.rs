pub mod checkered;
pub mod noise_texture;
pub mod solid_color;
use serde::{Deserialize, Serialize};

pub use checkered::*;
// pub use noise_texture::*;
pub use solid_color::*;

use crate::{color::Color, Float, Vec3};
use noise_texture::NoiseTexture;

#[derive(Copy, Clone, Deserialize, Serialize, Debug)]
pub enum Texture {
    Checkered(Checkered),
    SolidColor(SolidColor),
    NoiseTexture(NoiseTexture),
}

impl Texture {
    pub fn color(&self, u: Float, v: Float, position: Vec3) -> Color {
        match *self {
            Texture::Checkered(c) => Checkered::color(c, u, v, position),
            Texture::SolidColor(s) => SolidColor::color(s, u, v, position),
            Texture::NoiseTexture(n) => NoiseTexture::color(n, u, v, position),
        }
    }
}

impl Default for Texture {
    fn default() -> Self {
        SolidColor::new(Color::default())
    }
}
