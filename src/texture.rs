use crate::{color::Color, Float, Vec3};

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
