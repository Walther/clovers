pub mod checkered;
pub mod noise_texture;
pub mod solid_color;

use crate::{color::Color, Float, Vec3};

pub trait Texture: Sync + Send {
    fn color(&self, u: Float, v: Float, position: Vec3) -> Color;
}

impl dyn Texture {}
