use serde::{Deserialize, Serialize};

use super::Texture;
use crate::{color::Color, perlin::Perlin, Float, Vec3};

// TODO: This might be currently oddly broken and resulting in overflowy surfaces
#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct NoiseTexture {
    #[serde(skip)]
    noise: Perlin,
    scale: Float,
}

impl NoiseTexture {
    pub fn new(noise: Perlin, scale: Float) -> Texture {
        Texture::NoiseTexture(NoiseTexture { noise, scale })
    }

    // TODO: investigate why this does not swirl as well as the example marble in tutorial
    pub fn color(self, _u: Float, _v: Float, position: Vec3) -> Color {
        let depth = 7;
        return Color::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0
                + (self.scale * position.z + 10.0 * self.noise.turbulence(position, depth)).sin());
    }
}
