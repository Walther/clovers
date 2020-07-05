use super::{XYRect, XZRect, YZRect};
use crate::{
    hitable::{HitRecord, Hitable, HitableList, AABB},
    materials::Material,
    ray::Ray,
    Float, Vec3,
};
use rand::prelude::*;
use std::sync::Arc;

// Avoid keyword clash
pub struct Boxy {
    corner_0: Vec3,
    corner_1: Vec3,
    sides: HitableList,
    material: Material,
}

impl Boxy {
    pub fn new(corner_0: Vec3, corner_1: Vec3, material: Material) -> Hitable {
        let mut sides: Vec<Hitable> = Vec::new();
        sides.push(XYRect::new(
            corner_0.x, corner_1.x, corner_0.y, corner_1.y, corner_1.z, material,
        ));
        sides.push(XYRect::new(
            corner_0.x, corner_1.x, corner_0.y, corner_1.y, corner_0.z, material,
        ));

        sides.push(XZRect::new(
            corner_0.x, corner_1.x, corner_0.z, corner_1.z, corner_1.y, material,
        ));
        sides.push(XZRect::new(
            corner_0.x, corner_1.x, corner_0.z, corner_1.z, corner_0.y, material,
        ));

        sides.push(YZRect::new(
            corner_0.y, corner_1.y, corner_0.z, corner_1.z, corner_1.x, material,
        ));
        sides.push(YZRect::new(
            corner_0.y, corner_1.y, corner_0.z, corner_1.z, corner_0.x, material,
        ));

        Hitable::Boxy(Boxy {
            corner_0,
            corner_1,
            sides: HitableList::from(sides),
            material,
        })
    }

    pub fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        rng: ThreadRng,
    ) -> Option<HitRecord> {
        self.sides.hit(ray, distance_min, distance_max, rng)
    }

    pub fn bounding_box(&self, _t0: crate::Float, _t1: crate::Float) -> Option<AABB> {
        Some(AABB::new(self.corner_0, self.corner_1))
    }
}
