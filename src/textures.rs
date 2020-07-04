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
    Checkered {
        // TODO: get recursive textures back, maybe?
        even: SolidColor,
        odd: SolidColor,
        density: Float,
    },
    // NoiseTexture {
    //     noise: Perlin,
    //     scale: Float,
    // },
    SolidColor {
        color: Color,
    },
}

impl Texture {
    pub fn color(&self, u: Float, v: Float, position: Vec3) -> Color {
        match *self {
            Texture::Checkered { even, odd, density } => {
                Checkered::color(even, odd, density, u, v, position)
            }
            // Texture::NoiseTexture { noise, scale } => {
            //     NoiseTexture::color(noise, scale, u, v, position)
            // }
            Texture::SolidColor { color } => SolidColor::color(color, u, v, position),
        }
    }
}
