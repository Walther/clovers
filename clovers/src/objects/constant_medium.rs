//! `ConstantMedium` object. This should probably be a Material at some point, but this will do for now. This is essentially a fog with a known size, shape and density.

use crate::{
    aabb::AABB,
    hitable::{Hitable, HitableTrait},
    materials::{isotropic::Isotropic, Kind},
    random::random_unit_vector,
    ray::Ray,
    textures::Texture,
    wavelength::Wavelength,
    Box, Direction, Float, HitRecord, Position, EPSILON_CONSTANT_MEDIUM,
};
use rand::rngs::SmallRng;
use rand::Rng;

use super::Object;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// `ConstantMediumInit` structure describes the necessary data for constructing a [`ConstantMedium`].
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
/// `ConstantMedium` object. This should probably be a Material at some point, but this will do for now. This is essentially a fog with a known size, shape and density.
pub struct ConstantMedium<'scene> {
    boundary: Box<Hitable<'scene>>,
    phase_function: Kind,
    neg_inv_density: Float,
}

impl<'scene> ConstantMedium<'scene> {
    /// Creates a new [`ConstantMedium`] with a known size, shape and density.
    #[must_use]
    pub fn new(boundary: Box<Hitable<'scene>>, density: Float, texture: Texture) -> Self {
        ConstantMedium {
            boundary,
            phase_function: Kind::Isotropic(Isotropic::new(texture)),
            neg_inv_density: -1.0 / density,
        }
    }
}

impl HitableTrait for ConstantMedium<'_> {
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

        let mut rec1 = self
            .boundary
            .hit(ray, Float::NEG_INFINITY, Float::INFINITY, rng)?;

        let mut rec2 = self.boundary.hit(
            ray,
            rec1.distance + EPSILON_CONSTANT_MEDIUM,
            Float::INFINITY,
            rng,
        )?;

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
        let hit_distance: Float = self.neg_inv_density * (rng.random::<Float>()).ln(); // TODO: verify if log_e is correct here

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
    fn aabb(&self) -> Option<&AABB> {
        self.boundary.aabb()
    }

    /// Returns a probability density function value based on the boundary object
    #[must_use]
    fn pdf_value(
        &self,
        origin: Position,
        direction: Direction,
        wavelength: Wavelength,
        time: Float,
        rng: &mut SmallRng,
    ) -> Float {
        self.boundary
            .pdf_value(origin, direction, wavelength, time, rng)
    }

    fn centroid(&self) -> Position {
        self.boundary.centroid()
    }
}
