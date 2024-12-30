//! Textures enable different surface textures for colorizing objects in various ways.

pub mod solid_color;
pub mod spatial_checker;
pub mod surface_checker;

use crate::{
    illuminants::{D50, D65},
    materials::gltf::GLTFMaterial,
};
use enum_dispatch::enum_dispatch;
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
    /// GLTF material as a texture - a bit of a hack
    #[cfg(feature = "gltf")]
    #[cfg_attr(feature = "serde-derive", serde(skip))]
    GLTFTexture(&'static GLTFMaterial),
    /// D50 Standard Illuminant
    IlluminantD50(D50),
    /// D65 Standard Illuminant
    IlluminantD65(D65),
}

#[enum_dispatch]
/// The main texture trait
pub trait TextureTrait {
    /// Returns the spectral reflectance of the texture sampled at the given wavelength at the given location.
    #[must_use]
    fn color(&self, ray: &Ray, wavelength: Wavelength, hit_record: &HitRecord) -> Float;

    /// Returns the spectral power of the texture sampled at the given wavelength. Defaults to none, override for emissive textures.
    #[must_use]
    fn emit(&self, _ray: &Ray, _wavelength: Wavelength, _hit_record: &HitRecord) -> Float {
        0.0
    }
}

impl Default for Texture {
    fn default() -> Self {
        SolidColor::default().into()
    }
}
