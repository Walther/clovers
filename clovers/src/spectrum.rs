//! Utilities for [Physically Meaningful Rendering using Tristimulus Colours](https://doi.org/10.1111/cgf.12676)

#![allow(clippy::cast_precision_loss)]

use crate::{
    colors::{Wavelength, XYZ_Tristimulus},
    Float,
};

use self::spectra_xyz_5nm_380_780_097::equal_energy_reflectance;

pub mod spectra_xyz_5nm_380_780_097;
pub mod spectrum_grid;

/// Evaluate the spectrum at the given wavelength for the given XYZ color
#[must_use]
pub fn spectrum_xyz_to_p(lambda: Wavelength, xyz: XYZ_Tristimulus) -> Float {
    // Currently, the data is only built for 5nm intervals. Find a nearby value
    // TODO: generate a file with 1nm intervals?
    let lambda = (lambda / 5) * 5;
    let lambda: f64 = lambda as f64;
    let xyz: [f64; 3] = [f64::from(xyz.x), f64::from(xyz.y), f64::from(xyz.z)];
    let p = spectrum_grid::spectrum_xyz_to_p(lambda, xyz) / equal_energy_reflectance;
    // Scale due to 5nm intervals
    let p = p / 5.0;

    #[allow(clippy::cast_possible_truncation)]
    let p = p as Float;
    p
}
