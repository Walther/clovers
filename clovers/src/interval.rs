//! Interval helper adapted from the book

use core::ops::Add;

use crate::Float;

/// An interval structure.
#[derive(Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
pub struct Interval {
    /// Smallest value of the interval. Must be kept in order  
    pub min: Float,
    /// Largest value of the interval. Must be kept in order
    pub max: Float,
}

impl Interval {
    /// Constructs a new interval
    #[must_use]
    pub fn new(a: Float, b: Float) -> Self {
        Interval {
            min: a.min(b),
            max: a.max(b),
        }
    }

    /// Constructs a new interval from two intervals
    // TODO: explanation, clarification
    #[must_use]
    pub fn combine(a: &Interval, b: &Interval) -> Self {
        Interval {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }

    /// Returns an interval expanded with delta at both ends
    #[must_use]
    pub fn expand(&self, delta: Float) -> Self {
        Interval::new(self.min - delta, self.max + delta)
    }

    /// Returns the size of the interval
    #[must_use]
    pub fn size(&self) -> Float {
        self.max - self.min
    }

    /// Returns the center of this [`Interval`]
    #[must_use]
    pub fn center(&self) -> Float {
        self.min + 0.5 * self.size()
    }
}

impl Add<Float> for Interval {
    type Output = Interval;

    fn add(self, offset: Float) -> Self::Output {
        Interval::new(self.min + offset, self.max + offset)
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    #[test]
    fn center() {
        let interval = Interval::new(0.0, 1.0);
        let center = interval.center();
        let expected = 0.5;
        assert_eq!(center, expected);
    }

    #[test]
    fn center_zero_crossing() {
        let interval = Interval::new(-1.0, 1.0);
        let center = interval.center();
        let expected = 0.0;
        assert_eq!(center, expected);
    }

    #[test]
    fn size() {
        let interval = Interval::new(0.0, 1.0);
        let size = interval.size();
        let expected = 1.0;
        assert_eq!(size, expected);
    }

    #[test]
    fn size_zero_crossing() {
        let interval = Interval::new(-1.0, 1.0);
        let size = interval.size();
        let expected = 2.0;
        assert_eq!(size, expected);
    }
}
