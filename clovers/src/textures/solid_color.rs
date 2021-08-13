//! A solid color texture.

use crate::{color::Color, Float, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Deserialize, Serialize, Debug, Default)]
/// A solid color texture. Simplest possible [Texture](crate::textures::Texture): returns a solid color at any surface coordinate or spatial position.
pub struct SolidColor {
    /// The color of the [Texture](crate::textures::Texture).
    pub color: Color,
}

impl SolidColor {
    /// Creates a new solid color texture with the specified color.
    pub fn new(color: Color) -> Self {
        SolidColor { color }
    }

    /// Evaluates the color ignoring the given surface coordinates and spatial position - always returns the solid color.
    pub fn color(self, _u: Float, _v: Float, _position: Vec3) -> Color {
        self.color
    }
}
