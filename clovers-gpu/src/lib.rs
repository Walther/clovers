//! TODO: documentation
#![cfg_attr(
    target_arch = "spirv",
    no_std,
    feature(register_attr),
    register_attr(spirv)
)]
#![deny(warnings)]
// TODO: temporary during development
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::all)]

use spirv_std::glam::{vec2, vec4, Vec2, Vec4};

use clovers::{
    color::Color,
    hitrecord::GPUHitRecord,
    materials::{GPULambertian, GPUMaterial, GPUMaterialKind, GPUScatterRecord},
    ray::Ray,
    textures::GPUTexture,
    textures::GPUTextureKind,
    CloversRng, Float, FloatTrait, Vec3,
};

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
    // TODO: actual shader for the raytracing

    let x = in_frag_coord.x;
    let y = in_frag_coord.y;
    let width = constants.width as f32;
    let height = constants.height as f32;

    // Surface coordinates
    let u: Float = x / width;
    let v: Float = y / height;

    let seed: u32 = y.trunc() as u32 * constants.width + x.trunc() as u32;
    let mut rng = CloversRng::from_seed(seed);
    let first: Float = rng.gen::<Float>();

    // TODO: actual 3d position
    let position: Vec3 = Vec3::new(u, v, 1.0);

    // Texture
    let color1 = Color::new(0.82, 0.82, 0.82);
    let color2 = Color::new(0.18, 0.18, 0.18);

    let density: Float = 10.0;
    // TODO: better ergonomics...
    let texture: GPUTexture = GPUTexture {
        kind: GPUTextureKind::SurfaceChecker,
        even: color1,
        odd: color2,
        density,
    };
    let color = texture.color(u, v, position);

    // Material
    let _lambertian: GPULambertian = GPULambertian { albedo: texture };
    let refractive_index: Float = 1.5;
    let fuzz: Float = 0.0;
    let material: GPUMaterial = GPUMaterial {
        kind: GPUMaterialKind::Lambertian,
        emit: texture,    // ignored
        refractive_index, // ignored
        color: color1,    // ignored
        albedo: texture,
        fuzz, // ignored
    };
    let ray: Ray = Ray::new(Vec3::new(u, v, 0.0), Vec3::new(u, v, 1.0).normalize(), 0.0);
    let normal: Vec3 = Vec3::new(u, v, -1.0).normalize();
    let distance: Float = 1.0;
    let hit_record: GPUHitRecord = GPUHitRecord {
        distance,
        position,
        normal,
        u,
        v,
        material,
        front_face: 1,
    };

    // TODO: currently requires a forked `rust-gpu` with more aggressive inlining
    let scatter_record: GPUScatterRecord = material.scatter(&ray, &hit_record, &mut rng);
    let color = scatter_record.attenuation;

    *output = vec4(color.r, color.g, color.b, 1.0);
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
