//! Sampler architecture for the renderer, based on the sampling infrastructure described in the book Physically Based Rendering, chapter [8.3 Sampling Interface](https://pbr-book.org/4ed/Sampling_and_Reconstruction/Sampling_Interface#Sampler)

use std::fmt::Display;

use clap::ValueEnum;
use clovers::{wavelength::Wavelength, Float, Vec2};

pub mod blue;
pub mod random;

pub trait SamplerTrait<'scene> {
    // TODO: better types
    fn sample(&mut self, i: i32, j: i32, index: i32) -> Randomness;

    /// Manually request a sample from the specific dimension
    #[allow(dead_code)] // TODO: remove
    fn sample_dimension(
        &mut self,
        i: i32,
        j: i32,
        index: i32,
        dimension: SamplerDimension,
    ) -> Float;
}

/// A collection of random values to be used for each sample. Returned as a struct to ensure the correct sampling order for the underlying source of randomness.
pub struct Randomness {
    /// Intra-pixel `(x,y)` offset, both in range `[0..1]`. Used for antialiasing.
    pub pixel_offset: Vec2,
    /// The `(x,y)` offset used in the lens equations for aperture / depth-of-field simulation. The coordinates are within the range `[-0.5..0.5]` and within a unit disk.
    pub lens_offset: Vec2,
    /// The time of the ray, in range `[0..1]`
    pub time: Float,
    /// Wavelength of the ray
    pub wavelength: Wavelength,
}

/// Enum of the supported samplers.
#[derive(Clone, Debug, PartialEq, ValueEnum)]
pub enum Sampler {
    /// Blue noise based sampler, see [BlueSampler](blue::BlueSampler)
    Blue,
    /// Random number generator based sampler, see [RandomSampler](random::RandomSampler)
    Random,
}

impl Display for Sampler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Sampler::Blue => "blue",
            Sampler::Random => "random",
        };
        write!(f, "{s}")
    }
}

/// Various sampling dimensions used by the samplers
#[derive(Clone, Copy)]
pub enum SamplerDimension {
    PixelOffsetX,
    PixelOffsetY,
    LensOffsetX,
    LensOffsetY,
    Time,
    Wavelength,
}
