use super::Texture;
use crate::{color::Color, perlin::Perlin, Float, Vec3};

// TODO: This might be currently oddly broken and resulting in overflowy surfaces
pub struct NoiseTexture {
    noise: Perlin,
    scale: Float,
}

impl Texture for NoiseTexture {
    // TODO: investigate why this does not swirl as well as the example marble in tutorial
    fn color(&self, _u: Float, _v: Float, position: Vec3) -> Color {
        let depth = 7;
        return Color::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0
                + (self.scale * position.z + 10.0 * self.noise.turbulence(position, depth)).sin());
    }
}

impl NoiseTexture {
    pub fn new(noise: Perlin, scale: Float) -> NoiseTexture {
        NoiseTexture { noise, scale }
    }
}
