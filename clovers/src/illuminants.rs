#![allow(non_camel_case_types)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::excessive_precision)]
#![allow(clippy::approx_constant)]

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

include!(concat!(env!("OUT_DIR"), "/FL1.rs"));
include!(concat!(env!("OUT_DIR"), "/FL2.rs"));
include!(concat!(env!("OUT_DIR"), "/FL3.rs"));
include!(concat!(env!("OUT_DIR"), "/FL4.rs"));
include!(concat!(env!("OUT_DIR"), "/FL5.rs"));
include!(concat!(env!("OUT_DIR"), "/FL6.rs"));
include!(concat!(env!("OUT_DIR"), "/FL7.rs"));
include!(concat!(env!("OUT_DIR"), "/FL8.rs"));
include!(concat!(env!("OUT_DIR"), "/FL9.rs"));
include!(concat!(env!("OUT_DIR"), "/FL10.rs"));
include!(concat!(env!("OUT_DIR"), "/FL11.rs"));
include!(concat!(env!("OUT_DIR"), "/FL12.rs"));

include!(concat!(env!("OUT_DIR"), "/FL3_1.rs"));
include!(concat!(env!("OUT_DIR"), "/FL3_2.rs"));
include!(concat!(env!("OUT_DIR"), "/FL3_3.rs"));
include!(concat!(env!("OUT_DIR"), "/FL3_4.rs"));
include!(concat!(env!("OUT_DIR"), "/FL3_5.rs"));
include!(concat!(env!("OUT_DIR"), "/FL3_6.rs"));
include!(concat!(env!("OUT_DIR"), "/FL3_7.rs"));
include!(concat!(env!("OUT_DIR"), "/FL3_8.rs"));
include!(concat!(env!("OUT_DIR"), "/FL3_9.rs"));
include!(concat!(env!("OUT_DIR"), "/FL3_10.rs"));
include!(concat!(env!("OUT_DIR"), "/FL3_11.rs"));
include!(concat!(env!("OUT_DIR"), "/FL3_12.rs"));
include!(concat!(env!("OUT_DIR"), "/FL3_13.rs"));
include!(concat!(env!("OUT_DIR"), "/FL3_14.rs"));
include!(concat!(env!("OUT_DIR"), "/FL3_15.rs"));

fn default_intensity() -> Float {
    1.0
}
