//! The very core of the ray tracing rendering itself: the [Ray](crate::ray::Ray)

use crate::{Float, Vec3};

/// A Ray has an origin and a direction, as well as an instant in time it exists in. Motion blur is achieved by creating multiple rays with slightly different times.
#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub time: Float,
}

impl Ray {
    /// Creates a single Ray. A Ray has an origin and a direction, as well as an instant in time it exists in. Motion blur is achieved by creating multiple rays with slightly different times.
    pub fn new(origin: Vec3, direction: Vec3, time: Float) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn point_at_parameter(&self, t: Float) -> Vec3 {
        self.origin + t * self.direction
    }
}
