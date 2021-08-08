use crate::{
    aabb::AABB,
    hitable::{HitRecord, Hitable},
    ray::Ray,
    Float,
};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::Object;

#[derive(Serialize, Deserialize, Debug)]
/// FlipFaceInit structure describes the necessary data for constructing a [FlipFace]. Used with [serde] when importing [SceneFiles](crate::scenes::SceneFile).
pub struct FlipFaceInit {
    pub object: Box<Object>,
}

#[derive(Debug)]
/// An utility object that can be used to flip the face of the object. TODO: possibly deprecated?
pub struct FlipFace {
    object: Arc<Hitable>,
}

impl FlipFace {
    // TODO: possibly deprecate / remove?

    /// Creates a new instance of a [FlipFace]
    pub fn new(object: Hitable) -> Hitable {
        Hitable::FlipFace(FlipFace {
            object: Arc::new(object),
        })
    }

    /// Hit function for the [FlipFace] object. Considering this is a utility object that wraps an internal `object`, it returns a [HitRecord] with the `front_face` property flipped, if the given [Ray] hits the object.
    pub fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        rng: ThreadRng,
    ) -> Option<HitRecord> {
        self.object
            .hit(ray, distance_min, distance_max, rng)
            .map(|hit_record| HitRecord {
                front_face: !hit_record.front_face,
                ..hit_record
            })
    }

    /// Returns the axis-aligned bounding box [AABB] of the [FlipFace] object. Considering this is a utility object that wraps an internal `object`, it returns the bounding box of the internal object.
    pub fn bounding_box(&self, t0: Float, t1: Float) -> Option<AABB> {
        self.object.bounding_box(t0, t1)
    }
}
