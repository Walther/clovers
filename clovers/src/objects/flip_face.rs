//! An utility object that can be used to flip the face of the object. TODO: possibly deprecated?

use crate::{
    aabb::AABB,
    colors::Wavelength,
    hitable::{HitRecord, Hitable, HitableTrait},
    ray::Ray,
    Box, Float, Vec3,
};
use rand::rngs::SmallRng;

use super::Object;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// `FlipFaceInit` structure describes the necessary data for constructing a [`FlipFace`]. Used with [serde] when importing [`SceneFile`](crate::scenes::SceneFile)s.
pub struct FlipFaceInit {
    /// The object to wrap with the face flipping feature.
    pub object: Box<Object>,
}

#[derive(Debug, Clone)]
/// An utility object that can be used to flip the face of the object. TODO: possibly deprecated?
pub struct FlipFace<'scene> {
    object: Box<Hitable<'scene>>,
}

impl<'scene> FlipFace<'scene> {
    // TODO: possibly deprecate / remove?

    /// Creates a new instance of a [`FlipFace`]
    #[must_use]
    pub fn new(object: Hitable<'scene>) -> Self {
        FlipFace {
            object: Box::new(object),
        }
    }
}

impl<'scene> HitableTrait for FlipFace<'scene> {
    /// Hit function for the [`FlipFace`] object. Considering this is a utility object that wraps an internal `object`, it returns a [`HitRecord`] with the `front_face` property flipped, if the given [Ray] hits the object.
    #[must_use]
    fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        rng: &mut SmallRng,
    ) -> Option<HitRecord> {
        self.object
            .hit(ray, distance_min, distance_max, rng)
            .map(|hit_record| HitRecord {
                front_face: !hit_record.front_face,
                ..hit_record
            })
    }

    /// Returns the axis-aligned bounding box [AABB] of the [`FlipFace`] object. Considering this is a utility object that wraps an internal `object`, it returns the bounding box of the internal object.
    #[must_use]
    fn bounding_box(&self, t0: Float, t1: Float) -> Option<&AABB> {
        self.object.bounding_box(t0, t1)
    }

    /// Returns a probability density function value based on the inner object
    #[must_use]
    fn pdf_value(
        &self,
        origin: Vec3,
        vector: Vec3,
        wavelength: Wavelength,
        time: Float,
        rng: &mut SmallRng,
    ) -> Float {
        self.object.pdf_value(origin, vector, wavelength, time, rng)
    }

    /// Returns a random point on the surface of the inner object
    #[must_use]
    fn random(&self, origin: Vec3, rng: &mut SmallRng) -> Vec3 {
        self.object.random(origin, rng)
    }
}
