use super::{Texture, TextureTrait};
use crate::{color::Color, perlin::Perlin, Float, Vec3};
use serde::{Deserialize, Serialize};

// TODO: This might be currently oddly broken and resulting in overflowy surfaces
#[derive(Copy, Clone, Deserialize, Serialize, Debug)]
pub struct NoiseTexture {
    #[serde(skip)]
    noise: Perlin,
    scale: Float,
}

impl NoiseTexture {
    pub fn new(noise: Perlin, scale: Float) -> dyn TextureTrait {
        Texture::NoiseTexture(NoiseTexture { noise, scale })
    }
}

impl TextureTrait for NoiseTexture {
    // TODO: investigate why this does not swirl as well as the example marble in tutorial
    fn color(self, _u: Float, _v: Float, position: Vec3) -> Color {
        let depth = 7;
        Color::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0
                + (self.scale * position.z + 10.0 * self.noise.turbulence(position, depth)).sin())
    }
}
