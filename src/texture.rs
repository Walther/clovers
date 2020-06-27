use crate::{color::Color, perlin::Perlin, Float, Vec3};
use std::sync::Arc;

pub trait Texture: Sync + Send {
    fn color(&self, u: Float, v: Float, position: Vec3) -> Color;
}

impl dyn Texture {}

#[derive(Clone)]
pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> SolidColor {
        SolidColor { color }
    }
}

impl Texture for SolidColor {
    fn color(&self, _u: Float, _v: Float, _position: Vec3) -> Color {
        self.color
    }
}

pub struct Checkered {
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
    density: Float,
}

impl Checkered {
    pub fn new(
        texture1: Arc<dyn Texture>,
        texture2: Arc<dyn Texture>,
        density: Float,
    ) -> Checkered {
        Checkered {
            even: Arc::clone(&texture1),
            odd: Arc::clone(&texture2),
            density,
        }
    }
}

impl Texture for Checkered {
    fn color(&self, u: Float, v: Float, position: Vec3) -> Color {
        let sines = (self.density * position.x).sin()
            * (self.density * position.y).sin()
            * (self.density * position.z).sin();
        if sines < 0.0 {
            return self.odd.color(u, v, position);
        } else {
            return self.even.color(u, v, position);
        }
    }
}

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
