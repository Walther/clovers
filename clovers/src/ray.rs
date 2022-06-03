//! The very core of the ray tracing rendering itself: the [Ray](crate::ray::Ray)

use crate::{Float, Vec3};

/// A Ray has an origin and a direction, as well as an instant in time it exists in. Motion blur is achieved by creating multiple rays with slightly different times.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray {
    /// The origin of the ray.
    pub origin: Vec3,
    /// The direction of the ray.
    pub direction: Vec3,
    /// The time instant at which the ray exists.
    pub time: Float,
}

impl Ray {
    /// Creates a single Ray. A Ray has an origin and a direction, as well as an instant in time it exists in. Motion blur is achieved by creating multiple rays with slightly different times.
    #[must_use]
    pub fn new(origin: Vec3, direction: Vec3, time: Float) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }

    /// Evaluates the position (coordinate) at which the ray is at the given parameter, considering the origin and direction. Considering a default unit speed of 1 per unit time, this function can be given either a time or a distance.
    #[must_use]
    pub fn evaluate(&self, parameter: Float) -> Vec3 {
        self.origin + parameter * self.direction
    }
}
