//! CIE 1931 XYZ colorspace utilities.

use crate::Float;

use super::Wavelength;

/// CIE 1931 XYZ Tristimulus color based on three [Floats](crate::Float) values.
#[derive(Copy, Clone, Debug)]
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

/// Tristimulus value normalization. <https://color.org/chardata/rgb/sRGB.pdf>
impl From<XYZ_Tristimulus> for XYZ_Normalized {
    fn from(value: XYZ_Tristimulus) -> Self {
        let XYZ_Tristimulus { x, y, z } = value;
        // TODO: why does this normalization make the image bad? It should be needed?

        // let x_n = (76.04 * (x - 0.1901)) / (80.0 * (76.04 - 0.1901));
        // let y_n = (y - 0.2) / (80.0 - 0.2);
        // let z_n = (87.12 * (z - 0.2178)) / (80.0 * (87.12 - 0.2178));

        // FIXME: normalization not done, but image looks more correct?
        let x_n = x;
        let y_n = y;
        let z_n = z;

        XYZ_Normalized {
            x: x_n,
            y: y_n,
            z: z_n,
        }
    }
}
