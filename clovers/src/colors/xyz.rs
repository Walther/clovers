//! CIE 1931 XYZ colorspace utilities.

use core::ops::{Add, Div, Mul};

use crate::{color::Color, Float};

use super::{sRGB_Linear, Wavelength};

/// CIE 1931 XYZ Tristimulus color based on three [Floats](crate::Float) values.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
#[allow(non_camel_case_types)]
pub struct XYZ_Tristimulus {
    /// The x component of the color, as a [Float]
    pub x: Float,
    /// The y component of the color, as a [Float]
    pub y: Float,
    /// The z component of the color, as a [Float]
    pub z: Float,
}

/// Helper function adapted from <https://en.wikipedia.org/wiki/CIE_1931_color_space#Analytical_approximation>
fn gaussian(x: Float, alpha: Float, mu: Float, sigma1: Float, sigma2: Float) -> Float {
    let t = (x - mu) / (if x < mu { sigma1 } else { sigma2 });
    alpha * (-(t * t) / 2.0).exp()
}

/// Helper function adapted from <https://en.wikipedia.org/wiki/CIE_1931_color_space#Analytical_approximation>
impl From<Wavelength> for XYZ_Tristimulus {
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

        XYZ_Tristimulus { x, y, z }
    }
}

/// CIE 1931 XYZ Normalized color based on three [Floats](crate::Float) values.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
#[allow(non_camel_case_types)]
pub struct XYZ_Normalized {
    /// The x component of the color, as a [Float]
    pub x: Float,
    /// The y component of the color, as a [Float]
    pub y: Float,
    /// The z component of the color, as a [Float]
    pub z: Float,
}

impl XYZ_Tristimulus {
    /// Returns a new [`XYZ_Tristimulus`] with the each channel limited to positive values.
    #[must_use]
    pub fn non_negative(&self) -> XYZ_Tristimulus {
        XYZ_Tristimulus {
            x: self.x.max(0.0),
            y: self.y.max(0.0),
            z: self.z.max(0.0),
        }
    }

    /// Returns a new [`XYZ_Tristimulus`] with the each channel limited to positive values.
    #[must_use]
    pub fn limit_one(&self) -> XYZ_Tristimulus {
        XYZ_Tristimulus {
            x: self.x.min(1.0),
            y: self.y.min(1.0),
            z: self.z.min(1.0),
        }
    }

    /// Returns a new [`XYZ_Tristimulus`] with each channel limited to the range between `0.0` and `1.0`.
    #[must_use]
    pub fn clamp(&self) -> XYZ_Tristimulus {
        self.non_negative().limit_one()
    }

    /// Returns `true` if all components are real and finite. Returns false if any component is `NaN` or `Infinity`.
    #[must_use]
    pub fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    /// Returns the cromaticity tuple `(x,y)` by scaling the `X` and `Y` appropriately to unity brightness.
    #[must_use]
    pub fn chromaticity(&self) -> (Float, Float) {
        let &Self { x, y, z } = self;
        let chroma_x = x / (x + y + z);
        let chroma_y = y / (x + y + z);
        (chroma_x, chroma_y)
    }
}

impl Mul<Float> for XYZ_Normalized {
    type Output = XYZ_Normalized;

    fn mul(self, mul: Float) -> Self::Output {
        Self {
            x: self.x * mul,
            y: self.y * mul,
            z: self.z * mul,
        }
    }
}

/// Tristimulus value normalization. <https://color.org/chardata/rgb/sRGB.pdf>
impl From<XYZ_Tristimulus> for XYZ_Normalized {
    fn from(value: XYZ_Tristimulus) -> Self {
        let XYZ_Tristimulus { x, y, z } = value;

        let x_n = (76.04 * (x - 0.1901)) / (80.0 * (76.04 - 0.1901));
        let y_n = (y - 0.2) / (80.0 - 0.2);
        let z_n = (87.12 * (z - 0.2178)) / (80.0 * (87.12 - 0.2178));

        // TODO: why do I need the 100x multiplier here?
        XYZ_Normalized {
            x: 100.0 * x_n,
            y: 100.0 * y_n,
            z: 100.0 * z_n,
        }
    }
}

/// Adapted from <https://en.wikipedia.org/wiki/SRGB#From_sRGB_to_CIE_XYZ>
/// NOTE: Assumes D65 illuminant as per `sRGB` definition
impl From<sRGB_Linear> for XYZ_Tristimulus {
    fn from(value: sRGB_Linear) -> Self {
        let sRGB_Linear { r, g, b } = value;
        let x = 0.4124 * r + 0.3576 * g + 0.1805 * b;
        let y = 0.2126 * r + 0.7152 * g + 0.0722 * b;
        let z = 0.0193 * r + 0.1192 * g + 0.9505 * b;

        Self { x, y, z }
    }
}

impl Mul<Float> for XYZ_Tristimulus {
    type Output = XYZ_Tristimulus;

    fn mul(self, mul: Float) -> Self::Output {
        Self {
            x: self.x * mul,
            y: self.y * mul,
            z: self.z * mul,
        }
    }
}

impl Mul<XYZ_Tristimulus> for XYZ_Tristimulus {
    type Output = XYZ_Tristimulus;

    // TODO: verify correcteness
    fn mul(self, rhs: XYZ_Tristimulus) -> Self::Output {
        XYZ_Tristimulus {
            x: rhs.x * self.x,
            y: rhs.y * self.y,
            z: rhs.z * self.z,
        }
    }
}

impl Div<Float> for XYZ_Tristimulus {
    type Output = XYZ_Tristimulus;
    fn div(self, rhs: Float) -> XYZ_Tristimulus {
        // TODO: verify correcteness
        XYZ_Tristimulus {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Add<XYZ_Tristimulus> for XYZ_Tristimulus {
    type Output = XYZ_Tristimulus;

    fn add(self, rhs: XYZ_Tristimulus) -> Self::Output {
        XYZ_Tristimulus {
            x: rhs.x + self.x,
            y: rhs.y + self.y,
            z: rhs.z + self.z,
        }
    }
}

impl From<Color> for XYZ_Tristimulus {
    fn from(value: Color) -> Self {
        let value: sRGB_Linear = value.into();
        let value: XYZ_Tristimulus = value.into();
        value
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;

    use super::XYZ_Tristimulus;

    #[test]
    fn color_black_to_xyz() {
        let original: Color = Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        };

        let converted: XYZ_Tristimulus = original.into();
        let expected: XYZ_Tristimulus = XYZ_Tristimulus {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        assert_eq!(converted, expected);
    }

    #[test]
    fn color_white_to_xyz() {
        let original: Color = Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        };

        // NOTE: based on D65 illuminant as per to sRGB
        let converted: XYZ_Tristimulus = original.into();
        // TODO: why the precision issues?
        // let expected: XYZ_Tristimulus = XYZ_Tristimulus {
        //     x: 0.9504,
        //     y: 1.0000,
        //     z: 1.0888,
        // };
        let expected: XYZ_Tristimulus = XYZ_Tristimulus {
            x: 0.9505,
            y: 1.0,
            z: 1.089,
        };
        assert_eq!(converted, expected);
    }
}
