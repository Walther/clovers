//! Utility object for translating i.e. moving another object.

use crate::CloversRng;
use crate::{aabb::AABB, hitable::Hitable, hitrecord::HitRecord, ray::Ray, Box, Float, Vec3};

use super::Object;

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// TranslateInit structure describes the necessary data for constructing a [Translate] object. Used with [serde] when importing [SceneFiles](crate::scenes::SceneFile).
pub struct TranslateInit {
    /// The encased [Object] to translate i.e. move
    pub object: Box<Object>,
    /// The vector describing the movement of the object
    pub offset: Vec3,
}

#[derive(Clone)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
/// Translate object. It wraps the given [Object] and has adjusted `hit()` and `bounding_box()` methods based on the `offset` given.
pub struct Translate {
    object: Box<Hitable>,
    offset: Vec3,
}

impl Translate {
    /// Creates a new `Translate` object. It wraps the given [Object] and has adjusted `hit()` and `bounding_box()` methods based on the `offset` given.
    pub fn new(object: Box<Hitable>, offset: Vec3) -> Self {
        Translate { object, offset }
    }

    /// Hit method for the [Translate] object. Finds the translation-adjusted [HitRecord] for the possible intersection of the [Ray] with the encased [Object].
    pub fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        rng: &mut CloversRng,
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
    pub fn bounding_box(&self, t0: Float, t1: Float) -> Option<AABB> {
        // TODO: cached into self.aabb ?
        let aabb = self.object.bounding_box(t0, t1);
        aabb.map(|b| b + self.offset)
    }
}
