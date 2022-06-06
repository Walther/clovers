//! Utility object for translating i.e. moving another object.

use crate::{
    aabb::AABB,
    hitable::{HitRecord, Hitable},
    ray::Ray,
    Box, Float, Vec3,
};
use rand::rngs::SmallRng;

use super::Object;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// `TranslateInit` structure describes the necessary data for constructing a [Translate] object. Used with [serde] when importing [`SceneFile`](crate::scenes::SceneFile)s.
pub struct TranslateInit {
    /// The encased [Object] to translate i.e. move
    pub object: Box<Object>,
    /// The vector describing the movement of the object
    pub offset: Vec3,
}

#[derive(Debug, Clone)]
/// Translate object. It wraps the given [Object] and has adjusted `hit()` and `bounding_box()` methods based on the `offset` given.
pub struct Translate {
    object: Box<Hitable>,
    offset: Vec3,
}

impl Translate {
    /// Creates a new `Translate` object. It wraps the given [Object] and has adjusted `hit()` and `bounding_box()` methods based on the `offset` given.
    #[must_use]
    pub fn new(object: Box<Hitable>, offset: Vec3) -> Self {
        Translate { object, offset }
    }

    /// Hit method for the [Translate] object. Finds the translation-adjusted [`HitRecord`] for the possible intersection of the [Ray] with the encased [Object].
    pub fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        rng: &mut SmallRng,
    ) -> Option<HitRecord> {
        let moved_ray: Ray = Ray::new(ray.origin - self.offset, ray.direction, ray.time);

        match self.object.hit(&moved_ray, distance_min, distance_max, rng) {
            // Didn't hit anything, return None
            None => None,
            // Hit something, adjust the position and normal
            Some(mut hit_record) => {
                hit_record.position += self.offset;
                hit_record.set_face_normal(&moved_ray, hit_record.normal);
                Some(hit_record)
            }
        }
    }

    /// Bounding box method for the [Translate] object. Finds the axis-aligned bounding box [AABB] for the encased [Object] after adjusting for translation.
    #[must_use]
    pub fn bounding_box(&self, t0: Float, t1: Float) -> Option<AABB> {
        // TODO: cached into self.aabb ?
        let aabb = self.object.bounding_box(t0, t1);
        aabb.map(|b| b + self.offset)
    }

    /// Returns a probability density function value based on the inner object
    pub fn pdf_value(&self, origin: Vec3, vector: Vec3, time: Float, rng: &mut SmallRng) -> Float {
        // TODO: is this correct?
        self.object
            .pdf_value(origin + self.offset, vector, time, rng)
    }

    /// Returns a random point on the surface of the moved object
    pub fn random(&self, origin: Vec3, rng: &mut SmallRng) -> Vec3 {
        self.object.random(origin, rng) + self.offset
    }
}
