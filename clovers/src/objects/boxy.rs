//! A box or a cuboid object: a parallelepiped with six rectangular faces. Named [Boxy] to avoid clashing with [Box].

use super::Quad;
use crate::{
    aabb::AABB,
    hitable::{HitRecord, Hitable, HitableTrait},
    materials::{Material, MaterialInit},
    ray::Ray,
    wavelength::Wavelength,
    Box, Direction, Float, Position, Vec3,
};
use rand::rngs::SmallRng;

/// `BoxyInit` structure describes the necessary data for constructing a [Boxy]. Used with [serde] when importing [`SceneFile`](crate::scenes::SceneFile)s.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
pub struct BoxyInit {
    /// Used for multiple importance sampling
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub priority: bool,
    /// First corner for the box
    pub corner_0: Position,
    /// Second, opposing corner for the box
    pub corner_1: Position,
    #[cfg_attr(feature = "serde-derive", serde(default))]
    /// Material used for the box
    pub material: MaterialInit,
}

/// A box or a cuboid object: a parallelepiped with six rectangular faces. Named [Boxy] to avoid clashing with [Box].
#[derive(Debug, Clone)]
pub struct Boxy<'scene> {
    sides: Box<[Hitable<'scene>; 6]>,
    /// The material of the box
    pub material: &'scene Material,
    /// Axis-aligned bounding box
    pub aabb: AABB,
}

impl<'scene> Boxy<'scene> {
    /// Initializes a new instance of a box, given two opposing [Vec3] corners `corner_0` and `corner_1`, and a [Material] `material`.
    #[must_use]
    pub fn new(corner_0: Position, corner_1: Position, material: &'scene Material) -> Self {
        // Construct the two opposite vertices with the minimum and maximum coordinates.
        let min: Position = Position::new(
            corner_0.x.min(corner_1.x),
            corner_0.y.min(corner_1.y),
            corner_0.z.min(corner_1.z),
        );
        let max: Position = Position::new(
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
}

impl<'scene> HitableTrait for Boxy<'scene> {
    /// The main `hit` function for a [Boxy]. Given a [Ray], and an interval `distance_min` and `distance_max`, returns either `None` or `Some(HitRecord)` based on whether the ray intersects with the object during that interval.
    #[must_use]
    fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        rng: &mut SmallRng,
    ) -> Option<HitRecord> {
        // start with an empty hit_record, hit all sides, return closest
        let mut hit_record: Option<HitRecord> = None;
        let mut closest = distance_max;
        for hitable in &*self.sides {
            if let Some(record) = hitable.hit(ray, distance_min, closest, rng) {
                closest = record.distance;
                hit_record = Some(record);
            }
        }
        hit_record
    }

    /// Returns the axis-aligned bounding box [AABB] of the object.
    #[must_use]
    fn bounding_box(&self, _t0: Float, _t1: Float) -> Option<&AABB> {
        Some(&self.aabb)
    }

    // TODO: improve correctness & optimization!
    /// Returns a probability density function value?
    #[must_use]
    fn pdf_value(
        &self,
        origin: Position,
        direction: Direction,
        wavelength: Wavelength,
        time: Float,
        rng: &mut SmallRng,
    ) -> Float {
        let mut sum = 0.0;

        self.sides.iter().for_each(|object| {
            sum += object.pdf_value(origin, direction, wavelength, time, rng) / 6.0;
        });

        sum
    }
}
