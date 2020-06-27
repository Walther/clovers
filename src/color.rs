use crate::{Float, ThreadRng, Vec3};
use image::Rgb;
use rand::prelude::*;
use std::ops::{Add, AddAssign, DivAssign, Mul, MulAssign};

#[derive(Clone, Copy)]
pub struct Color {
    pub r: Float,
    pub g: Float,
    pub b: Float,
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

    pub fn to_rgb_u8(&self) -> Rgb<u8> {
        // Integer-i-fy
        let r = (255.99 * self.r).floor() as u8;
        let g = (255.99 * self.g).floor() as u8;
        let b = (255.99 * self.b).floor() as u8;
        Rgb([r, g, b])
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

impl DivAssign<Float> for Color {
    fn div_assign(&mut self, rhs: Float) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
    }
}
