use super::Texture;
use crate::{color::Color, perlin::Perlin, Float, Vec3};
use serde::{Deserialize, Serialize};

// TODO: This might be currently oddly broken and resulting in overflowy surfaces
// TODO: better documentation
#[derive(Copy, Clone, Deserialize, Serialize, Debug)]
/// A NoiseTexture object.
pub struct NoiseTexture {
    #[serde(skip)]
    noise: Perlin,
    scale: Float,
}

impl NoiseTexture {
    /// Creates a new NoiseTexture object with the specified Perlin noise and scaling factor.
    pub fn new(noise: Perlin, scale: Float) -> Texture {
        Texture::NoiseTexture(NoiseTexture { noise, scale })
    }

    // TODO: investigate why this does not swirl as well as the example marble in tutorial
    // TODO: spatial vs surface noise alternatives?
    /// Evaluates the color at the given spatial coordinate.
    pub fn color(self, _u: Float, _v: Float, position: Vec3) -> Color {
        let depth = 7;
        Color::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0
                + (self.scale * position.z + 10.0 * self.noise.turbulence(position, depth)).sin())
    }
}
