//! The fundamental building blocks of spectral rendering.

use core::{array::from_fn, ops::Range};
use rand::rngs::SmallRng;
use rand_distr::uniform::SampleRange;

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

const MIN_WAVELENGTH: Wavelength = 380;
const MAX_WAVELENGTH: Wavelength = 780;
const SPECTRUM: Range<Wavelength> = MIN_WAVELENGTH..MAX_WAVELENGTH;
const SPECTRUM_SIZE: usize = MAX_WAVELENGTH - MIN_WAVELENGTH;
const WAVE_SAMPLE_COUNT: usize = 4;

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
