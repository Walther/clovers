//! Initialization structures for colors. This exists for deserialization purposes.

use core::str::FromStr;

use palette::{
    chromatic_adaptation::AdaptInto,
    white_point::{D65, E},
    LinSrgb, Oklch, Srgb, Xyz,
};

use crate::Float;

/// Type safe initialization structure for a color. Can be specified in a variety of color spaces.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde-derive", serde(rename_all = "snake_case"))]
pub enum TypedColorInit {
    /// Hex "web color" Srgb
    Hex(String),
    /// Linear Srgb
    LinSrgb(LinSrgb),
    /// Non-linear Srgb
    Srgb(Srgb),
    /// XYZ, E illuminant
    XyzE(Xyz<E>),
    /// XYZ, D65 illuminant
    XyzD65(Xyz<D65>),
    /// Oklch
    Oklch(Oklch),
    // TODO: add more
}

/// Initialization structure for a color. Contains either an untyped, legacy variant (assumed Srgb) or one of the new type-safe, tagged versions.
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde-derive", serde(untagged))]
#[derive(Clone, Debug)]
pub enum ColorInit {
    /// Legacy color, assume Srgb given as array of three floats in range 0..1 for up to unity gain, >1 for positive gain (illuminating) colors
    Color([Float; 3]),
    /// Typesafe color initialization structure
    TypedColor(TypedColorInit),
}

impl From<ColorInit> for Xyz<E> {
    fn from(val: ColorInit) -> Self {
        // TODO: ensure correctness
        match val {
            ColorInit::Color(c) => Srgb::new(c[0], c[1], c[2]).adapt_into(),
            ColorInit::TypedColor(s) => match s {
                TypedColorInit::Hex(c) => Srgb::from_str(&c).unwrap().into_format().adapt_into(),
                TypedColorInit::LinSrgb(c) => LinSrgb::new(c.red, c.green, c.blue).adapt_into(),
                TypedColorInit::Srgb(c) => Srgb::new(c.red, c.green, c.blue).adapt_into(),
                TypedColorInit::XyzE(c) => c,
                TypedColorInit::XyzD65(c) => c.adapt_into(),
                TypedColorInit::Oklch(c) => Oklch::new(c.l, c.chroma, c.hue).adapt_into(),
            },
        }
    }
}
