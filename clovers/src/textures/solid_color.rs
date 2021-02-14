use std::sync::Arc;

use super::{Texture, TextureTrait};
use crate::{color::Color, Float, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Deserialize, Serialize, Debug, Default)]
pub struct SolidColor {
    pub color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Arc<dyn TextureTrait> {
        Arc::new(SolidColor { color })
    }
}

impl TextureTrait for SolidColor {
    fn color(&self, _u: Float, _v: Float, _position: Vec3) -> Color {
        self.color
    }
}
