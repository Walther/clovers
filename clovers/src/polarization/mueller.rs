//! Mueller matrix for attenuation of polarized light

use nalgebra::{Matrix4, RowVector4};

use crate::Float;

/// Mueller matrix for attenuation of polarized light.
///
/// Based on Chapter 4.3 of "How to write a polarisation ray tracer"[^1]
///
/// [^1]: Wilkie, A., & Weidlich, A. (2011). How to write a polarisation ray tracer. SIGGRAPH Asia 2011 Courses, 1–36. <https://doi.org/10.1145/2077434.2077442>
#[derive(Debug)]
pub struct Mueller {
    #[expect(unused)]
    matrix: Matrix4<Float>,
}

impl Mueller {
    /// Creates a new Mueller matrix with plain, uniform attenuation affecting polarized and unpolarized components in an equal fashion
    /// ```
    /// a 0 0 0
    /// 0 a 0 0
    /// 0 0 a 0
    /// 0 0 0 a
    /// ```
    #[must_use]
    pub fn plain(attenuation: Float) -> Self {
        Self {
            matrix: Matrix4::from_diagonal_element(attenuation),
        }
    }

    /// Creates a new Mueller matrix with configurable attenuation on the total unpolarized radiant intensity and attenuation to zero on the polarized components
    ///
    /// ```
    /// a 0 0 0
    /// 0 0 0 0
    /// 0 0 0 0
    /// 0 0 0 0
    /// ```
    #[must_use]
    pub fn depolarizer(attenuation: Float) -> Self {
        Self {
            matrix: Matrix4::from_partial_diagonal(&[attenuation]),
        }
    }

    /// Creates a new Mueller matrix for an ideal linear polarization filter with zero attenuation, at a given angle against the axis of the incoming ray
    #[must_use]
    pub fn linear(angle: Float) -> Self {
        // Helper: two times theta
        let t = 2.0 * angle;
        Self {
            matrix: 0.5
                * Matrix4::from_rows(&[
                    RowVector4::new(1.0, t.cos(), t.sin(), 0.0),
                    RowVector4::new(t.cos(), t.cos().powi(2), t.sin() * t.cos(), 0.0),
                    RowVector4::new(t.sin(), t.sin() * t.cos(), t.sin().powi(2), 0.0),
                    RowVector4::new(0.0, 0.0, 0.0, 0.0),
                ]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain() {
        let _ = Mueller::plain(1.0);
    }

    #[test]
    fn depolarizer() {
        let _ = Mueller::depolarizer(1.0);
    }

    #[test]
    fn linear() {
        let _ = Mueller::linear(1.0);
    }
}
