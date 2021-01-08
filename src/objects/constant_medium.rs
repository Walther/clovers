use crate::{
    aabb::AABB,
    hitable::{HitRecord, Hitable},
    materials::{isotropic::Isotropic, Material},
    ray::Ray,
    textures::Texture,
    Float, Vec3, EPSILON_CONSTANT_MEDIUM,
};
use rand::prelude::*;
use serde::{Deserialize, Serialize};

use std::sync::Arc;

use super::Object;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConstantMediumInit {
    pub boundary: Box<Object>,
    #[serde(default = "default_density")]
    pub density: Float,
    #[serde(default)]
    pub texture: Texture,
}

// TODO: does this density setting even work?
fn default_density() -> Float {
    0.1
    // 1e-9
    // 1e9
}

pub struct ConstantMedium {
    boundary: Arc<Hitable>,
    phase_function: Material,
    neg_inv_density: Float,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<Hitable>, density: Float, texture: Texture) -> Hitable {
        Hitable::ConstantMedium(ConstantMedium {
            boundary,
            phase_function: Isotropic::new(texture),
            neg_inv_density: -1.0 / density,
        })
    }

    pub fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        mut rng: ThreadRng,
    ) -> Option<HitRecord> {
        let mut rec1: HitRecord;
        let mut rec2: HitRecord;

        rec1 = match self
            .boundary
            .hit(ray, Float::NEG_INFINITY, Float::INFINITY, rng)
        {
            Some(record) => record,
            None => return None,
        };

        rec2 = match self.boundary.hit(
            ray,
            rec1.distance + EPSILON_CONSTANT_MEDIUM,
            Float::INFINITY,
            rng,
        ) {
            Some(record) => record,
            None => return None,
        };

        if rec1.distance < distance_min {
            rec1.distance = distance_min;
        }
        if rec2.distance > distance_max {
            rec2.distance = distance_max;
        }

        if rec1.distance >= rec2.distance {
            return None;
        }

        if rec1.distance < 0.0 {
            rec1.distance = 0.0;
        }

        let ray_length: Float = ray.direction.norm();
        let distance_inside_boundary: Float = (rec2.distance - rec1.distance) * ray_length;
        let hit_distance: Float = self.neg_inv_density * (rng.gen::<Float>()).ln(); // TODO: verify if log_e is correct here

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let distance = rec1.distance + hit_distance / ray_length;
        let position = ray.point_at_parameter(distance);

        let normal: Vec3 = Vec3::new(1.0, 0.0, 0.0); // tutorial says: arbitrary
        let front_face: bool = true; // tutorial says: also arbitrary

        let u: Float = 0.5; // TODO: should this be something sensible?
        let v: Float = 0.5; // TODO: should this be something sensible?

        Some(HitRecord {
            distance,
            position,
            normal,
            u,
            v,
            material: &self.phase_function,
            front_face,
        })
    }

    pub fn bounding_box(&self, t0: Float, t1: Float) -> Option<AABB> {
        self.boundary.bounding_box(t0, t1)
    }
}
