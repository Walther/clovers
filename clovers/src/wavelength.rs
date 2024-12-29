//! The fundamental building blocks of spectral rendering.

use core::{array::from_fn, ops::Range};
use palette::{white_point::E, Xyz};
use rand::rngs::SmallRng;
use rand_distr::uniform::SampleRange;

use crate::Float;

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

// TODO: clippy fixes possible?
/// Given a sample seed from a sampler, return the approximate wavelenght.
///
/// # Panics
/// This method may panic if the runtime asserts are triggered. This would indicate a bug in the implementation.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
pub fn sample_wavelength(sample: Float) -> Wavelength {
    let pick = (sample * SPECTRUM_SIZE as Float).floor() as usize + MIN_WAVELENGTH;
    assert!(pick <= MAX_WAVELENGTH);
    assert!(pick >= MIN_WAVELENGTH);
    pick
}

/// Given a hero wavelength, create additional equidistant wavelengths in the visible spectrum. Returns an array of wavelengths, with the original hero wavelength as the first one.
#[must_use]
pub fn rotate_wavelength(hero: Wavelength) -> [Wavelength; WAVE_SAMPLE_COUNT] {
    from_fn(|j| {
        let step = j * SPECTRUM_SIZE / WAVE_SAMPLE_COUNT;
        MIN_WAVELENGTH + (hero - MIN_WAVELENGTH + step) % SPECTRUM_SIZE
    })
}

/// Helper function adapted from <https://en.wikipedia.org/wiki/CIE_1931_color_space#Analytical_approximation>
fn gaussian(x: Float, alpha: Float, mu: Float, sigma1: Float, sigma2: Float) -> Float {
    let t = (x - mu) / (if x < mu { sigma1 } else { sigma2 });
    alpha * (-(t * t) / 2.0).exp()
}

/// Helper function adapted from <https://en.wikipedia.org/wiki/CIE_1931_color_space#Analytical_approximation>
#[allow(clippy::cast_precision_loss)]
#[must_use]
pub fn wavelength_into_xyz(lambda: Wavelength) -> Xyz<E> {
    // With the wavelength λ measured in nanometers, we then approximate the 1931 color matching functions:
    let l: Float = lambda as Float;
    let x = 0.0 // for readability of next lines
        + gaussian(l, 1.056, 599.8, 37.9, 31.0)
        + gaussian(l, 0.362, 442.0, 16.0, 26.7)
        + gaussian(l, -0.065, 501.1, 20.4, 26.2);
    let y = gaussian(l, 0.821, 568.8, 46.9, 40.5) + gaussian(l, 0.286, 530.9, 16.3, 31.1);
    let z = gaussian(l, 1.217, 437.0, 11.8, 36.0) + gaussian(l, 0.681, 459.0, 26.0, 13.8);

    // The functions above have been designed for the whitepoint E
    Xyz::<E>::new(x, y, z)
}
