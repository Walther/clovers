//! `ConstantMedium` object. This should probably be a [Material] at some point, but this will do for now. This is essentially a fog with a known size, shape and density.

use crate::{
    aabb::AABB,
    hitable::{HitRecord, Hitable, HitableTrait},
    materials::{isotropic::Isotropic, Material},
    random::random_unit_vector,
    ray::Ray,
    textures::Texture,
    wavelength::Wavelength,
    Box, Direction, Float, Vec3, EPSILON_CONSTANT_MEDIUM,
};
use rand::rngs::SmallRng;
use rand::Rng;

use super::Object;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// `ConstantMediumInit` structure describes the necessary data for constructing a [`ConstantMedium`]. Used with [serde] when importing [`SceneFiles`](crate::scenes::SceneFile).
pub struct ConstantMediumInit {
    /// Used for multiple importance sampling
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub priority: bool,
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

#[derive(Debug, Clone)]
/// `ConstantMedium` object. This should probably be a [Material] at some point, but this will do for now. This is essentially a fog with a known size, shape and density.
pub struct ConstantMedium<'scene> {
    boundary: Box<Hitable<'scene>>,
    phase_function: Material,
    neg_inv_density: Float,
}

impl<'scene> ConstantMedium<'scene> {
    /// Creates a new [`ConstantMedium`] with a known size, shape and density.
    #[must_use]
    pub fn new(boundary: Box<Hitable<'scene>>, density: Float, texture: Texture) -> Self {
        ConstantMedium {
            boundary,
            phase_function: Material::Isotropic(Isotropic::new(texture)),
            neg_inv_density: -1.0 / density,
        }
    }
}

impl<'scene> HitableTrait for ConstantMedium<'scene> {
    /// Hit function for the [`ConstantMedium`] object. Returns a [`HitRecord`] if hit. TODO: explain the math for the fog
    #[must_use]
    fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        rng: &mut SmallRng,
    ) -> Option<HitRecord> {
        // TODO: explain how the fog works.

        let Some(mut rec1) = self
            .boundary
            .hit(ray, Float::NEG_INFINITY, Float::INFINITY, rng)
        else {
            return None;
        };

        let Some(mut rec2) = self.boundary.hit(
            ray,
            rec1.distance + EPSILON_CONSTANT_MEDIUM,
            Float::INFINITY,
            rng,
        ) else {
            return None;
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

        let normal: Direction = random_unit_vector(rng); // tutorial says: arbitrary
        let front_face: bool = true; // tutorial says: also arbitrary

        let u = rec1.u;
        let v = rec1.v;

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
    #[must_use]
    fn bounding_box(&self, t0: Float, t1: Float) -> Option<&AABB> {
        self.boundary.bounding_box(t0, t1)
    }

    /// Returns a probability density function value based on the boundary object
    #[must_use]
    fn pdf_value(
        &self,
        origin: Vec3,
        direction: Direction,
        wavelength: Wavelength,
        time: Float,
        rng: &mut SmallRng,
    ) -> Float {
        self.boundary
            .pdf_value(origin, direction, wavelength, time, rng)
    }

    /// Returns a random point on the surface of the boundary of the fog
    // TODO: should this return a random point inside the volume instead?
    #[must_use]
    fn random(&self, origin: Vec3, rng: &mut SmallRng) -> Vec3 {
        self.boundary.random(origin, rng)
    }
}
