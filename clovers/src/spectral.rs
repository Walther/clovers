//! Spectral rendering functionalities for the renderer

use core::{array::from_fn, ops::Range};
use rand::rngs::SmallRng;
use rand_distr::uniform::SampleRange;

use crate::{color::Color, Float};

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
        (hero - MIN_WAVELENGTH + ((1 + j) / WAVE_SAMPLE_COUNT) * SPECTRUM_SIZE)
            % (SPECTRUM_SIZE + MIN_WAVELENGTH)
    })
}

/// Helper function adapted from <https://en.wikipedia.org/wiki/CIE_1931_color_space#Analytical_approximation>
fn gaussian(x: Float, alpha: Float, mu: Float, sigma1: Float, sigma2: Float) -> Float {
    let t = (x - mu) / (if x < mu { sigma1 } else { sigma2 });
    alpha * (-(t * t) / 2.0).exp()
}

/// Color component transfer function.
/// Note: Produces `sRGB` digital values with a range 0 to 1, which must then be multiplied by 2^(bit depth) – 1 and quantized.
/// <https://color.org/chardata/rgb/sRGB.pdf>
#[must_use]
pub fn color_component_transfer(c: Float) -> Float {
    if c.abs() < 0.003_130_8 {
        12.92 * c
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

/// Helper function adapted from <https://en.wikipedia.org/wiki/CIE_1931_color_space#Analytical_approximation>
impl From<Wavelength> for Color {
    // TODO: precision loss
    #[allow(clippy::cast_precision_loss)]
    fn from(lambda: Wavelength) -> Self {
        // With the wavelength λ measured in nanometers, we then approximate the 1931 color matching functions:
        let l: Float = lambda as Float;
        let x = 0.0 // for readability of next lines
            + gaussian(l, 1.056, 599.8, 37.9, 31.0)
            + gaussian(l, 0.362, 442.0, 16.0, 26.7)
            + gaussian(l, -0.065, 501.1, 20.4, 26.2);
        let y = gaussian(l, 0.821, 568.8, 46.9, 40.5) + gaussian(l, 0.286, 530.9, 16.3, 31.1);
        let z = gaussian(l, 1.217, 437.0, 11.8, 36.0) + gaussian(l, 0.681, 459.0, 26.0, 13.8);
        // Convert from XYZ to sRGB
        // https://color.org/chardata/rgb/sRGB.pdf
        // TODO: more correct color management!
        let r = 3.240_625_5 * x - 1.537_208 * y - 0.498_628_6 * z;
        let g = -0.968_930_7 * x + 1.875_756_1 * y + 0.041_517_5 * z;
        let b = 0.055_710_1 * x - 0.204_021_1 * y + 1.056_995_9 * z;

        Color { r, g, b }
    }
}
