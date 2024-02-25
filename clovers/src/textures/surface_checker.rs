//! Checkered texture based on the surface coordinates of an object.

use palette::convert::IntoColorUnclamped;
use palette::white_point::E;
use palette::Xyz;

use super::{ColorInit, TextureTrait, TypedColorInit};
use crate::{Float, Vec3, PI};

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

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// A standard checkered texture based on 2D surface UV coordinates.
pub struct SurfaceChecker {
    /// Uniform color for the even-numbered checkers of the texture.
    pub(crate) even: Xyz<E>,
    /// Uniform color for the odd-numbered checkers of the texture.
    pub(crate) odd: Xyz<E>,
    /// Controls the density of the checkered pattern. Default value is 10, which corresponds to using 10 tiles over the width of the object. On spheres, this means 10 tiles around the sphere.
    pub(crate) density: Float,
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
        SurfaceChecker {
            even: color1.into(),
            odd: color2.into(),
            density,
        }
    }
}

impl TextureTrait for SurfaceChecker {
    /// Evaluates the color at the given surface position coordinates. Note that `SurfaceChecker` is surface-based, and thus ignores the spatial position coordinate.
    #[must_use]
    fn color(&self, u: Float, v: Float, _position: Vec3) -> Xyz<E> {
        // TODO: convert ahead-of-time. NOTE: take into account serde-i-fication; not enough to do in `new` alone
        let density = self.density * PI;
        let sines = 1.0 // cosmetic 1 for readability of following lines :)
              * (density * u).sin()
              * (density * v).sin();
        if sines < 0.0 {
            self.odd.into_color_unclamped()
        } else {
            self.even.into_color_unclamped()
        }
    }
}
