//! Orthonormal bases

use crate::Vec3;

#[derive(Debug)]
/// An orthonormal basis structure.
pub struct ONB {
    /// U
    pub u: Vec3,
    /// V
    pub v: Vec3,
    /// W
    pub w: Vec3,
}

// TODO: understand, explain

impl ONB {
    /// Builds a new [ONB] structure given a normal vector.
    pub fn build_from_w(normal: Vec3) -> ONB {
        let w = (normal).normalize();
        let a: Vec3 = if (w.x).abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let v = (w.cross(&a)).normalize();
        let u = w.cross(&v);

        ONB { u, v, w }
    }

    /// Returns the ONB-projected version of the provided vector?
    pub fn local(&self, vec: Vec3) -> Vec3 {
        vec.x * self.u + vec.y * self.v + vec.z * self.w
    }
}
