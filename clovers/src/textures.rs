//! Textures enable different surface textures for colorizing objects in various ways.

pub mod solid_color;
pub mod spatial_checker;
pub mod surface_checker;

use crate::illuminants::{D50, D65};
use enum_dispatch::enum_dispatch;
use palette::{white_point::E, Xyz};
pub use solid_color::*;
pub use spatial_checker::*;
pub use surface_checker::*;

use crate::{ray::Ray, wavelength::Wavelength, Float, HitRecord};

#[enum_dispatch(TextureTrait)]
#[derive(Clone, Debug)]
/// A texture enum.
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde-derive", serde(tag = "kind"))]
pub enum Texture {
    /// SolidColor texture
    SolidColor(SolidColor),
    /// SpatialChecker texture
    SpatialChecker(SpatialChecker),
    /// SurfaceChecker texture
    SurfaceChecker(SurfaceChecker),
    /// D50 Standard Illuminant
    IlluminantD50(D50),
    /// D65 Standard Illuminant
    IlluminantD65(D65),
}

#[enum_dispatch]
pub(crate) trait TextureTrait {
    /// Evaluates the color of the texture at the given surface coordinates or spatial coordinate.
    #[must_use]
    fn color(&self, hit_record: &HitRecord) -> Xyz<E>;

    /// Returns the spectral power of the texture sampled at the given wavelength. Defaults to none, override for emissive textures.
    fn emit(&self, _ray: &Ray, _wavelength: Wavelength, _hit_record: &HitRecord) -> Float {
        0.0
    }
}

impl Default for Texture {
    fn default() -> Self {
        SolidColor::default().into()
    }
}
