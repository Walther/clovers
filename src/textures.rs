pub mod checkered;
pub mod noise_texture;
pub mod solid_color;

pub use checkered::*;
// pub use noise_texture::*;
pub use solid_color::*;

use crate::{color::Color, Float, Vec3};

// pub trait Texture: Sync + Send {
//     fn color(&self, u: Float, v: Float, position: Vec3) -> Color;
// }

#[derive(Copy, Clone)]
pub enum Texture {
    Checkered(Checkered),
    SolidColor(SolidColor),
}

impl Texture {
    pub fn color(&self, u: Float, v: Float, position: Vec3) -> Color {
        match *self {
            Texture::Checkered(c) => Checkered::color(c, u, v, position),
            Texture::SolidColor(s) => SolidColor::color(s, u, v, position),
        }
    }
}
