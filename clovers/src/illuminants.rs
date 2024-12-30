//! Spectral power distributions of various illuminants.

use crate::ray::Ray;
use crate::textures::TextureTrait;
use crate::wavelength::Wavelength;
use crate::{Float, HitRecord};

include!(concat!(env!("OUT_DIR"), "/D50.rs"));
include!(concat!(env!("OUT_DIR"), "/D65.rs"));

fn default_intensity() -> Float {
    1.0
}
