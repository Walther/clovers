#![deny(clippy::all)]

// Externals
use nalgebra::Vector3;

// Internals
pub mod aabb;
pub mod bvhnode;
pub mod camera;
pub mod color;
pub mod colorize;
pub mod hitable;
pub mod materials;
pub mod objects;
pub mod onb;
pub mod pdf;
pub mod perlin;
pub mod random;
pub mod ray;
pub mod scenes;
pub mod textures;

// Handy aliases for internal use
pub type Float = f32;
pub const PI: Float = std::f32::consts::PI as Float;
pub type Vec3 = Vector3<Float>;
pub const SHADOW_EPSILON: Float = 0.001;
pub const RECT_EPSILON: Float = 0.0001;
pub const CONSTANT_MEDIUM_EPSILON: Float = 0.0001;
