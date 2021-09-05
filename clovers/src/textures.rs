//! Textures enable different surface textures for colorizing objects in various ways.

#![allow(clippy::large_enum_variant)] // TODO: NoiseTexture is massive compared to others.

pub mod checkered;
pub mod noise_texture;
pub mod solid_color;

pub use checkered::*;
// pub use noise_texture::*;
pub use solid_color::*;

use crate::{color::Color, Float, Vec3};
use noise_texture::NoiseTexture;

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// A texture enum.
pub enum Texture {
    /// NoiseTexture texture
    NoiseTexture(NoiseTexture),
    /// SolidColor texture
    SolidColor(SolidColor),
    /// SpatialChecker texture
    SpatialChecker(SpatialChecker),
    /// SurfaceChecker texture
    SurfaceChecker(SurfaceChecker),
}

impl Texture {
    /// Evaluates the color of the texture at the given surface coordinates or spatial coordinate.
    pub fn color(&self, u: Float, v: Float, position: Vec3) -> Color {
        match *self {
            Texture::NoiseTexture(n) => NoiseTexture::color(n, u, v, position),
            Texture::SolidColor(s) => SolidColor::color(s, u, v, position),
            Texture::SpatialChecker(c) => SpatialChecker::color(c, u, v, position),
            Texture::SurfaceChecker(c) => SurfaceChecker::color(c, u, v, position),
        }
    }
}

impl Default for Texture {
    fn default() -> Self {
        SolidColor::default().into()
    }
}

impl From<SolidColor> for Texture {
    fn from(s: SolidColor) -> Self {
        Texture::SolidColor(s)
    }
}

impl From<SpatialChecker> for Texture {
    fn from(s: SpatialChecker) -> Self {
        Texture::SpatialChecker(s)
    }
}

impl From<SurfaceChecker> for Texture {
    fn from(s: SurfaceChecker) -> Self {
        Texture::SurfaceChecker(s)
    }
}

impl From<NoiseTexture> for Texture {
    fn from(s: NoiseTexture) -> Self {
        Texture::NoiseTexture(s)
    }
}
