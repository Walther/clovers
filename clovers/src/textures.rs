//! Textures enable different surface textures for colorizing objects in various ways.

pub mod checkered;
pub mod noise_texture;
pub mod solid_color;
use serde::{Deserialize, Serialize};

pub use checkered::*;
// pub use noise_texture::*;
pub use solid_color::*;

use crate::{color::Color, Float, Vec3};
use noise_texture::NoiseTexture;

pub trait TextureTrait {
    fn color(&self, _u: Float, _v: Float, _position: Vec3) -> Color {
        Color::default()
    }
}

#[derive(Copy, Clone, Deserialize, Serialize, Debug)]
pub struct Texture<T>
where
    T: TextureTrait,
{
    texture: T,
}

impl<T> Texture<T>
where
    T: TextureTrait,
{
    pub fn color(&self, u: Float, v: Float, position: Vec3) -> Color {
        match *self {
            Texture::SpatialChecker(c) => SpatialChecker::color(c, u, v, position),
            Texture::SurfaceChecker(c) => SurfaceChecker::color(c, u, v, position),
            Texture::SolidColor(s) => SolidColor::color(s, u, v, position),
            Texture::NoiseTexture(n) => NoiseTexture::color(n, u, v, position),
        }
    }
}

impl<T> Default for Texture<T>
where
    T: TextureTrait,
{
    fn default() -> Self {
        SolidColor::default().into()
    }
}

impl<T> From<SolidColor> for Texture<T>
where
    T: TextureTrait,
{
    fn from(s: SolidColor) -> Self {
        Texture::SolidColor(s)
    }
}

impl<T> From<SpatialChecker> for Texture<T>
where
    T: TextureTrait,
{
    fn from(s: SpatialChecker) -> Self {
        Texture::SpatialChecker(s)
    }
}

impl<T> From<SurfaceChecker> for Texture<T>
where
    T: TextureTrait,
{
    fn from(s: SurfaceChecker) -> Self {
        Texture::SurfaceChecker(s)
    }
}

impl<T> From<NoiseTexture> for Texture<T>
where
    T: TextureTrait,
{
    fn from(s: NoiseTexture) -> Self {
        Texture::NoiseTexture(s)
    }
}
