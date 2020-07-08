use super::Texture;
use crate::{color::Color, Float, Vec3};

#[derive(Copy, Clone)]
pub struct SolidColor {
    pub color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Texture {
        Texture::SolidColor(SolidColor { color })
    }

    pub fn color(self, _u: Float, _v: Float, _position: Vec3) -> Color {
        self.color
    }
}
