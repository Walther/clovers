#![deny(clippy::all)]

// Externals
use nalgebra::Vector3;

// Internals
pub mod color;
pub mod hitable;
pub mod objects;
pub mod ray;
pub use ray::Ray;
pub mod aabb;
pub use aabb::AABB;
pub mod bvhnode;
pub use bvhnode::BVHNode;
pub mod camera;
pub mod colorize;
pub mod materials;
pub mod onb;
pub mod pdf;
pub mod perlin;
pub mod random;
pub mod scenes;
pub mod textures;

// Handy aliases for internal use
pub type Float = f32;
pub const PI: Float = std::f32::consts::PI as Float;
pub type Vec3 = Vector3<Float>;
pub const SHADOW_EPSILON: Float = 0.001;
pub const RECT_EPSILON: Float = 0.0001;
pub const CONSTANT_MEDIUM_EPSILON: Float = 0.0001;
