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
pub struct RotateInit {
    pub object: Box<Object>,
    pub angle: Float,
}

pub struct RotateY {
    object: Arc<Hitable>,
    sin_theta: Float,
    cos_theta: Float,
    bounding_box: Option<AABB>,
}

impl RotateY {
    pub fn new(object: Arc<Hitable>, angle: Float) -> Hitable {
        // TODO: add proper time support
        let time_0: Float = 0.0;
        let time_1: Float = 1.0;
        let radians: Float = angle.to_radians();
        let sin_theta: Float = radians.sin();
        let cos_theta: Float = radians.cos();
        let bounding_box: Option<AABB> = object.bounding_box(time_0, time_1);

        match bounding_box {
            // No bounding box for object
            None => Hitable::RotateY(RotateY {
                object,
                sin_theta,
                cos_theta,
                bounding_box: None,
            }),
            // Got a bounding box
            Some(bbox) => {
                let mut min: Vec3 = Vec3::new(Float::INFINITY, Float::INFINITY, Float::INFINITY);
                let mut max: Vec3 = Vec3::new(
                    Float::NEG_INFINITY,
                    Float::NEG_INFINITY,
                    Float::NEG_INFINITY,
                );

                for i in 0..2 {
                    for j in 0..2 {
                        for k in 0..2 {
                            let i_f: Float = i as Float;
                            let j_f: Float = j as Float;
                            let k_f: Float = k as Float;

                            let x: Float = i_f * bbox.max.x + (1.0 - i_f) * bbox.min.x;
                            let y: Float = j_f * bbox.max.y + (1.0 - j_f) * bbox.min.y;
                            let z: Float = k_f * bbox.max.z + (1.0 - k_f) * bbox.min.z;

                            let newx: Float = cos_theta * x + sin_theta * z;
                            let newz: Float = -sin_theta * x + cos_theta * z;

                            let tester: Vec3 = Vec3::new(newx, y, newz);

                            for c in 0..3 {
                                min[c] = min[c].min(tester[c]);
                                max[c] = max[c].max(tester[c]);
                            }
                        }
                    }
                }

                Hitable::RotateY(RotateY {
                    object,
                    sin_theta,
                    cos_theta,
                    bounding_box: Some(AABB::new(min, max)),
                })
            }
        }
    }

    pub fn hit(
        &self,
        ray: &crate::ray::Ray,
        distance_min: Float,
        distance_max: Float,
        rng: ThreadRng,
    ) -> Option<HitRecord> {
        let mut origin: Vec3 = ray.origin;
        let mut direction: Vec3 = ray.direction;

        origin[0] = self.cos_theta * ray.origin[0] - self.sin_theta * ray.origin[2];
        origin[2] = self.sin_theta * ray.origin[0] + self.cos_theta * ray.origin[2];

        direction[0] = self.cos_theta * ray.direction[0] - self.sin_theta * ray.direction[2];
        direction[2] = self.sin_theta * ray.direction[0] + self.cos_theta * ray.direction[2];

        let rotated_r: Ray = Ray::new(origin, direction, ray.time);

        match self.object.hit(&rotated_r, distance_min, distance_max, rng) {
            // Did not hit rotated object, return None
            None => None,
            // Hit the rotated object
            Some(hit_record) => {
                // Determine where the intersection is
                let mut position: Vec3 = hit_record.position;
                let mut normal: Vec3 = hit_record.normal;
                let distance: Float = hit_record.distance;

                position[0] = self.cos_theta * hit_record.position[0]
                    + self.sin_theta * hit_record.position[2];
                position[2] = -self.sin_theta * hit_record.position[0]
                    + self.cos_theta * hit_record.position[2];

                normal[0] =
                    self.cos_theta * hit_record.normal[0] + self.sin_theta * hit_record.normal[2];
                normal[2] =
                    -self.sin_theta * hit_record.normal[0] + self.cos_theta * hit_record.normal[2];

                // TODO: uv coords?
                let mut record = HitRecord {
                    distance,
                    position,
                    normal,
                    u: 0.0,
                    v: 0.0,
                    material: &*hit_record.material,
                    front_face: false, // TODO: fix having to declare it before calling face_normal
                };
                record.set_face_normal(&rotated_r, normal);
                Some(record)
            }
        }
    }

    pub fn bounding_box(&self, _t0: Float, _t1: Float) -> Option<AABB> {
        self.bounding_box
    }
}
