//! Checkered texture based on the surface coordinates of an object.

use palette::convert::IntoColorUnclamped;
use palette::{LinSrgb, Srgb};

use super::TextureTrait;
use crate::{Float, Vec3, PI};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// A standard checkered texture based on 2D surface UV coordinates.
pub struct SurfaceChecker {
    #[cfg_attr(feature = "serde-derive", serde(default = "default_even"))]
    pub(crate) even: Srgb,
    #[cfg_attr(feature = "serde-derive", serde(default = "default_odd"))]
    pub(crate) odd: Srgb,
    #[cfg_attr(feature = "serde-derive", serde(default = "default_density_surface"))]
    /// Controls the density of the checkered pattern. Default value is 10, which corresponds to using 10 tiles over the width of the object. On spheres, this means 10 tiles around the sphere.
    pub(crate) density: Float,
}

#[cfg(feature = "serde-derive")]
fn default_even() -> Srgb {
    // White minus middle gray 18%

    LinSrgb::new(0.82, 0.82, 0.82).into_color_unclamped()
}

#[cfg(feature = "serde-derive")]
fn default_odd() -> Srgb {
    // Middle gray 18%
    LinSrgb::new(0.18, 0.18, 0.18).into_color_unclamped()
}

#[cfg(feature = "serde-derive")]
fn default_density_surface() -> Float {
    10.0
}

impl SurfaceChecker {
    /// Create a new `SurfaceChecker` object with the specified colors and density.
    #[must_use]
    pub fn new(color1: Srgb, color2: Srgb, density: Float) -> Self {
        SurfaceChecker {
            even: color1,
            odd: color2,
            density,
        }
    }
}

impl TextureTrait for SurfaceChecker {
    /// Evaluates the color at the given surface position coordinates. Note that `SurfaceChecker` is surface-based, and thus ignores the spatial position coordinate.
    #[must_use]
    fn color(&self, u: Float, v: Float, _position: Vec3) -> LinSrgb {
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
