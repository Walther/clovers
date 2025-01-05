//! Textures enable different surface textures for colorizing objects in various ways.

pub mod solid_color;
pub mod spatial_checker;
pub mod surface_checker;

use crate::{illuminants::*, materials::gltf::GLTFMaterial};
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
    /// LED-B1 Standard Illuminant
    IlluminantLedB1(LED_B1),
    /// LED-B2 Standard Illuminant
    IlluminantLedB2(LED_B2),
    /// LED-B3 Standard Illuminant
    IlluminantLedB3(LED_B3),
    /// LED-B4 Standard Illuminant
    IlluminantLedB4(LED_B4),
    /// LED-B5 Standard Illuminant
    IlluminantLedB5(LED_B5),
    /// LED-BH1 Standard Illuminant
    IlluminantLedBH1(LED_BH1),
    /// LED-RGB1 Standard Illuminant
    IlluminantLedRGB1(LED_RGB1),
    /// LED-V1 Standard Illuminant
    IlluminantLedV1(LED_V1),
    /// LED-V2 Standard Illuminant
    IlluminantLedV2(LED_V2),
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
