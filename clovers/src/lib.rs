//! # clovers - ray tracing in rust!
//!
//! **Note**: This library is experimental & heavily work in progress. Everything can change at a moment's notice. It is probably not a good idea to use this library for anything other than experimentation for now!
//!
//! This project uses GitHub for development and issue tracking. [Link to the repository](https://github.com/Walther/clovers).
//!
//! # Guiding thoughts
//! - Keep it clean: prefer good abstractions, avoid deep integration
//! - Platform agnostic: hopefully runnable by both CPU and GPU, on desktop and WebAssembly, etc
//! - Prefer correctness: no "cheating" optimizations / approximations
//! - Look for beautiful light <3
//!
//! # How it works
//!
//! There are a few core stages of using clovers.
//!
//! ## Creating and Loading a Scene
//!
//! First, you will need a [Scene](scenes::Scene). You can create a scene manually or utilize [serde](https://docs.serde.rs/) to deserialize from a file. Currently, the example binary uses a JSON format.
//!
//! - [Scenes](scenes::Scene) have [Objects](objects::Object)
//! - [Objects](objects::Object) have a [Material](materials::Material)
//! - [Materials](materials::Material) usually have a [Texture](textures::Texture)
//! - Materials and Textures may have unique paramteres to adjust
//!
//!
//! ## Rendering the Scene
//!
//! clovers is not opinionated on how you want to render your scene. In a usual scenario, you probably want to have some form of a pixel buffer, with knowledge of the `x` and `y` coordinates of your buffer.
//!
//! - Rendering is done by creating [Rays](ray::Ray) and seeing what they hit
//! - A [Ray](ray::Ray) has an origin and a direction
//! - Every [Object](objects::Object) has a `hit()` method that takes a [Ray](ray::Ray) and returns an Option<[HitRecord](hitable::HitRecord)>
//! - If you get None, use that information to colorize your pixel with a default color
//! - If you get Some([HitRecord](hitable::HitRecord)), use its details to colorize your pixel
//! - You most likely also want to recurse: depending on the material, maybe `scatter()` and cast a new [Ray](ray::Ray)?
//!
//! You most likely want to repeat this process multiple times for each of your pixels: generating multiple samples per pixel results in a higher quality image.
//!
//! The library provides an opinionated [colorize()](colorize::colorize) function that does the steps mentioned above. Using it is optional - feel free to implement your own methods that utilize the lower-level building blocks for more creative power!
//!
//! ## Post processing
//!
//! **TODO:** maybe add some post processing utilities?
//! - denoise support?
//! - 3D & rendering aware effects?
//! - etc
//!
//! ## Using the result
//!
//! At the end, use your pixel buffer - save to an image file, draw a frame in a GUI window, etc.

// Lints
#![deny(clippy::all)]
#![deny(explicit_outlives_requirements)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unsafe_code)]
#![deny(unused_lifetimes)]
#![deny(unused_qualifications)]
// #![deny(missing_debug_implementations)] // TODO: how to have this for standard builds but not for gpu builds?
#![deny(missing_docs)]
// TODO: Lots of places with coordinates etc
#![allow(clippy::many_single_char_names)]
//

// TODO: make these cfg bits cleaner and nicer. Lots of cfg mess here in order to build spirv target support bit by bit
#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(not(target_arch = "spirv"))]
extern crate alloc;
#[cfg(not(target_arch = "spirv"))]
pub use alloc::boxed::Box;
#[cfg(not(target_arch = "spirv"))]
pub use alloc::vec::Vec;

// Externals
#[cfg(feature = "nalg")]
use nalgebra::base::Vector3;
#[cfg(target_arch = "spirv")]
pub use spirv_std::glam::Vec3;

// Internals: GPU compatible
pub mod aabb;
pub mod camera;
pub mod color;
pub mod interval;
pub mod onb;
pub mod random;
pub mod ray;
pub mod textures; // TODO: partial support

// Internals: not gpu compatible yet
#[cfg(not(target_arch = "spirv"))]
pub mod bvhnode;
#[cfg(not(target_arch = "spirv"))]
pub mod colorize;
#[cfg(not(target_arch = "spirv"))]
pub mod hitable;
#[cfg(not(target_arch = "spirv"))]
pub mod materials;
#[cfg(not(target_arch = "spirv"))]
pub mod normals;
#[cfg(not(target_arch = "spirv"))]
pub mod objects;
#[cfg(not(target_arch = "spirv"))]
pub mod pdf;
#[cfg(not(target_arch = "spirv"))]
pub mod perlin;
#[cfg(not(target_arch = "spirv"))]
pub mod scenes;

/// Rendering options struct
#[derive(Clone, Copy)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
pub struct RenderOpts {
    /// Width of the render in pixels
    pub width: u32,
    /// Height of the render in pixels
    pub height: u32,
    /// Samples per pixel to render for multisampling. Higher number implies higher quality.
    pub samples: u32,
    /// Maximum ray bounce depth. Higher number implies higher quality.
    pub max_depth: u32,
    /// Gamma correction value
    pub gamma: Float,
    /// Optionally, suppress CLI output
    pub quiet: bool,
    /// Experimental render mode: return a normal map only instead of doing a full path trace render.
    pub normalmap: bool,
}

// Handy aliases for internal use

/// Internal type alias: this allows the crate to easily switch between float precision without modifying a lot of files.
pub type Float = f32;
/// Internal helper: re-exports the pi constant as our internal [Float] type. TODO: selectable at run time instead of build time?
pub const PI: Float = core::f32::consts::PI;
/// Internal type alias: a nalgebra [Vector3](nalgebra::Vector3) which is a vector with three dimensions, containing three of our internal [Float] types
#[cfg(feature = "nalg")]
pub type Vec3 = Vector3<Float>;
/// Internal const: epsilon used for avoiding "shadow acne". This is mostly used for the initial minimum distance for ray hits after reflecting or scattering from a surface.
pub const EPSILON_SHADOW_ACNE: Float = 0.001;
/// Internal const: epsilon used for having a finitely-sized thickness for the bounding box of an infinitely-thin rectangle. Shouldn't be too small.
pub const EPSILON_RECT_THICKNESS: Float = 0.000_1;
/// Internal const: epsilon used in the hit calculation of a [ConstantMedium](objects::constant_medium::ConstantMedium).
// TODO: what would be an appropriate value?
pub const EPSILON_CONSTANT_MEDIUM: Float = 0.000_1;
/// Internal alias: allows swapping the implementation for CPU vs GPU
#[cfg(feature = "rand-crate")]
#[cfg(not(target_arch = "spirv"))]
pub use rand::rngs::SmallRng as CloversRng;
#[cfg(not(feature = "rand-crate"))]
pub use random::CloversRng;
