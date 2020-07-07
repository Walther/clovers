use crate::{
    hitable::{HitRecord, Hitable, AABB},
    ray::Ray,
    Float,
};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
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

    pub fn bounding_box(&self, t0: crate::Float, t1: crate::Float) -> Option<AABB> {
        self.object.bounding_box(t0, t1)
    }
}

// class flip_face : public hittable {
//   public:
//       flip_face(shared_ptr<hittable> p) : ptr(p) {}

//       virtual bool hit(const ray& r, double t_min, double t_max, hit_record& rec) const {
//           if (!ptr->hit(r, t_min, t_max, rec))
//               return false;

//           rec.front_face = !rec.front_face;
//           return true;
//       }

//       virtual bool bounding_box(double t0, double t1, aabb& output_box) const {
//           return ptr->bounding_box(t0, t1, output_box);
//       }

//   public:
//       shared_ptr<hittable> ptr;
// };
