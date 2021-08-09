//! A box or a cuboid object: a parallelepiped with six rectangular faces. Named [Boxy] to avoid clashing with [Box].

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

/// BoxyInit structure describes the necessary data for constructing a [Boxy]. Used with [serde] when importing [SceneFiles](crate::scenes::SceneFile).
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct BoxyInit {
    /// First corner for the box
    pub corner_0: Vec3,
    /// Second, opposing corner for the box
    pub corner_1: Vec3,
    #[serde(default)]
    /// Material used for the box
    pub material: Material,
}

/// A box or a cuboid object: a parallelepiped with six rectangular faces. Named [Boxy] to avoid clashing with [Box].
#[derive(Debug)]
pub struct Boxy {
    corner_0: Vec3,
    corner_1: Vec3,
    sides: Arc<HitableList>,
    /// The material of the box
    pub material: Material,
}

impl Boxy {
    /// Initializes a new instance of a box, given two opposing [Vec3] corners `corner_0` and `corner_1`, and a [Material] `material`.
    pub fn new(corner_0: Vec3, corner_1: Vec3, material: Material) -> Self {
        let mut sides = HitableList::new();
        sides.add(Hitable::XYRect(XYRect::new(
            corner_0.x, corner_1.x, corner_0.y, corner_1.y, corner_1.z, material,
        )));
        sides.add(Hitable::XYRect(XYRect::new(
            corner_0.x, corner_1.x, corner_0.y, corner_1.y, corner_0.z, material,
        )));

        sides.add(Hitable::XZRect(XZRect::new(
            corner_0.x, corner_1.x, corner_0.z, corner_1.z, corner_1.y, material,
        )));
        sides.add(Hitable::XZRect(XZRect::new(
            corner_0.x, corner_1.x, corner_0.z, corner_1.z, corner_0.y, material,
        )));

        sides.add(Hitable::YZRect(YZRect::new(
            corner_0.y, corner_1.y, corner_0.z, corner_1.z, corner_1.x, material,
        )));
        sides.add(Hitable::YZRect(YZRect::new(
            corner_0.y, corner_1.y, corner_0.z, corner_1.z, corner_0.x, material,
        )));

        Boxy {
            corner_0,
            corner_1,
            sides: Arc::new(sides),
            material,
        }
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
