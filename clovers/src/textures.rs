//! Textures enable different surface textures for colorizing objects in various ways.

pub mod solid_color;
pub mod spatial_checker;
pub mod surface_checker;

use enum_dispatch::enum_dispatch;
pub use solid_color::*;
pub use spatial_checker::*;
pub use surface_checker::*;

use crate::{color::Color, Float, Vec3};

#[enum_dispatch(TextureTrait)]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// A texture enum.
#[cfg_attr(feature = "serde-derive", serde(tag = "kind"))]
pub enum Texture {
    /// SolidColor texture
    SolidColor(SolidColor),
    /// SpatialChecker texture
    SpatialChecker(SpatialChecker),
    /// SurfaceChecker texture
    SurfaceChecker(SurfaceChecker),
}

#[enum_dispatch]
pub(crate) trait TextureTrait {
    /// Evaluates the color of the texture at the given surface coordinates or spatial coordinate.
    #[must_use]
    fn color(&self, u: Float, v: Float, position: Vec3) -> Color;
}

impl Default for Texture {
    fn default() -> Self {
        SolidColor::default().into()
    }
}
