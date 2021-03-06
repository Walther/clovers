//! Color utilities.

use crate::{Float, Vec3};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign};

/// RGB color based on three [Floats](crate::Float)
#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct Color {
    pub r: Float,
    pub g: Float,
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
    pub fn new(r: Float, g: Float, b: Float) -> Color {
        Color { r, g, b }
    }

    pub fn random(mut rng: ThreadRng) -> Color {
        Color {
            r: rng.gen::<Float>(),
            g: rng.gen::<Float>(),
            b: rng.gen::<Float>(),
        }
    }

    pub fn component_mul(&self, other: &Color) -> Color {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }

    // TODO: why did this misbehave when attempted as a mutable self?
    pub fn gamma_correction(&self, gamma: Float) -> Color {
        // Raise to the power of inverse of gamma number given
        let power: Float = 1.0 / gamma;
        Color {
            r: self.r.powf(power),
            g: self.g.powf(power),
            b: self.b.powf(power),
        }
    }

    pub fn to_rgb_u8(&self) -> [u8; 3] {
        // TODO: might be possible to optimize
        let mut r = self.r;
        let mut g = self.g;
        let mut b = self.b;
        // Fix NaN to zero & remove negatives
        if r.is_nan() || r < 0.0 {
            r = 0.0;
        };
        if g.is_nan() || g < 0.0 {
            g = 0.0;
        };
        if b.is_nan() || b < 0.0 {
            b = 0.0;
        };
        // Fix too large numbers
        r = r.min(1.0);
        g = g.min(1.0);
        b = b.min(1.0);
        // Integer-i-fy
        let r = (255.99 * r).floor() as u8;
        let g = (255.99 * g).floor() as u8;
        let b = (255.99 * b).floor() as u8;
        [r, g, b]
    }
}

impl From<[u8; 3]> for Color {
    fn from(rgb: [u8; 3]) -> Self {
        Color::new(
            (rgb[0] as Float) / 255.99,
            (rgb[1] as Float) / 255.99,
            (rgb[2] as Float) / 255.99,
        )
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
