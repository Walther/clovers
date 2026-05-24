//! Stokes vector for characterizing polarized light

use crate::Float;

/// Stokes vector for characterizing polarized light.
///
/// Based on Chapter 4.1 of "How to write a polarisation ray tracer"[^1]
///
/// [^1]: Wilkie, A., & Weidlich, A. (2011). How to write a polarisation ray tracer. SIGGRAPH Asia 2011 Courses, 1–36. <https://doi.org/10.1145/2077434.2077442>
#[derive(Default, Debug)]
pub struct Stokes {
    /// Total (unpolarized) radiant intensity
    s0: Float,
    /// Linear polarization at 0 degrees
    s1: Float,
    /// Linear polarization at 45 degrees
    s2: Float,
    /// Right-circular polarization
    s3: Float,
}

impl Stokes {
    /// Creates a new Stokes vector with the given four components
    #[must_use]
    pub fn new(s0: Float, s1: Float, s2: Float, s3: Float) -> Self {
        Self { s0, s1, s2, s3 }
    }

    /// Validates the constraint that the total radiant intensity is greater than or equal to the sum of the polarized components.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.s0 >= (self.s1.powi(2) + self.s2.powi(2) + self.s3.powi(2)).sqrt()
    }
}

impl From<[Float; 4]> for Stokes {
    fn from(components: [Float; 4]) -> Self {
        Self {
            s0: components[0],
            s1: components[1],
            s2: components[2],
            s3: components[3],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let _ = Stokes::new(1.0, 0.0, 0.0, 0.0);
    }

    #[test]
    fn from_array() {
        let _ = Stokes::from([1.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn unpolarized() {
        let s = Stokes::from([1.0, 0.0, 0.0, 0.0]);
        assert!(s.is_valid());
    }

    #[test]
    fn polarized() {
        let s = Stokes::from([Float::sqrt(3.0), 1.0, 1.0, 1.0]);
        assert!(s.is_valid());
    }

    #[test]
    fn invalid() {
        let s = Stokes::from([1.0, 1.0, 1.0, 1.0]);
        assert!(!s.is_valid());
    }
}
