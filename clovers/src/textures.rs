//! Textures enable different surface textures for colorizing objects in various ways.

pub mod solid_color;
pub mod spatial_checker;
pub mod surface_checker;

use enum_dispatch::enum_dispatch;
use palette::{
    chromatic_adaptation::AdaptInto,
    convert::IntoColorUnclamped,
    white_point::{D65, E},
    LinSrgb, Oklch, Srgb, Xyz,
};
pub use solid_color::*;
pub use spatial_checker::*;
pub use surface_checker::*;

use crate::{Float, Vec3};

/// Type safe initialization structure for a color. Can be specified in a variety of color spaces.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde-derive", serde(rename_all = "snake_case"))]
pub enum TypedColorInit {
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
            ColorInit::Color(c) => {
                let c = Srgb::new(c[0], c[1], c[2]);
                let c: Xyz = c.into_color_unclamped();
                let c: Xyz<E> = c.adapt_into();
                c
            }
            ColorInit::TypedColor(s) => match s {
                TypedColorInit::LinSrgb(c) => {
                    let c = LinSrgb::new(c.red, c.green, c.blue);
                    let c: Xyz = c.into_color_unclamped();
                    let c: Xyz<E> = c.adapt_into();
                    c
                }
                TypedColorInit::Srgb(c) => {
                    let c = Srgb::new(c.red, c.green, c.blue);
                    let c: Xyz = c.into_color_unclamped();
                    let c: Xyz<E> = c.adapt_into();
                    c
                }
                TypedColorInit::XyzE(c) => c,
                TypedColorInit::XyzD65(c) => c.adapt_into(),
                TypedColorInit::Oklch(c) => {
                    let c = Oklch::new(c.l, c.chroma, c.hue);
                    let c: Xyz = c.into_color_unclamped();
                    let c: Xyz<E> = c.adapt_into();
                    c
                }
            },
        }
    }
}

/// Initialization structure for a texture.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde-derive", serde(tag = "kind"))]
pub enum TextureInit {
    /// SolidColor texture
    SolidColor(SolidColorInit),
    /// SpatialChecker texture
    SpatialChecker(SpatialCheckerInit),
    /// SurfaceChecker texture
    SurfaceChecker(SurfaceCheckerInit),
}

impl Default for TextureInit {
    fn default() -> Self {
        TextureInit::SolidColor(SolidColorInit {
            color: ColorInit::TypedColor(TypedColorInit::XyzE(Xyz::new(0.5, 0.5, 0.5))),
        })
    }
}

impl From<TextureInit> for Texture {
    fn from(value: TextureInit) -> Self {
        match value {
            TextureInit::SolidColor(s) => Texture::SolidColor(SolidColor::new(s.color)),
            TextureInit::SpatialChecker(s) => {
                Texture::SpatialChecker(SpatialChecker::new(s.even, s.odd, s.density))
            }
            TextureInit::SurfaceChecker(s) => {
                Texture::SurfaceChecker(SurfaceChecker::new(s.even, s.odd, s.density))
            }
        }
    }
}

#[enum_dispatch(TextureTrait)]
#[derive(Clone, Debug)]
/// A texture enum.
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde-derive", serde(from = "TextureInit"))]
pub enum Texture {
    /// SolidColor texture
    SolidColor(SolidColor),
    /// SpatialChecker texture
    SpatialChecker(SpatialChecker),
    /// SurfaceChecker texture
    SurfaceChecker(SurfaceChecker),
}

#[enum_dispatch]
pub(crate) trait TextureTrait {
    /// Evaluates the color of the texture at the given surface coordinates or spatial coordinate.
    #[must_use]
    fn color(&self, u: Float, v: Float, position: Vec3) -> Xyz<E>;
}

impl Default for Texture {
    fn default() -> Self {
        SolidColor::default().into()
    }
}
