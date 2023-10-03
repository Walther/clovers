//! RGB colorspace utilities.

use crate::Float;

use super::XYZ_Normalized;

/// Linear `sRGB` color based on three [Floats](crate::Float) values.
#[derive(Copy, Clone, Debug)]
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
#[derive(Copy, Clone, Debug)]
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
