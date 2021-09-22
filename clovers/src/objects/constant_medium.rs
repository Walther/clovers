//! ConstantMedium object. This should probably be a [Material] at some point, but this will do for now. This is essentially a fog with a known size, shape and density.

use crate::CloversRng;
use crate::{
    aabb::AABB,
    hitable::{HitRecord, Hitable},
    materials::{isotropic::Isotropic, Material},
    ray::Ray,
    textures::Texture,
    Box, Float, Vec3, EPSILON_CONSTANT_MEDIUM,
};
// TODO: fix trait import
#[cfg(feature = "rand-crate")]
#[cfg(not(target_arch = "spirv"))]
use rand::Rng;

use super::Object;

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// ConstantMediumInit structure describes the necessary data for constructing a [ConstantMedium]. Used with [serde] when importing [SceneFiles](crate::scenes::SceneFile).
pub struct ConstantMediumInit {
    /// The boundary object for the constant medium. This determines the size and shape of the fog object.
    pub boundary: Box<Object>,
    #[cfg_attr(feature = "serde-derive", serde(default = "default_density"))]
    /// Density of the fog. TODO: example good value range?
    pub density: Float,
    #[cfg_attr(feature = "serde-derive", serde(default))]
    /// [Texture] used for the colorization of the fog.
    pub texture: Texture,
}

#[cfg(feature = "serde-derive")]
// TODO: does this density setting even work?
fn default_density() -> Float {
    0.1
    // 1e-9
    // 1e9
}

#[derive(Clone)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
/// ConstantMedium object. This should probably be a [Material] at some point, but this will do for now. This is essentially a fog with a known size, shape and density.
pub struct ConstantMedium {
    boundary: Box<Hitable>,
    phase_function: Material,
    neg_inv_density: Float,
}

impl ConstantMedium {
    /// Creates a new [ConstantMedium] with a known size, shape and density.
    pub fn new(boundary: Box<Hitable>, density: Float, texture: Texture) -> Self {
        ConstantMedium {
            boundary,
            phase_function: Material::Isotropic(Isotropic::new(texture)),
            neg_inv_density: -1.0 / density,
        }
    }

    /// Hit function for the [ConstantMedium] object. Returns a [HitRecord] if hit. TODO: explain the math for the fog
    pub fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        rng: &mut CloversRng,
    ) -> Option<HitRecord> {
        let mut rec1: HitRecord;
        let mut rec2: HitRecord;

        // TODO: explain how the fog works.

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
        let position = ray.evaluate(distance);

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

    /// Returns the axis-aligned bounding box [AABB] of the defining `boundary` object for the fog.
    pub fn bounding_box(&self, t0: Float, t1: Float) -> Option<AABB> {
        self.boundary.bounding_box(t0, t1)
    }
}
