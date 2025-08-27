//! Textures enable different surface textures for colorizing objects in various ways.

pub mod solid_color;
pub mod spatial_checker;
pub mod surface_checker;

#[allow(clippy::wildcard_imports)]
use crate::illuminants::*;
use crate::materials::gltf::GLTFMaterial;
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
    /// `SolidColor` texture
    SolidColor(SolidColor),
    /// `SpatialChecker` texture
    SpatialChecker(SpatialChecker),
    /// `SurfaceChecker` texture
    SurfaceChecker(SurfaceChecker),
    /// GLTF material as a texture - a bit of a hack
    #[cfg(feature = "gltf")]
    #[cfg_attr(feature = "serde-derive", serde(skip))]
    GLTFTexture(&'static GLTFMaterial),
    /// Standard Illuminant `D50`
    IlluminantD50(D50),
    /// Standard Illuminant `D65`
    IlluminantD65(D65),
    /// Standard Illuminant `LED-B1`
    IlluminantLedB1(LED_B1),
    /// Standard Illuminant `LED-B2`
    IlluminantLedB2(LED_B2),
    /// Standard Illuminant `LED-B3`
    IlluminantLedB3(LED_B3),
    /// Standard Illuminant `LED-B4`
    IlluminantLedB4(LED_B4),
    /// Standard Illuminant `LED-B5`
    IlluminantLedB5(LED_B5),
    /// Standard Illuminant `LED-BH1`
    IlluminantLedBH1(LED_BH1),
    /// Standard Illuminant `LED-RGB1`
    IlluminantLedRGB1(LED_RGB1),
    /// Standard Illuminant `LED-V1`
    IlluminantLedV1(LED_V1),
    /// Standard Illuminant `LED-V2`
    IlluminantLedV2(LED_V2),
    /// Standard Illuminant `FL1`
    IlluminantFL1(FL1),
    /// Standard Illuminant `FL2`
    IlluminantFL2(FL2),
    /// Standard Illuminant `FL3`
    IlluminantFL3(FL3),
    /// Standard Illuminant `FL4`
    IlluminantFL4(FL4),
    /// Standard Illuminant `FL5`
    IlluminantFL5(FL5),
    /// Standard Illuminant `FL6`
    IlluminantFL6(FL6),
    /// Standard Illuminant `FL7`
    IlluminantFL7(FL7),
    /// Standard Illuminant `FL8`
    IlluminantFL8(FL8),
    /// Standard Illuminant `FL9`
    IlluminantFL9(FL9),
    /// Standard Illuminant `FL10`
    IlluminantFL10(FL10),
    /// Standard Illuminant `FL11`
    IlluminantFL11(FL11),
    /// Standard Illuminant `FL12`
    IlluminantFL12(FL12),
    /// Standard Illuminant `FL3_1`
    IlluminantFL3_1(FL3_1),
    /// Standard Illuminant `FL3_2`
    IlluminantFL3_2(FL3_2),
    /// Standard Illuminant `FL3_3`
    IlluminantFL3_3(FL3_3),
    /// Standard Illuminant `FL3_4`
    IlluminantFL3_4(FL3_4),
    /// Standard Illuminant `FL3_5`
    IlluminantFL3_5(FL3_5),
    /// Standard Illuminant `FL3_6`
    IlluminantFL3_6(FL3_6),
    /// Standard Illuminant `FL3_7`
    IlluminantFL3_7(FL3_7),
    /// Standard Illuminant `FL3_8`
    IlluminantFL3_8(FL3_8),
    /// Standard Illuminant `FL3_9`
    IlluminantFL3_9(FL3_9),
    /// Standard Illuminant `FL3_10`
    IlluminantFL3_10(FL3_10),
    /// Standard Illuminant `FL3_11`
    IlluminantFL3_11(FL3_11),
    /// Standard Illuminant `FL3_12`
    IlluminantFL3_12(FL3_12),
    /// Standard Illuminant `FL3_13`
    IlluminantFL3_13(FL3_13),
    /// Standard Illuminant `FL3_14`
    IlluminantFL3_14(FL3_14),
    /// Standard Illuminant `FL3_15`
    IlluminantFL3_15(FL3_15),
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
