//! Textures enable different surface textures for colorizing objects in various ways.

pub mod checkered;
pub mod noise_texture;
pub mod solid_color;
use serde::{Deserialize, Serialize};

pub use checkered::*;
pub use solid_color::*;

use crate::{color::Color, Float, Vec3};

pub trait TextureTrait {
    fn color(&self, _u: Float, _v: Float, _position: Vec3) -> Color {
        Color::default()
    }
}

#[derive(Copy, Clone, Deserialize, Serialize, Debug)]
pub struct Texture<T: TextureTrait> {
    texture: T,
}

impl<T> Default for Texture<T>
where
    T: TextureTrait,
{
    fn default() -> Self {
        Texture {
            texture: SolidColor::default(),
        }
    }
}
