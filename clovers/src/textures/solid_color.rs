//! A solid color texture.

use palette::{white_point::E, Xyz};

use super::TextureTrait;
use crate::colorinit::ColorInit;
use crate::ray::Ray;
use crate::spectrum::SPD;
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
        SolidColor::new(value.color)
    }
}

#[derive(Clone, Debug)]
/// A solid color texture. Simplest possible [Texture](crate::textures::Texture): returns a solid color at any surface coordinate or spatial position.
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde-derive", serde(from = "SolidColorInit"))]
pub struct SolidColor {
    #[cfg_attr(feature = "serde-derive", serde(skip))]
    spectrum: SPD,
}

impl TextureTrait for SolidColor {
    /// Evaluates the color ignoring the given surface coordinates and spatial position - always returns the solid color.
    #[must_use]
    fn color(&self, _ray: &Ray, wavelength: Wavelength, _hit_record: &HitRecord) -> Float {
        self.spectrum.get(wavelength)
    }
}

impl SolidColor {
    /// Creates a new solid color texture with the specified color.
    #[must_use]
    pub fn new(color: impl Into<Xyz<E>>) -> Self {
        let spectrum = SPD::new(color.into());
        SolidColor { spectrum }
    }
}

impl Default for SolidColor {
    fn default() -> Self {
        // middle grey
        let color = Xyz::new(0.5, 0.5, 0.5);
        SolidColor::new(color)
    }
}
