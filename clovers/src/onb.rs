//! Orthonormal bases

use nalgebra::Unit;

use crate::{Direction, Vec3};

#[derive(Debug, Clone)]
/// An orthonormal basis structure.
pub struct ONB {
    /// U
    pub u: Direction,
    /// V
    pub v: Direction,
    /// W
    pub w: Direction,
}

// TODO: improve correctness & optimization!

impl ONB {
    /// Builds a new [ONB] structure given a normal vector.
    #[must_use]
    pub fn build_from_w(w: Direction) -> ONB {
        let a: Direction = if (w.x).abs() > 0.9 {
            Unit::new_normalize(Vec3::new(0.0, 1.0, 0.0))
        } else {
            Unit::new_normalize(Vec3::new(1.0, 0.0, 0.0))
        };
        let v = Unit::new_normalize(w.cross(&a));
        let u = Unit::new_normalize(w.cross(&v));

        ONB { u, v, w }
    }

    /// Returns the ONB-projected version of the provided vector?
    #[must_use]
    pub fn local(&self, vec: Direction) -> Direction {
        let d = vec.x * *self.u + vec.y * *self.v + vec.z * *self.w;
        Unit::new_normalize(d)
    }
}
