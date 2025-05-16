//! The very core of the ray tracing rendering itself: the [Ray]

use crate::{Direction, Float, Position, wavelength::Wavelength};

/// A Ray has an origin and a direction, as well as an instant in time it exists in. Motion blur is achieved by creating multiple rays with slightly different times.
#[derive(Clone, Debug, PartialEq)]
pub struct Ray {
    /// The origin of the ray.
    pub origin: Position,
    /// The direction of the ray.
    pub direction: Direction,
    /// The time instant at which the ray exists.
    pub time: Float,
    /// Wavelength of the ray
    pub wavelength: Wavelength,
}

impl Ray {
    /// Evaluates the position (coordinate) at which the ray is at the given parameter, considering the origin and direction. Considering a default unit speed of 1 per unit time, this function can be given either a time or a distance.
    #[must_use]
    pub fn evaluate(&self, parameter: Float) -> Position {
        self.origin + parameter * *self.direction
    }
}
