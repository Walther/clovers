//! Empty object, cannot be hit.

use crate::{aabb::AABB, hitable::HitRecord, ray::Ray, Float, Vec3};
use rand::{rngs::SmallRng, Rng};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// Empty object.
pub struct Empty {}

impl Empty {
    /// New empty object
    pub fn new() -> Self {
        Empty {}
    }

    /// Hit function for Empty object, never hits, always returns None
    pub fn hit(
        &self,
        _ray: &Ray,
        _distance_min: Float,
        _distance_max: Float,
        _rng: &mut SmallRng,
    ) -> Option<HitRecord> {
        None
    }

    /// Bounding box function for empty object, always returns None
    pub fn bounding_box(&self, _t0: Float, _t1: Float) -> Option<AABB> {
        None
    }

    /// PDF value function for Empty object, always returns 0.0
    pub fn pdf_value(
        &self,
        _origin: Vec3,
        _vector: Vec3,
        _time: Float,
        _rng: &mut SmallRng,
    ) -> Float {
        // TODO: is 0.0 a good value?
        0.0
    }

    /// Random function for Empty object. Returns a random normalized vector
    pub fn random(&self, _origin: Vec3, rng: &mut SmallRng) -> Vec3 {
        let random: Vec3 = Vec3::new(rng.gen::<Float>(), rng.gen::<Float>(), rng.gen::<Float>());
        random.normalize()
    }
}

impl Default for Empty {
    fn default() -> Self {
        Self::new()
    }
}
