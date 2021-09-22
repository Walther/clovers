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

#[derive(Copy, Clone)]
#[repr(C)]
/// A texture kind enum, GPU version
pub enum GPUTextureKind {
    /// SolidColor texture
    SolidColor,
    /// SpatialChecker texture
    SpatialChecker,
    /// SurfaceChecker texture
    SurfaceChecker,
}

/// A texture struct, GPU version
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GPUTexture {
    /// Which kind of a texture is this
    pub kind: GPUTextureKind,
    /// Stores the even color for SurfaceChecker and SpatialChecker, also used as the main color for SolidColor
    pub even: Color,
    /// Stores the odd color for SurfaceChecker and SpatialChecker
    pub odd: Color,
    /// Stores the density for SurfaceChecker and SpatialChecker
    pub density: Float,
}

#[cfg(not(target_arch = "spirv"))]
impl Texture {
    /// Evaluates the color of the texture at the given surface coordinates or spatial coordinate.
    #[deprecated]
    pub fn color(&self, u: Float, v: Float, position: Vec3) -> Color {
        match self {
            Texture::NoiseTexture(n) => n.color(u, v, position),
            Texture::SolidColor(s) => s.color(u, v, position),
            Texture::SpatialChecker(c) => c.color(u, v, position),
            Texture::SurfaceChecker(c) => c.color(u, v, position),
        }
    }
}

impl GPUTexture {
    /// Evaluates the color of the texture at the given surface coordinates or spatial coordinate.
    pub fn color(&self, u: Float, v: Float, position: Vec3) -> Color {
        match self.kind {
            GPUTextureKind::SolidColor => self.even,
            // TODO: cleaner implementation! These are copy-pasted from `clovers/src/textures/checkered.rs`
            GPUTextureKind::SpatialChecker => {
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
            GPUTextureKind::SurfaceChecker => {
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

#[cfg(not(target_arch = "spirv"))]
impl From<Texture> for GPUTexture {
    fn from(t: Texture) -> Self {
        match t {
            Texture::NoiseTexture(_) => todo!(),
            Texture::SolidColor(s) => GPUTexture {
                kind: GPUTextureKind::SolidColor,
                even: s.color,
                odd: s.color, // Ignored
                density: 1.0, // Ignored
            },
            Texture::SpatialChecker(s) => GPUTexture {
                kind: GPUTextureKind::SpatialChecker,
                even: s.even,
                odd: s.odd,
                density: s.density,
            },
            Texture::SurfaceChecker(s) => GPUTexture {
                kind: GPUTextureKind::SurfaceChecker,
                even: s.even,
                odd: s.odd,
                density: s.density,
            },
        }
    }
}
