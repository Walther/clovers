use crate::{
    hitable::{HitRecord, Hitable},
    ray::Ray,
    Float, Vec3, AABB,
};
use rand::prelude::*;
use std::sync::Arc;

pub struct Translate {
    object: Arc<Hitable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(object: Arc<Hitable>, offset: Vec3) -> Hitable {
        Hitable::Translate(Translate {
            object: object,
            offset,
        })
    }

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

    pub fn bounding_box(&self, t0: Float, t1: Float) -> Option<AABB> {
        let object_bounding_box = self.object.bounding_box(t0, t1);
        match object_bounding_box {
            Some(aabb) => Some(AABB::new(aabb.min + self.offset, aabb.max + self.offset)),
            None => None,
        }
    }
}
