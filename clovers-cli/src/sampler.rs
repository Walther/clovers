use std::fmt::Display;

use clap::ValueEnum;
use clovers::{wavelength::Wavelength, Float, Vec2};

pub mod blue;
pub mod random;

pub trait SamplerTrait<'scene> {
    // TODO: better types
    fn sample(&mut self, i: i32, j: i32, index: i32) -> Sample;
}

/// A collection of random values to be used for each sample. Returned as a struct to ensure the correct sampling order for the underlying source of randomness.
pub struct Sample {
    /// Intra-pixel x,y offset. Used for antialiasing.
    pub pixel_offset: Vec2,
    /// The x,y offset used in the lens equations for aperture / depth-of-field simulation
    pub lens_offset: Vec2,
    /// The time of the ray, in range 0..1
    pub time: Float,
    /// Wavelength of the ray
    pub wavelength: Wavelength,
}

#[derive(Clone, Debug, PartialEq, ValueEnum)]
pub enum Sampler {
    Blue,
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
