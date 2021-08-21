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
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
// TODO:
#![allow(clippy::many_single_char_names)] // Lots of places with coordinates etc

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
pub mod shaders;
pub mod textures;

// Handy aliases for internal use

/// Internal type alias: this allows the crate to easily switch between float precision without modifying a lot of files.
pub type Float = f32;
/// Internal helper: re-exports the pi constant as our internal [Float] type. TODO: selectable at run time instead of build time?
pub const PI: Float = std::f32::consts::PI;
/// Internal type alias: a nalgebra [Vector3](nalgebra::Vector3) which is a vector with three dimensions, containing three of our internal [Float] types
pub type Vec3 = Vector3<Float>;
/// Internal const: epsilon used for avoiding "shadow acne". See e.g. [Raytracing In One Weekend](https://raytracing.github.io/)
pub const EPSILON_SHADOW_ACNE: Float = 0.001;
/// Internal const: epsilon used for having a finitely-sized thickness for the bounding box of an infinitely-thin rectangle
pub const EPSILON_RECT_THICKNESS: Float = 0.0001;
/// Internal const: epsilon used in the hit calculation of a [ConstantMedium](objects::constant_medium::ConstantMedium)
pub const EPSILON_CONSTANT_MEDIUM: Float = 0.0001;
