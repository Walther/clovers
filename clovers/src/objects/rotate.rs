//! Utility object for rotating another object.

use crate::{
    aabb::AABB,
    hitable::{Hitable, HitableTrait},
    ray::Ray,
    wavelength::Wavelength,
    Box, Direction, Float, HitRecord, Position, Vec3,
};
use nalgebra::Unit;
use rand::rngs::SmallRng;

use super::Object;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// `RotateInit` structure describes the necessary data for constructing a [`RotateY`].
pub struct RotateInit {
    /// Used for multiple importance sampling
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub priority: bool,
    /// The encased [Object] to rotate
    pub object: Box<Object>,
    /// Angle to rotate the object, in degrees
    pub angle: Float,
}

#[derive(Debug, Clone)]
/// `RotateY` object. It wraps the given [Object] and has adjusted `hit()` and `bounding_box()` methods based on the `angle` given.
pub struct RotateY<'scene> {
    object: Box<Hitable<'scene>>,
    sin_theta: Float,
    cos_theta: Float,
    aabb: Option<AABB>,
}

impl<'scene> RotateY<'scene> {
    /// Creates a new `RotateY` object. It wraps the given [Object] and has adjusted `hit()` and `bounding_box()` methods based on the `angle` given.
    #[must_use]
    pub fn new(object: Box<Hitable<'scene>>, angle: Float) -> Self {
        // TODO: add proper time support
        let radians: Float = angle.to_radians();
        let sin_theta: Float = radians.sin();
        let cos_theta: Float = radians.cos();
        let bounding_box: Option<&AABB> = object.aabb();

        // Does our object have a bounding box?
        let Some(bbox) = bounding_box else {
            return RotateY {
                object,
                sin_theta,
                cos_theta,
                aabb: None,
            };
        };

        // Start with infinite bounds
        let mut min: Vec3 = Vec3::new(Float::INFINITY, Float::INFINITY, Float::INFINITY);
        let mut max: Vec3 = Vec3::new(
            Float::NEG_INFINITY,
            Float::NEG_INFINITY,
            Float::NEG_INFINITY,
        );

        // Calculate new bounds
        for i in [0.0, 1.0] {
            for j in [0.0, 1.0] {
                for k in [0.0, 1.0] {
                    let i_f: Float = i;
                    let j_f: Float = j;
                    let k_f: Float = k;

                    let x: Float = i_f * bbox.x.max + (1.0 - i_f) * bbox.x.min;
                    let y: Float = j_f * bbox.y.max + (1.0 - j_f) * bbox.y.min;
                    let z: Float = k_f * bbox.z.max + (1.0 - k_f) * bbox.z.min;

                    let new_x: Float = cos_theta * x + sin_theta * z;
                    let new_z: Float = -sin_theta * x + cos_theta * z;

                    let tester: Vec3 = Vec3::new(new_x, y, new_z);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        // Return a Rotate object with the new bounding box and pre-calculated rotation utilities
        RotateY {
            object,
            sin_theta,
            cos_theta,
            aabb: Some(AABB::new_from_coords(min, max)),
        }
    }
}

impl HitableTrait for RotateY<'_> {
    /// Hit method for the [`RotateY`] object. Finds the rotation-adjusted [`HitRecord`] for the possible intersection of the [Ray] with the encased [Object].
    #[must_use]
    fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        rng: &mut SmallRng,
    ) -> Option<HitRecord> {
        let mut origin: Position = ray.origin;
        let mut direction: Vec3 = *ray.direction;

        origin[0] = self.cos_theta * ray.origin[0] - self.sin_theta * ray.origin[2];
        origin[2] = self.sin_theta * ray.origin[0] + self.cos_theta * ray.origin[2];

        direction[0] = self.cos_theta * ray.direction[0] - self.sin_theta * ray.direction[2];
        direction[2] = self.sin_theta * ray.direction[0] + self.cos_theta * ray.direction[2];

        let direction = Unit::new_normalize(direction);

        let rotated_r: Ray = Ray {
            origin,
            direction,
            time: ray.time,
            wavelength: ray.wavelength,
        };

        let Some(hit_record) = self.object.hit(&rotated_r, distance_min, distance_max, rng) else {
            // Did not hit rotated object, early return None
            return None;
        };

        // Determine where the intersection is
        // TODO: understand and explain
        let mut position: Position = hit_record.position;
        let mut normal: Vec3 = *hit_record.normal;
        let distance: Float = hit_record.distance;

        position[0] =
            self.cos_theta * hit_record.position[0] + self.sin_theta * hit_record.position[2];
        position[2] =
            -self.sin_theta * hit_record.position[0] + self.cos_theta * hit_record.position[2];

        normal[0] = self.cos_theta * hit_record.normal[0] + self.sin_theta * hit_record.normal[2];
        normal[2] = -self.sin_theta * hit_record.normal[0] + self.cos_theta * hit_record.normal[2];

        let normal = Unit::new_normalize(normal);

        let mut record = HitRecord {
            distance,
            position,
            normal,
            u: hit_record.u,
            v: hit_record.v,
            material: hit_record.material,
            front_face: false, // TODO: fix having to declare it before calling face_normal
        };
        record.set_face_normal(&rotated_r, normal);
        Some(record)
    }

    /// Bounding box method for the [`RotateY`] object. Finds the axis-aligned bounding box [AABB] for the encased [Object] after adjusting for rotation.
    #[must_use]
    fn aabb(&self) -> Option<&AABB> {
        self.aabb.as_ref()
    }

    fn pdf_value(
        &self,
        _origin: Position,
        _direction: Direction,
        _wavelength: Wavelength,
        _time: Float,
        _rng: &mut SmallRng,
    ) -> Float {
        // TODO: fix
        0.0
    }

    // TODO: correctness!
    fn centroid(&self) -> Position {
        self.object.centroid()
    }
}
