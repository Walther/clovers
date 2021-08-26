//! TODO: documentation
#![cfg_attr(
    target_arch = "spirv",
    no_std,
    feature(register_attr),
    register_attr(spirv)
)]
#![deny(warnings)]
// TODO: temporary during development
#![allow(clippy::all)]

use spirv_std::glam::{vec2, vec4, Vec2, Vec4};
pub struct ShaderConstants {
    pub width: u32,
    pub height: u32,
    pub samples: u32,
    pub max_depth: u32,
    pub time: f32,
}
#[cfg(not(target_arch = "spirv"))]
use spirv_std::macros::spirv;

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] in_frag_coord: Vec4,
    #[spirv(push_constant)] constants: &ShaderConstants,
    output: &mut Vec4,
) {
    let x = in_frag_coord.x;
    let y = in_frag_coord.y;
    let width = constants.width as f32;
    let height = constants.height as f32;
    let a = x / width;
    let b = y / height;
    let c = (x as u32 | y as u32) as f32;
    *output = vec4(a, b, c, 1.0);
}

#[spirv(vertex)]
/// TODO:
pub fn main_vs(#[spirv(vertex_index)] vert_idx: i32, #[spirv(position)] builtin_pos: &mut Vec4) {
    // Create a "full screen triangle" by mapping the vertex index.
    // ported from https://www.saschawillems.de/blog/2016/08/13/vulkan-tutorial-on-rendering-a-fullscreen-quad-without-buffers/
    let uv = vec2(((vert_idx << 1) & 2) as f32, (vert_idx & 2) as f32);
    let pos = 2.0 * uv - Vec2::ONE;

    *builtin_pos = pos.extend(0.0).extend(1.0);
}
