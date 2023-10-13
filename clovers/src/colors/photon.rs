//! The fundamental building blocks of spectral rendering.

use core::{array::from_fn, ops::Range};
use rand::rngs::SmallRng;
use rand_distr::uniform::SampleRange;

use crate::Float;

/// A fundamental light particle.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
pub struct Photon {
    /// Wavelength of the photon
    pub wavelength: Wavelength,
    // TODO: spin for polarization
    // pub spin: bool,
}

/// Wavelength in nanometers
pub type Wavelength = usize;

/// The lower bound for the wavelengths, inclusive
pub const MIN_WAVELENGTH: Wavelength = 380;
/// The upper bound for the wavelenghts, exclusive
pub const MAX_WAVELENGTH: Wavelength = 780;
/// The range of wavelenghts used, inclusive low, exclusive high
pub const SPECTRUM: Range<Wavelength> = MIN_WAVELENGTH..MAX_WAVELENGTH;
/// The length of the wavelength spectrum used
pub const SPECTRUM_SIZE: usize = MAX_WAVELENGTH - MIN_WAVELENGTH;
/// The probability of picking a specific wavelength
#[allow(clippy::cast_precision_loss)]
pub const WAVELENGTH_PROBABILITY: Float = 1.0 / (SPECTRUM_SIZE as Float);
/// The count of wavelenghts used per ray in Hero Wavelength Sampling
pub const WAVE_SAMPLE_COUNT: usize = 4;

/// Return a random wavelength, sampled uniformly from the visible spectrum.
pub fn random_wavelength(rng: &mut SmallRng) -> Wavelength {
    SPECTRUM.sample_single(rng)
}

/// Given a hero wavelength, create additional equidistant wavelengths in the visible spectrum. Returns an array of wavelengths, with the original hero wavelength as the first one.
#[must_use]
pub fn rotate_wavelength(hero: Wavelength) -> [Wavelength; WAVE_SAMPLE_COUNT] {
    from_fn(|j| {
        (hero - MIN_WAVELENGTH + (j * SPECTRUM_SIZE / WAVE_SAMPLE_COUNT)) % SPECTRUM_SIZE
            + MIN_WAVELENGTH
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_wavelength_spread() {
        let hero = 380;
        let rotations = rotate_wavelength(hero);
        let expected = [380, 480, 580, 680];
        assert_eq!(rotations, expected);
    }
}
