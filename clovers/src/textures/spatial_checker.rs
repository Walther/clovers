//! Checkered texture based on the world coordinates.

// TODO: object-aligned spatial checker?

use palette::convert::IntoColorUnclamped;
use palette::white_point::E;
use palette::Xyz;

use super::{ColorInit, TextureTrait, TypedColorInit};
use crate::{Float, Vec3, PI};

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
    fn from(value: SpatialCheckerInit) -> Self {
        SpatialChecker {
            even: value.even.into(),
            odd: value.odd.into(),
            density: value.density,
        }
    }
}

/// A standard checkered texture based on spatial 3D texturing.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde-derive", serde(from = "SpatialCheckerInit"))]
pub struct SpatialChecker {
    /// Uniform color for the even-numbered checkers of the texture.
    pub even: Xyz<E>,
    /// Uniform color for the odd-numbered checkers of the texture.
    pub odd: Xyz<E>,
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
        SpatialChecker {
            even: color1.into(),
            odd: color2.into(),
            density,
        }
    }
}

impl TextureTrait for SpatialChecker {
    /// Evaluates the color at the given spatial position coordinate. Note that the `SpatialChecker` is spatial - surface coordinates are ignored.
    #[must_use]
    fn color(&self, _u: Float, _v: Float, position: Vec3) -> Xyz<E> {
        // TODO: convert ahead-of-time. NOTE: take into account serde-i-fication; not enough to do in `new` alone
        let density = self.density * PI;
        let sines = 1.0 // cosmetic 1 for readability of following lines :)
            * (density * position.x).sin()
            * (density * position.y).sin()
            * (density * position.z).sin();
        if sines < 0.0 {
            self.odd.into_color_unclamped()
        } else {
            self.even.into_color_unclamped()
        }
    }
}
