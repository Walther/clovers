use super::{XYRect, XZRect, YZRect};
use crate::{
    aabb::AABB,
    hitable::{HitRecord, Hitable, HitableList},
    materials::Material,
    ray::Ray,
    Float, Vec3,
};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Used for the scene files etc
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct BoxyInit {
    pub corner_0: Vec3,
    pub corner_1: Vec3,
    #[serde(default)]
    pub material: Material,
}

/// A box or a cuboid object: a parallelepiped with six rectangular faces. Named [Boxy] to avoid clashing with [Box].
#[derive(Debug)]
pub struct Boxy {
    corner_0: Vec3,
    corner_1: Vec3,
    sides: Arc<HitableList>,
    pub material: Material,
}

impl Boxy {
    /// Initializes a new instance of a box, given two opposing [Vec3] corners `corner_0` and `corner_1`, and a [Material] `material`.
    pub fn new(corner_0: Vec3, corner_1: Vec3, material: Material) -> Hitable {
        let mut sides = HitableList::new();
        sides.add(XYRect::new(
            corner_0.x, corner_1.x, corner_0.y, corner_1.y, corner_1.z, material,
        ));
        sides.add(XYRect::new(
            corner_0.x, corner_1.x, corner_0.y, corner_1.y, corner_0.z, material,
        ));

        sides.add(XZRect::new(
            corner_0.x, corner_1.x, corner_0.z, corner_1.z, corner_1.y, material,
        ));
        sides.add(XZRect::new(
            corner_0.x, corner_1.x, corner_0.z, corner_1.z, corner_0.y, material,
        ));

        sides.add(YZRect::new(
            corner_0.y, corner_1.y, corner_0.z, corner_1.z, corner_1.x, material,
        ));
        sides.add(YZRect::new(
            corner_0.y, corner_1.y, corner_0.z, corner_1.z, corner_0.x, material,
        ));

        Hitable::Boxy(Boxy {
            corner_0,
            corner_1,
            sides: Arc::new(sides),
            material,
        })
    }

    /// The main `hit` function for a [Boxy]. Given a [Ray](crate::ray::Ray), and an interval `distance_min` and `distance_max`, returns either `None` or `Some(HitRecord)` based on whether the ray intersects with the object during that interval.
    pub fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        rng: ThreadRng,
    ) -> Option<HitRecord> {
        self.sides.hit(ray, distance_min, distance_max, rng)
    }

    /// Returns the axis-aligned bounding box [AABB] of the object.
    pub fn bounding_box(&self, _t0: Float, _t1: Float) -> Option<AABB> {
        Some(AABB::new(self.corner_0, self.corner_1))
    }
}
