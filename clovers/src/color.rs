//! Color utilities.

// TODO: more flexible colors?

use crate::colors::sRGB;
use crate::{Float, Vec3};
use core::iter::Sum;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign};
use rand::rngs::SmallRng;
use rand::Rng;

/// RGB color based on three [Floats](crate::Float) values.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
pub struct Color {
    /// The red component of the color, as a [Float]
    pub r: Float,
    /// The green component of the color, as a [Float]
    pub g: Float,
    /// The blue component of the color, as a [Float]
    pub b: Float,
}

impl Default for Color {
    /// Default color: middle gray 18%
    fn default() -> Self {
        Color {
            r: 0.18,
            g: 0.18,
            b: 0.18,
        }
    }
}

impl Color {
    /// Creates a new [Color] with the given parameters.
    #[must_use]
    pub fn new(r: Float, g: Float, b: Float) -> Color {
        Color { r, g, b }
    }

    /// Returns a new [Color] with the each channel limited to positive values.
    #[must_use]
    pub fn non_negative(&self) -> Color {
        Color {
            r: self.r.max(0.0),
            g: self.g.max(0.0),
            b: self.b.max(0.0),
        }
    }

    /// Returns a new [Color] with the each channel limited to positive values.
    #[must_use]
    pub fn limit_one(&self) -> Color {
        Color {
            r: self.r.min(1.0),
            g: self.g.min(1.0),
            b: self.b.min(1.0),
        }
    }

    /// Returns a new [Color] with each channel limited to the range between `0.0` and `1.0`.
    #[must_use]
    pub fn clamp(&self) -> Color {
        self.non_negative().limit_one()
    }

    /// Creates a new [Color] with random parameters between `0.0..1.0`.
    #[must_use]
    pub fn random(mut rng: SmallRng) -> Color {
        Color {
            r: rng.gen::<Float>(),
            g: rng.gen::<Float>(),
            b: rng.gen::<Float>(),
        }
    }

    /// Component-wise multiplication of two [Colors](Color).
    #[must_use]
    pub fn component_mul(&self, other: &Color) -> Color {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }

    // TODO: why did this misbehave when attempted as a mutable self?
    /// Returns the gamma-corrected [Color].
    #[must_use]
    pub fn gamma_correction(&self, gamma: Float) -> Color {
        // Raise to the power of inverse of gamma number given
        let power: Float = 1.0 / gamma;
        Color {
            r: self.r.powf(power),
            g: self.g.powf(power),
            b: self.b.powf(power),
        }
    }

    /// Transforms the [Float] based [Color] into a 24-bit, 3 x u8 RGB color.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    pub fn to_rgb_u8(&self) -> [u8; 3] {
        [self.r, self.g, self.b].map(|mut component| {
            if component.is_nan() {
                component = 0.0;
            }
            component = component.clamp(0.0, 1.0);
            (255.99 * component).floor() as u8
        })
    }
}

impl From<[u8; 3]> for Color {
    fn from(rgb: [u8; 3]) -> Self {
        #[allow(clippy::cast_lossless)]
        Color::new(
            (rgb[0] as Float) / 255.99,
            (rgb[1] as Float) / 255.99,
            (rgb[2] as Float) / 255.99,
        )
    }
}

impl From<[Float; 3]> for Color {
    fn from(rgb: [Float; 3]) -> Self {
        Color::new(rgb[0], rgb[1], rgb[2])
    }
}

impl From<[Float; 4]> for Color {
    // TODO: ignores alpha
    fn from(rgb: [Float; 4]) -> Self {
        Color::new(rgb[0], rgb[1], rgb[2])
    }
}

impl From<Color> for Vec3 {
    fn from(color: Color) -> Vec3 {
        Vec3::new(color.r, color.g, color.b)
    }
}

impl Add<Color> for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}

impl AddAssign<Color> for Color {
    fn add_assign(&mut self, other: Color) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

impl Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
            },
            |a, b| a + b,
        )
    }
}

impl Mul<Float> for Color {
    type Output = Color;
    fn mul(self, rhs: Float) -> Self::Output {
        Color::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl MulAssign<Float> for Color {
    fn mul_assign(&mut self, rhs: Float) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
    }
}

// Really? Do I really need to implement this manually both ways?
impl Mul<Color> for Float {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(rhs.r * self, rhs.g * self, rhs.b * self)
    }
}

impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(rhs.r * self.r, rhs.g * self.g, rhs.b * self.b)
    }
}

impl DivAssign<Float> for Color {
    fn div_assign(&mut self, rhs: Float) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
    }
}

impl Div<Float> for Color {
    type Output = Color;
    fn div(self, rhs: Float) -> Color {
        Color::new(self.r / rhs, self.g / rhs, self.b / rhs)
    }
}

impl From<sRGB> for Color {
    fn from(value: sRGB) -> Self {
        // TODO: verify correctness / possibly remove the simplistic `Color` type
        let sRGB { r, g, b } = value;
        Color { r, g, b }
    }
}
