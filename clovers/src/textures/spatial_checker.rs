//! Checkered texture based on the world coordinates.

// TODO: object-aligned spatial checker?

use super::TextureTrait;
use crate::{color::Color, Float, Vec3, PI};

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// A standard checkered texture based on spatial 3D texturing.
pub struct SpatialChecker {
    #[cfg_attr(feature = "serde-derive", serde(default = "default_even"))]
    /// Uniform color for the even-numbered checkers of the texture.
    pub even: Color,
    #[cfg_attr(feature = "serde-derive", serde(default = "default_odd"))]
    /// Uniform color for the odd-numbered checkers of the texture.
    pub odd: Color,
    #[cfg_attr(feature = "serde-derive", serde(default = "default_density_spatial"))]
    /// Controls the density of the checkered pattern. Default value is 1.0, which corresponds to filling a 1.0 unit cube in the coordinate system with one color of the pattern. Even values preferred - odd values may create a visually thicker stripe due to two stripes with same color being next to each other.
    pub density: Float,
}

#[cfg(feature = "serde-derive")]
fn default_even() -> Color {
    // White minus middle gray 18%
    Color::new(0.82, 0.82, 0.82)
}

#[cfg(feature = "serde-derive")]
fn default_odd() -> Color {
    // Middle gray 18%
    Color::new(0.18, 0.18, 0.18)
}

#[cfg(feature = "serde-derive")]
fn default_density_spatial() -> Float {
    1.0
}

impl SpatialChecker {
    /// Create a new `SpatialChecker` object with the specified colors and density.
    #[must_use]
    pub fn new(color1: Color, color2: Color, density: Float) -> Self {
        SpatialChecker {
            even: color1,
            odd: color2,
            density,
        }
    }
}

impl TextureTrait for SpatialChecker {
    /// Evaluates the color at the given spatial position coordinate. Note that the `SpatialChecker` is spatial - surface coordinates are ignored.
    #[must_use]
    fn color(&self, _u: Float, _v: Float, position: Vec3) -> Color {
        // TODO: convert ahead-of-time. NOTE: take into account serde-i-fication; not enough to do in `new` alone
        let density = self.density * PI;
        let sines = 1.0 // cosmetic 1 for readability of following lines :)
            * (density * position.x).sin()
            * (density * position.y).sin()
            * (density * position.z).sin();
        if sines < 0.0 {
            self.odd
        } else {
            self.even
        }
    }
}