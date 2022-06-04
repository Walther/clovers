//! A box or a cuboid object: a parallelepiped with six rectangular faces. Named [Boxy] to avoid clashing with [Box].

use super::Quad;
use crate::{
    aabb::AABB,
    hitable::{HitRecord, Hitable},
    materials::Material,
    ray::Ray,
    Box, Float, Vec3,
};
use rand::{rngs::SmallRng, Rng};

/// `BoxyInit` structure describes the necessary data for constructing a [Boxy]. Used with [serde] when importing [`SceneFile`](crate::scenes::SceneFile)s.
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
    sides: Box<[Hitable; 6]>,
    /// The material of the box
    pub material: Material,
    /// Axis-aligned bounding box
    pub aabb: AABB,
}

impl Boxy {
    /// Initializes a new instance of a box, given two opposing [Vec3] corners `corner_0` and `corner_1`, and a [Material] `material`.
    #[must_use]
    pub fn new(corner_0: Vec3, corner_1: Vec3, material: Material) -> Self {
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

        let sides: [Hitable; 6] = [
            // front
            Hitable::Quad(Quad::new(Vec3::new(min.x, min.y, max.z), dx, dy, material)),
            // right
            Hitable::Quad(Quad::new(Vec3::new(max.x, min.y, max.z), -dz, dy, material)),
            // back
            Hitable::Quad(Quad::new(Vec3::new(max.x, min.y, min.z), -dx, dy, material)),
            // left
            Hitable::Quad(Quad::new(Vec3::new(min.x, min.y, min.z), dz, dy, material)),
            // top
            Hitable::Quad(Quad::new(Vec3::new(min.x, max.y, max.z), dx, -dz, material)),
            // bottom
            Hitable::Quad(Quad::new(Vec3::new(min.x, min.y, min.z), dx, dz, material)),
        ];
        // AABB
        let aabb = AABB::new_from_coords(corner_0, corner_1);

        Boxy {
            sides: Box::new(sides),
            material,
            aabb,
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
        // start with an empty hit_record, hit all sides, return closest
        let mut hit_record: Option<HitRecord> = None;
        let mut closest = distance_max;
        for hitable in self.sides.iter() {
            if let Some(record) = hitable.hit(ray, distance_min, closest, rng) {
                closest = record.distance;
                hit_record = Some(record);
            }
        }
        hit_record
    }

    /// Returns the axis-aligned bounding box [AABB] of the object.
    #[must_use]
    pub fn bounding_box(&self, _t0: Float, _t1: Float) -> Option<AABB> {
        Some(self.aabb)
    }

    /// Returns a probability density function value? // TODO: understand & explain
    pub fn pdf_value(&self, origin: Vec3, vector: Vec3, time: Float, rng: &mut SmallRng) -> Float {
        let mut sum = 0.0;

        self.sides.iter().for_each(|object| {
            sum += object.pdf_value(origin, vector, time, rng) / 6.0;
        });

        sum
    }

    /// Returns a random point on the box
    pub fn random(&self, origin: Vec3, rng: &mut SmallRng) -> Vec3 {
        let index: usize = rng.gen_range(0..7);
        self.sides[index].random(origin, rng)
    }
}
