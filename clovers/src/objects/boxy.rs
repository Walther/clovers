//! A box or a cuboid object: a parallelepiped with six rectangular faces. Named [Boxy] to avoid clashing with [Box].

use super::Quad;
use crate::{
    aabb::AABB,
    hitable::{HitRecord, Hitable, HitableList},
    materials::Material,
    ray::Ray,
    Box, Float, Vec3,
};
use rand::rngs::SmallRng;

/// BoxyInit structure describes the necessary data for constructing a [Boxy]. Used with [serde] when importing [SceneFiles](crate::scenes::SceneFile).
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
pub struct BoxyInit {
    /// First corner for the box
    pub corner_0: Vec3,
    /// Second, opposing corner for the box
    pub corner_1: Vec3,
    #[cfg_attr(feature = "serde-derive", serde(default))]
    /// Material used for the box
    pub material: Material,
}

/// A box or a cuboid object: a parallelepiped with six rectangular faces. Named [Boxy] to avoid clashing with [Box].
#[derive(Debug, Clone)]
pub struct Boxy {
    corner_0: Vec3,
    corner_1: Vec3,
    sides: Box<HitableList>,
    /// The material of the box
    pub material: Material,
}

impl Boxy {
    /// Initializes a new instance of a box, given two opposing [Vec3] corners `corner_0` and `corner_1`, and a [Material] `material`.
    pub fn new(corner_0: Vec3, corner_1: Vec3, material: Material) -> Self {
        let mut sides = HitableList::new();

        // Construct the two opposite vertices with the minimum and maximum coordinates.
        let min: Vec3 = Vec3::new(
            corner_0.x.min(corner_1.x),
            corner_0.y.min(corner_1.y),
            corner_0.z.min(corner_1.z),
        );
        let max: Vec3 = Vec3::new(
            corner_0.x.max(corner_1.x),
            corner_0.y.max(corner_1.y),
            corner_0.z.max(corner_1.z),
        );

        let dx: Vec3 = Vec3::new(max.x - min.x, 0.0, 0.0);
        let dy: Vec3 = Vec3::new(0.0, max.y - min.y, 0.0);
        let dz: Vec3 = Vec3::new(0.0, 0.0, max.z - min.z);

        // front
        sides.add(Hitable::Quad(Quad::new(
            Vec3::new(min.x, min.y, max.z),
            dx,
            dy,
            material,
        )));

        // right
        sides.add(Hitable::Quad(Quad::new(
            Vec3::new(max.x, min.y, max.z),
            -dz,
            dy,
            material,
        )));

        // back
        sides.add(Hitable::Quad(Quad::new(
            Vec3::new(max.x, min.y, min.z),
            -dx,
            dy,
            material,
        )));

        // left
        sides.add(Hitable::Quad(Quad::new(
            Vec3::new(min.x, min.y, min.z),
            dz,
            dy,
            material,
        )));

        // top
        sides.add(Hitable::Quad(Quad::new(
            Vec3::new(min.x, max.y, max.z),
            dx,
            -dz,
            material,
        )));

        // bottom
        sides.add(Hitable::Quad(Quad::new(
            Vec3::new(min.x, min.y, min.z),
            dx,
            dz,
            material,
        )));

        Boxy {
            corner_0,
            corner_1,
            sides: Box::new(sides),
            material,
        }
    }

    /// The main `hit` function for a [Boxy]. Given a [Ray](crate::ray::Ray), and an interval `distance_min` and `distance_max`, returns either `None` or `Some(HitRecord)` based on whether the ray intersects with the object during that interval.
    pub fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        rng: &mut SmallRng,
    ) -> Option<HitRecord> {
        self.sides.hit(ray, distance_min, distance_max, rng)
    }

    /// Returns the axis-aligned bounding box [AABB] of the object.
    pub fn bounding_box(&self, _t0: Float, _t1: Float) -> Option<AABB> {
        Some(AABB::new(self.corner_0, self.corner_1))
    }
}
