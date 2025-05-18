//! Checkered texture based on the world coordinates.

// TODO: object-aligned spatial checker?

use palette::Xyz;
use palette::white_point::E;

use super::TextureTrait;
#[cfg(feature = "serde-derive")]
use crate::colorinit::TypedColorInit;
use crate::ray::Ray;
use crate::spectrum::SPD;
use crate::wavelength::Wavelength;
use crate::{Float, PI};
use crate::{HitRecord, colorinit::ColorInit};

/// A standard checkered texture based on spatial 3D texturing.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
pub struct SpatialCheckerInit {
    /// Uniform color for the even-numbered checkers of the texture.
    #[cfg_attr(feature = "serde-derive", serde(default = "default_even"))]
    pub even: ColorInit,
    /// Uniform color for the odd-numbered checkers of the texture.
    #[cfg_attr(feature = "serde-derive", serde(default = "default_odd"))]
    pub odd: ColorInit,
    /// Controls the density of the checkered pattern. Default value is 1.0, which corresponds to filling a 1.0 unit cube in the coordinate system with one color of the pattern. Even values preferred - odd values may create a visually thicker stripe due to two stripes with same color being next to each other.
    #[cfg_attr(feature = "serde-derive", serde(default = "default_density_spatial"))]
    pub density: Float,
}

impl From<SpatialCheckerInit> for SpatialChecker {
    fn from(init: SpatialCheckerInit) -> Self {
        SpatialChecker::new(init.even, init.odd, init.density)
    }
}

/// A standard checkered texture based on spatial 3D texturing.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde-derive", serde(from = "SpatialCheckerInit"))]
pub struct SpatialChecker {
    /// Uniform color for the even-numbered checkers of the texture.
    #[cfg_attr(feature = "serde-derive", serde(skip))]
    even: SPD,
    /// Uniform color for the odd-numbered checkers of the texture.
    #[cfg_attr(feature = "serde-derive", serde(skip))]
    odd: SPD,
    /// Controls the density of the checkered pattern. Default value is 1.0, which corresponds to filling a 1.0 unit cube in the coordinate system with one color of the pattern. Even values preferred - odd values may create a visually thicker stripe due to two stripes with same color being next to each other.
    pub density: Float,
}

#[cfg(feature = "serde-derive")]
fn default_even() -> ColorInit {
    // TODO: what would be a sensible color here?
    ColorInit::TypedColor(TypedColorInit::XyzE(Xyz::new(0.8, 0.8, 0.8)))
}

#[cfg(feature = "serde-derive")]
fn default_odd() -> ColorInit {
    // Middle gray
    ColorInit::TypedColor(TypedColorInit::XyzE(Xyz::new(0.5, 0.5, 0.5)))
}

#[cfg(feature = "serde-derive")]
fn default_density_spatial() -> Float {
    1.0
}

impl SpatialChecker {
    /// Create a new `SpatialChecker` object with the specified colors and density.
    #[must_use]
    pub fn new(color1: impl Into<Xyz<E>>, color2: impl Into<Xyz<E>>, density: Float) -> Self {
        let even = SPD::new(color1.into());
        let odd = SPD::new(color2.into());
        SpatialChecker { even, odd, density }
    }
}

impl TextureTrait for SpatialChecker {
    /// Evaluates the color at the given spatial position coordinate. Note that the `SpatialChecker` is spatial - surface coordinates are ignored.
    fn color(&self, _ray: &Ray, wavelength: Wavelength, hit_record: &HitRecord) -> Float {
        let position = hit_record.position;
        let density = self.density * PI;
        let sines = 1.0 // cosmetic 1 for readability of following lines :)
            * (density * position.x).sin()
            * (density * position.y).sin()
            * (density * position.z).sin();
        if sines < 0.0 {
            self.odd.get(wavelength)
        } else {
            self.even.get(wavelength)
        }
    }
}
