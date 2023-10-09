//! RGB colorspace utilities.

use crate::{color::Color, Float};

use super::XYZ_Normalized;

/// Linear `sRGB` color based on three [Floats](crate::Float) values.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
#[allow(non_camel_case_types)]
pub struct sRGB_Linear {
    /// The red component of the color, as a [Float]
    pub r: Float,
    /// The green component of the color, as a [Float]
    pub g: Float,
    /// The blue component of the color, as a [Float]
    pub b: Float,
}

/// Conversion from XYZ (D65) to linear `sRGB` values <https://color.org/chardata/rgb/sRGB.pdf>
impl From<XYZ_Normalized> for sRGB_Linear {
    fn from(value: XYZ_Normalized) -> Self {
        let XYZ_Normalized { x, y, z } = value;
        let r = 3.240_625_5 * x - 1.537_208 * y - 0.498_628_6 * z;
        let g = -0.968_930_7 * x + 1.875_756_1 * y + 0.041_517_5 * z;
        let b = 0.055_710_1 * x - 0.204_021_1 * y + 1.056_995_9 * z;

        let r = r.clamp(0.0, 1.0);
        let g = g.clamp(0.0, 1.0);
        let b = b.clamp(0.0, 1.0);

        sRGB_Linear { r, g, b }
    }
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

/// Gamma-corrected `sRGB` color based on three [Floats](crate::Float) values.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
#[allow(non_camel_case_types)]
pub struct sRGB {
    /// The red component of the color, as a [Float]
    pub r: Float,
    /// The green component of the color, as a [Float]
    pub g: Float,
    /// The blue component of the color, as a [Float]
    pub b: Float,
}

impl sRGB {
    /// Transforms the [`sRGB`] into a 24-bit, 3 x `u8` representation.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    pub fn to_bytes(&self) -> [u8; 3] {
        [self.r, self.g, self.b].map(|mut component| {
            if component.is_nan() {
                component = 0.0;
            }
            component = component.clamp(0.0, 1.0);
            // TODO: why 255.99?
            (255.99 * component).floor() as u8
        })
    }
}

impl From<sRGB_Linear> for sRGB {
    fn from(value: sRGB_Linear) -> Self {
        let sRGB_Linear { r, g, b } = value;
        sRGB {
            r: color_component_transfer(r),
            g: color_component_transfer(g),
            b: color_component_transfer(b),
        }
    }
}

impl From<Color> for sRGB_Linear {
    fn from(value: Color) -> Self {
        // TODO: verify correctness / possibly remove the simplistic `Color` type
        let Color { r, g, b } = value;
        sRGB_Linear { r, g, b }
    }
}

#[cfg(test)]
mod tests {
    use crate::colors::{sRGB, XYZ_Normalized, XYZ_Tristimulus};

    use super::sRGB_Linear;

    #[test]
    fn xyz_black_to_srgb() {
        let original: XYZ_Tristimulus = XYZ_Tristimulus {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let converted: XYZ_Normalized = original.into();
        let converted: sRGB_Linear = converted.into();
        let expected = sRGB_Linear {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        };
        assert_eq!(converted, expected);
    }

    #[test]
    fn xyz_white_to_srgb() {
        // D65 standard illuminant white point
        let original: XYZ_Tristimulus = XYZ_Tristimulus {
            x: 0.9504,
            y: 1.0000,
            z: 1.0888,
        };
        let converted: XYZ_Normalized = original.into();
        let converted: sRGB_Linear = converted.into();
        let converted: sRGB = converted.into();
        // NOTE: if using floats in the expected, precision errors ensue. Womp womp.
        // let expected = sRGB {
        //     r: 1.0,
        //     g: 1.0,
        //     b: 1.0,
        // };
        let converted: [u8; 3] = converted.to_bytes();
        let expected = [255, 255, 255];
        assert_eq!(converted, expected);
    }
}
