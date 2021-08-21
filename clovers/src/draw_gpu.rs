#![cfg_attr(
    target_arch = "spirv",
    no_std,
    feature(register_attr),
    register_attr(spirv)
)]
// spirv errors
#![deny(warnings)]
// TODO: temporary during development
#![allow(dead_code)]

use clovers::{color::Color, scenes::Scene, Float};
use spirv_std::glam::{vec4, Vec4};

#[cfg(not(target_arch = "spirv"))]
use spirv_std::macros::spirv;

#[spirv(fragment)]
pub fn main_fs(output: &mut Vec4) {
    *output = vec4(1.0, 0.0, 0.0, 1.0);
}

#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_id: i32,
    #[spirv(position, invariant)] out_pos: &mut Vec4,
) {
    *out_pos = vec4(
        (vert_id - 1) as f32,
        ((vert_id & 1) * 2 - 1) as f32,
        0.0,
        1.0,
    );
}

/// The main drawing function, returns a Vec<Color> as a pixelbuffer.
pub fn draw(
    width: u32,
    height: u32,
    _samples: u32,
    _max_depth: u32,
    _gamma: Float,
    _quiet: bool,
    _scene: Scene,
) -> Vec<Color> {
    // TODO: implement this function!
    // this needs to call the main_fs somehow?
    // probably need to initialize a gpu context first too

    let pixels = (width * height) as u64;
    let black = Color::new(0.0, 0.0, 0.0);
    let pixelbuffer = vec![black; pixels as usize];
    pixelbuffer
}
