//! A solid color texture.

use palette::{convert::IntoColorUnclamped, LinSrgb, Srgb};

use crate::{Float, Vec3};

use super::TextureTrait;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// A solid color texture. Simplest possible [Texture](crate::textures::Texture): returns a solid color at any surface coordinate or spatial position.
pub struct SolidColor {
    /// The color of the [Texture](crate::textures::Texture).
    pub color: Srgb,
}

impl TextureTrait for SolidColor {
    /// Evaluates the color ignoring the given surface coordinates and spatial position - always returns the solid color.
    #[must_use]
    fn color(&self, _u: Float, _v: Float, _position: Vec3) -> LinSrgb {
        self.color.into_color_unclamped()
    }
}

impl SolidColor {
    /// Creates a new solid color texture with the specified color.
    #[must_use]
    pub fn new(color: Srgb) -> Self {
        SolidColor { color }
    }
}

impl Default for SolidColor {
    fn default() -> Self {
        // 18% grey
        Self {
            color: LinSrgb::new(0.18, 0.18, 0.18).into_color_unclamped(),
        }
    }
}
