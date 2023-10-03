//! Utility object for translating i.e. moving another object.

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
/// `TranslateInit` structure describes the necessary data for constructing a [Translate] object. Used with [serde] when importing [`SceneFile`](crate::scenes::SceneFile)s.
pub struct TranslateInit {
    /// The encased [Object] to translate i.e. move
    pub object: Box<Object>,
    /// The vector describing the movement of the object
    pub offset: Vec3,
}

#[derive(Debug, Clone)]
/// Translate object. It wraps the given [Object] and has adjusted `hit()` and `bounding_box()` methods based on the `offset` given.
pub struct Translate<'scene> {
    object: Box<Hitable<'scene>>,
    offset: Vec3,
    aabb: AABB,
}

impl<'scene> Translate<'scene> {
    /// Creates a new `Translate` object. It wraps the given [Object] and has adjusted `hit()` and `bounding_box()` methods based on the `offset` given.
    #[must_use]
    pub fn new(object: Box<Hitable<'scene>>, offset: Vec3) -> Self {
        // TODO: time
        let aabb = object.bounding_box(0.0, 1.0).unwrap().clone() + offset;
        Translate {
            object,
            offset,
            aabb,
        }
    }
}

impl<'scene> HitableTrait for Translate<'scene> {
    /// Hit method for the [Translate] object. Finds the translation-adjusted [`HitRecord`] for the possible intersection of the [Ray] with the encased [Object].
    #[must_use]
    fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        rng: &mut SmallRng,
    ) -> Option<HitRecord> {
        let moved_ray = Ray {
            origin: ray.origin - self.offset,
            direction: ray.direction,
            time: ray.time,
            wavelength: ray.wavelength,
        };

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
    fn bounding_box(&self, _t0: Float, _t1: Float) -> Option<&AABB> {
        Some(&self.aabb)
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
        // TODO: is this correct?
        self.object
            .pdf_value(origin + self.offset, vector, wavelength, time, rng)
    }

    /// Returns a random point on the surface of the moved object
    #[must_use]
    fn random(&self, origin: Vec3, rng: &mut SmallRng) -> Vec3 {
        self.object.random(origin, rng) + self.offset
    }
}
