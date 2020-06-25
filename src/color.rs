use crate::{Float, ThreadRng, Vec3};
use rand::prelude::*;

pub struct Color {
    r: Float,
    g: Float,
    b: Float,
}

impl Color {
    pub fn new(r: Float, g: Float, b: Float) -> Color {
        Color { r, g, b }
    }

    pub fn random(mut rng: ThreadRng) -> Color {
        Color {
            r: rng.gen::<Float>(),
            g: rng.gen::<Float>(),
            b: rng.gen::<Float>(),
        }
    }
}

impl From<Color> for Vec3 {
    fn from(color: Color) -> Vec3 {
        Vec3::new(color.r, color.g, color.b)
    }
}
