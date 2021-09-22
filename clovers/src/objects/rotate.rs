//! Utility object for rotating another object.

use crate::CloversRng;
use crate::{
    aabb::AABB,
    hitable::{HitRecord, Hitable},
    ray::Ray,
    Box, Float, Vec3,
};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float as FloatTrait;

use super::Object;

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// RotateInit structure describes the necessary data for constructing a [RotateY]. Used with [serde] when importing [SceneFiles](crate::scenes::SceneFile).
pub struct RotateInit {
    /// The encased [Object] to rotate
    pub object: Box<Object>,
    /// Angle to rotate the object, in degrees
    pub angle: Float,
}

#[derive(Clone)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
/// RotateY object. It wraps the given [Object] and has adjusted `hit()` and `bounding_box()` methods based on the `angle` given.
pub struct RotateY {
    object: Box<Hitable>,
    sin_theta: Float,
    cos_theta: Float,
    aabb: Option<AABB>,
}

impl RotateY {
    /// Creates a new `RotateY` object. It wraps the given [Object] and has adjusted `hit()` and `bounding_box()` methods based on the `angle` given.
    pub fn new(object: Box<Hitable>, angle: Float) -> Self {
        // TODO: add proper time support
        let time_0: Float = 0.0;
        let time_1: Float = 1.0;
        let radians: Float = angle.to_radians();
        let sin_theta: Float = radians.sin();
        let cos_theta: Float = radians.cos();
        let bounding_box: Option<AABB> = object.bounding_box(time_0, time_1);

        // Does our object have a bounding box?
        let bbox = match bounding_box {
            // No bounding box for object, give up and return early
            // TODO: is this even correct? How could it be?
            None => {
                return RotateY {
                    object,
                    sin_theta,
                    cos_theta,
                    aabb: None,
                }
            }
            // Got a bounding box
            Some(bbox) => bbox,
        };

        // Start with infinite bounds
        let mut min: Vec3 = Vec3::new(Float::INFINITY, Float::INFINITY, Float::INFINITY);
        let mut max: Vec3 = Vec3::new(
            Float::NEG_INFINITY,
            Float::NEG_INFINITY,
            Float::NEG_INFINITY,
        );

        // Calculate new bounds
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i_f: Float = i as Float;
                    let j_f: Float = j as Float;
                    let k_f: Float = k as Float;

                    let x: Float = i_f * bbox.x.max + (1.0 - i_f) * bbox.x.min;
                    let y: Float = j_f * bbox.y.max + (1.0 - j_f) * bbox.y.min;
                    let z: Float = k_f * bbox.z.max + (1.0 - k_f) * bbox.z.min;

                    let newx: Float = cos_theta * x + sin_theta * z;
                    let newz: Float = -sin_theta * x + cos_theta * z;

                    let tester: Vec3 = Vec3::new(newx, y, newz);

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

    /// Hit method for the [RotateY] object. Finds the rotation-adjusted [HitRecord] for the possible intersection of the [Ray] with the encased [Object].
    pub fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        rng: &mut CloversRng,
    ) -> Option<HitRecord> {
        let mut origin: Vec3 = ray.origin;
        let mut direction: Vec3 = ray.direction;

        origin[0] = self.cos_theta * ray.origin[0] - self.sin_theta * ray.origin[2];
        origin[2] = self.sin_theta * ray.origin[0] + self.cos_theta * ray.origin[2];

        direction[0] = self.cos_theta * ray.direction[0] - self.sin_theta * ray.direction[2];
        direction[2] = self.sin_theta * ray.direction[0] + self.cos_theta * ray.direction[2];

        let rotated_r: Ray = Ray::new(origin, direction, ray.time);

        let hit_record = match self.object.hit(&rotated_r, distance_min, distance_max, rng) {
            // Did not hit rotated object, early return None
            None => return None,
            // Hit the rotated object, continue evaluating
            Some(hit_record) => hit_record,
        };

        // Determine where the intersection is
        // TODO: understand and explain
        let mut position: Vec3 = hit_record.position;
        let mut normal: Vec3 = hit_record.normal;
        let distance: Float = hit_record.distance;

        position[0] =
            self.cos_theta * hit_record.position[0] + self.sin_theta * hit_record.position[2];
        position[2] =
            -self.sin_theta * hit_record.position[0] + self.cos_theta * hit_record.position[2];

        normal[0] = self.cos_theta * hit_record.normal[0] + self.sin_theta * hit_record.normal[2];
        normal[2] = -self.sin_theta * hit_record.normal[0] + self.cos_theta * hit_record.normal[2];

        // TODO: uv coords? This probably breaks surface textures completely
        let mut record = HitRecord {
            distance,
            position,
            normal,
            u: 0.0,
            v: 0.0,
            material: &*hit_record.material,
            front_face: false, // TODO: fix having to declare it before calling face_normal
        };
        record.set_face_normal(&rotated_r, normal);
        Some(record)
    }

    /// Bounding box method for the [RotateY] object. Finds the axis-aligned bounding box [AABB] for the encased [Object] after adjusting for rotation.
    pub fn bounding_box(&self, _t0: Float, _t1: Float) -> Option<AABB> {
        self.aabb
    }
}
