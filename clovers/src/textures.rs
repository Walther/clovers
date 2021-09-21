//! Textures enable different surface textures for colorizing objects in various ways.

#![allow(clippy::large_enum_variant)] // TODO: NoiseTexture is massive compared to others.

pub mod checkered;
#[cfg(not(target_arch = "spirv"))]
pub mod noise_texture;
pub mod solid_color;

pub use checkered::*;
// pub use noise_texture::*;
pub use solid_color::*;

use crate::{color::Color, Float, Vec3};
#[cfg(not(target_arch = "spirv"))]
use noise_texture::NoiseTexture;

#[cfg(target_arch = "spirv")]
use crate::PI;
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float as FloatTrait;

#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
#[cfg(not(target_arch = "spirv"))]
#[derive(Copy, Clone, Debug)]
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

#[cfg(target_arch = "spirv")]
#[derive(Copy, Clone)]
/// A texture kind enum, GPU version
pub enum TextureKind {
    /// SolidColor texture
    SolidColor,
    /// SpatialChecker texture
    SpatialChecker,
    /// SurfaceChecker texture
    SurfaceChecker,
}

#[cfg(target_arch = "spirv")]
/// A texture struct, GPU version
pub struct Texture {
    kind: TextureKind,
    color: Color,
    even: Color,
    odd: Color,
    density: Float,
}

#[cfg(not(target_arch = "spirv"))]
impl Texture {
    /// Evaluates the color of the texture at the given surface coordinates or spatial coordinate.
    pub fn color(&self, u: Float, v: Float, position: Vec3) -> Color {
        match self {
            Texture::NoiseTexture(n) => n.color(u, v, position),
            Texture::SolidColor(s) => s.color(u, v, position),
            Texture::SpatialChecker(c) => c.color(u, v, position),
            Texture::SurfaceChecker(c) => c.color(u, v, position),
        }
    }
}

#[cfg(target_arch = "spirv")]
impl Texture {
    /// Evaluates the color of the texture at the given surface coordinates or spatial coordinate.
    pub fn color(&self, u: Float, v: Float, position: Vec3) -> Color {
        match self.kind {
            TextureKind::SolidColor => self.color,
            // TODO: cleaner implementation! These are copy-pasted from `clovers/src/textures/checkered.rs`
            TextureKind::SpatialChecker => {
                let density = self.density * PI;
                let sines = 1.0
                    * (density * position.x).sin()
                    * (density * position.y).sin()
                    * (density * position.z).sin();
                if sines < 0.0 {
                    self.odd
                } else {
                    self.even
                }
            }
            TextureKind::SurfaceChecker => {
                let density = self.density * PI;
                let sines = 1.0 * (density * u).sin() * (density * v).sin();
                if sines < 0.0 {
                    self.odd
                } else {
                    self.even
                }
            }
        }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Default for Texture {
    fn default() -> Self {
        SolidColor::default().into()
    }
}

#[cfg(not(target_arch = "spirv"))]
impl From<SolidColor> for Texture {
    fn from(s: SolidColor) -> Self {
        Texture::SolidColor(s)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl From<SpatialChecker> for Texture {
    fn from(s: SpatialChecker) -> Self {
        Texture::SpatialChecker(s)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl From<SurfaceChecker> for Texture {
    fn from(s: SurfaceChecker) -> Self {
        Texture::SurfaceChecker(s)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl From<NoiseTexture> for Texture {
    fn from(s: NoiseTexture) -> Self {
        Texture::NoiseTexture(s)
    }
}
