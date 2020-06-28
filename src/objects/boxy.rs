use crate::{
    hitable::{HitRecord, Hitable, HitableList, AABB},
    material::Material,
    ray::Ray,
    rect::{XYRect, XZRect, YZRect},
    Float, Vec3,
};
use rand::prelude::*;
use std::sync::Arc;

// Avoid keyword clash
pub struct Boxy {
    corner_0: Vec3,
    corner_1: Vec3,
    sides: HitableList,
    material: Arc<dyn Material>,
}

impl Boxy {
    pub fn new(corner_0: Vec3, corner_1: Vec3, material: Arc<dyn Material>) -> Boxy {
        let mut sides = HitableList::new();
        sides.hitables.push(Arc::new(XYRect::new(
            corner_0.x,
            corner_1.x,
            corner_0.y,
            corner_1.y,
            corner_1.z,
            Arc::clone(&material),
        )));
        sides.hitables.push(Arc::new(XYRect::new(
            corner_0.x,
            corner_1.x,
            corner_0.y,
            corner_1.y,
            corner_0.z,
            Arc::clone(&material),
        )));

        sides.hitables.push(Arc::new(XZRect::new(
            corner_0.x,
            corner_1.x,
            corner_0.z,
            corner_1.z,
            corner_1.y,
            Arc::clone(&material),
        )));
        sides.hitables.push(Arc::new(XZRect::new(
            corner_0.x,
            corner_1.x,
            corner_0.z,
            corner_1.z,
            corner_0.y,
            Arc::clone(&material),
        )));

        sides.hitables.push(Arc::new(YZRect::new(
            corner_0.y,
            corner_1.y,
            corner_0.z,
            corner_1.z,
            corner_1.x,
            Arc::clone(&material),
        )));
        sides.hitables.push(Arc::new(YZRect::new(
            corner_0.y,
            corner_1.y,
            corner_0.z,
            corner_1.z,
            corner_0.x,
            Arc::clone(&material),
        )));

        Boxy {
            corner_0,
            corner_1,
            sides,
            material,
        }
    }
}

impl Hitable for Boxy {
    fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        rng: ThreadRng,
    ) -> Option<HitRecord> {
        self.sides.hit(ray, distance_min, distance_max, rng)
    }
    fn bounding_box(&self, _t0: crate::Float, _t1: crate::Float) -> Option<AABB> {
        Some(AABB::new(self.corner_0, self.corner_1))
    }
}
