use super::{SolidColor, Texture};
use crate::{color::Color, Float, Vec3};

use std::sync::Arc;
pub struct Checkered {
    // TODO: get recursive textures back, maybe?
    even: SolidColor,
    odd: SolidColor,
    density: Float,
}

impl Checkered {
    pub fn new(color1: SolidColor, color2: SolidColor, density: Float) -> Checkered {
        Checkered {
            even: color1,
            odd: color2,
            density,
        }
    }

    pub fn color(
        even: SolidColor,
        odd: SolidColor,
        density: Float,
        u: Float,
        v: Float,
        position: Vec3,
    ) -> Color {
        let sines = (density * position.x).sin()
            * (density * position.y).sin()
            * (density * position.z).sin();
        if sines < 0.0 {
            // return odd.color(u, v, position); TODO:
            return odd.color;
        } else {
            // return even.color(u, v, position); TODO:
            return even.color;
        }
    }
}
