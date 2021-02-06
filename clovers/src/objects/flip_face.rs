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
        match self.object.hit(ray, distance_min, distance_max, rng) {
            Some(hit_record) => Some(HitRecord {
                distance: hit_record.distance,
                position: hit_record.position,
                normal: hit_record.normal,
                u: hit_record.u,
                v: hit_record.v,
                material: hit_record.material,
                front_face: !hit_record.front_face,
            }),
            None => None,
        }
    }

    pub fn bounding_box(&self, t0: Float, t1: Float) -> Option<AABB> {
        self.object.bounding_box(t0, t1)
    }
}
