//! A sampler based on a random number generator. This is the default sampler used in this renderer. It works especially well at high samples-per-pixel counts.

use clovers::{random::random_in_unit_disk, wavelength::random_wavelength, Vec2};
use rand::{rngs::SmallRng, Rng};

use super::{Randomness, SamplerTrait};

#[derive(Debug)]
pub struct RandomSampler<'scene> {
    rng: &'scene mut SmallRng,
}

impl<'scene> RandomSampler<'scene> {
    pub fn new(rng: &'scene mut SmallRng) -> Self {
        Self { rng }
    }
}

impl<'scene> SamplerTrait<'scene> for RandomSampler<'scene> {
    fn sample(&mut self, _i: i32, _j: i32, _index: i32) -> Randomness {
        let pixel_offset = Vec2::new(self.rng.random(), self.rng.random());
        let lens_offset = random_in_unit_disk(self.rng);
        let time = self.rng.random();
        let wavelength = random_wavelength(self.rng);

        Randomness {
            pixel_offset,
            lens_offset,
            time,
            wavelength,
        }
    }

    fn sample_dimension(
        &mut self,
        _i: i32,
        _j: i32,
        _index: i32,
        _dimension: super::SamplerDimension,
    ) -> clovers::Float {
        self.rng.random()
    }
}
