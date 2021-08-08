//! Utility object for translating i.e. moving another object.

use crate::{
    aabb::AABB,
    hitable::{HitRecord, Hitable},
    ray::Ray,
    Float, Vec3,
};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::Object;

#[derive(Serialize, Deserialize, Debug)]
/// TranslateInit structure describes the necessary data for constructing a [Translate] object. Used with [serde] when importing [SceneFiles](crate::scenes::SceneFile).
pub struct TranslateInit {
    /// The encased [Object] to translate i.e. move
    pub object: Box<Object>,
    /// The vector describing the movement of the object
    pub offset: Vec3,
}

#[derive(Debug)]
/// Translate object. It wraps the given [Object] and has adjusted `hit()` and `bounding_box()` methods based on the `offset` given.
pub struct Translate {
    object: Arc<Hitable>,
    offset: Vec3,
}

impl Translate {
    /// Creates a new `Translate` object. It wraps the given [Object] and has adjusted `hit()` and `bounding_box()` methods based on the `offset` given.
    pub fn new(object: Arc<Hitable>, offset: Vec3) -> Hitable {
        Hitable::Translate(Translate { object, offset })
    }

    /// Hit method for the [Translate] object. Finds the translation-adjusted [HitRecord] for the possible intersection of the [Ray] with the encased [Object].
    pub fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        rng: ThreadRng,
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
        let object_bounding_box = self.object.bounding_box(t0, t1);
        object_bounding_box.map(|aabb| AABB::new(aabb.min + self.offset, aabb.max + self.offset))
    }
}
