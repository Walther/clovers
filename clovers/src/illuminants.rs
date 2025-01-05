#![allow(clippy::unreadable_literal)]
#![allow(non_camel_case_types)]

//! Spectral power distributions of various illuminants.

use crate::ray::Ray;
use crate::textures::TextureTrait;
use crate::wavelength::Wavelength;
use crate::{Float, HitRecord};

include!(concat!(env!("OUT_DIR"), "/D50.rs"));
include!(concat!(env!("OUT_DIR"), "/D65.rs"));
include!(concat!(env!("OUT_DIR"), "/LED_B1.rs"));
include!(concat!(env!("OUT_DIR"), "/LED_B2.rs"));
include!(concat!(env!("OUT_DIR"), "/LED_B3.rs"));
include!(concat!(env!("OUT_DIR"), "/LED_B4.rs"));
include!(concat!(env!("OUT_DIR"), "/LED_B5.rs"));
include!(concat!(env!("OUT_DIR"), "/LED_BH1.rs"));
include!(concat!(env!("OUT_DIR"), "/LED_RGB1.rs"));
include!(concat!(env!("OUT_DIR"), "/LED_V1.rs"));
include!(concat!(env!("OUT_DIR"), "/LED_V2.rs"));

fn default_intensity() -> Float {
    1.0
}
