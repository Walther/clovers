//! A solid color texture.

use palette::{white_point::E, Xyz};

use super::TextureTrait;
use crate::colorinit::ColorInit;
use crate::ray::Ray;
use crate::spectrum::spectral_power;
use crate::wavelength::Wavelength;
use crate::{Float, HitRecord};

/// Initialization structure for a solid color texture.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
pub struct SolidColorInit {
    /// Initialization struct for the color.
    pub color: ColorInit,
}

impl From<SolidColorInit> for SolidColor {
    fn from(value: SolidColorInit) -> Self {
        SolidColor {
            color: value.color.into(),
        }
    }
}

#[derive(Clone, Debug)]
/// A solid color texture. Simplest possible [Texture](crate::textures::Texture): returns a solid color at any surface coordinate or spatial position.
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde-derive", serde(from = "SolidColorInit"))]
pub struct SolidColor {
    /// The color of the [Texture](crate::textures::Texture).
    pub color: Xyz<E>,
}

impl TextureTrait for SolidColor {
    /// Evaluates the color ignoring the given surface coordinates and spatial position - always returns the solid color.
    #[must_use]
    fn color(&self, _ray: &Ray, wavelength: Wavelength, _hit_record: &HitRecord) -> Float {
        spectral_power(self.color, wavelength)
    }
}

impl SolidColor {
    /// Creates a new solid color texture with the specified color.
    #[must_use]
    pub fn new(color: impl Into<Xyz<E>>) -> Self {
        SolidColor {
            color: color.into(),
        }
    }
}

impl Default for SolidColor {
    fn default() -> Self {
        // middle grey
        Self {
            color: Xyz::new(0.5, 0.5, 0.5),
        }
    }
}
