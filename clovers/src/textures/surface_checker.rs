//! Checkered texture based on the surface coordinates of an object.

use palette::white_point::E;
use palette::Xyz;

use super::TextureTrait;
#[cfg(feature = "serde-derive")]
use crate::colorinit::TypedColorInit;
use crate::ray::Ray;
use crate::spectrum::SPD;
use crate::wavelength::Wavelength;
use crate::{colorinit::ColorInit, HitRecord};
use crate::{Float, PI};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// A standard checkered texture based on 2D surface UV coordinates.
pub struct SurfaceCheckerInit {
    /// Uniform color for the even-numbered checkers of the texture.
    #[cfg_attr(feature = "serde-derive", serde(default = "default_even"))]
    pub even: ColorInit,
    /// Uniform color for the odd-numbered checkers of the texture.
    #[cfg_attr(feature = "serde-derive", serde(default = "default_odd"))]
    pub odd: ColorInit,
    /// Controls the density of the checkered pattern. Default value is 10, which corresponds to using 10 tiles over the width of the object. On spheres, this means 10 tiles around the sphere.
    #[cfg_attr(feature = "serde-derive", serde(default = "default_density_surface"))]
    pub density: Float,
}

impl From<SurfaceCheckerInit> for SurfaceChecker {
    fn from(value: SurfaceCheckerInit) -> Self {
        SurfaceChecker::new(value.even, value.odd, value.density)
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde-derive", serde(from = "SurfaceCheckerInit"))]
/// A standard checkered texture based on 2D surface UV coordinates.
pub struct SurfaceChecker {
    /// Uniform color for the even-numbered checkers of the texture.
    #[cfg_attr(feature = "serde-derive", serde(skip))]
    even: SPD,
    /// Uniform color for the odd-numbered checkers of the texture.
    #[cfg_attr(feature = "serde-derive", serde(skip))]
    odd: SPD,
    /// Controls the density of the checkered pattern. Default value is 10, which corresponds to using 10 tiles over the width of the object. On spheres, this means 10 tiles around the sphere.
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
fn default_density_surface() -> Float {
    10.0
}

impl SurfaceChecker {
    /// Create a new `SurfaceChecker` object with the specified colors and density.
    #[must_use]
    pub fn new(color1: impl Into<Xyz<E>>, color2: impl Into<Xyz<E>>, density: Float) -> Self {
        let even = SPD::new(color1.into());
        let odd = SPD::new(color2.into());
        SurfaceChecker { even, odd, density }
    }
}

impl TextureTrait for SurfaceChecker {
    /// Evaluates the color at the given surface position coordinates. Note that `SurfaceChecker` is surface-based, and thus ignores the spatial position coordinate.
    fn color(&self, _ray: &Ray, wavelength: Wavelength, hit_record: &HitRecord) -> Float {
        let density = self.density * PI;
        let sines = 1.0 // cosmetic 1 for readability of following lines :)
              * (density * hit_record.u).sin()
              * (density * hit_record.v).sin();
        if sines < 0.0 {
            self.odd.get(wavelength)
        } else {
            self.even.get(wavelength)
        }
    }
}
