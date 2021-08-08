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
pub struct FlipFaceInit {
    pub object: Box<Object>,
}

#[derive(Debug)]
pub struct FlipFace {
    object: Arc<Hitable>,
}

impl FlipFace {
    pub fn new(object: Hitable) -> Hitable {
        Hitable::FlipFace(FlipFace {
            object: Arc::new(object),
        })
    }

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

    pub fn bounding_box(&self, t0: Float, t1: Float) -> Option<AABB> {
        self.object.bounding_box(t0, t1)
    }
}
