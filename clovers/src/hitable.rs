//! An abstraction for things that can be hit by [Rays](crate::ray::Ray).

#[cfg(feature = "stl")]
use crate::objects::STL;
#[cfg(feature = "gl_tf")]
use crate::objects::{GLTFTriangle, GLTF};

use crate::{
    aabb::AABB,
    bvh::BVHNode,
    objects::{Boxy, ConstantMedium, MovingSphere, Quad, RotateY, Sphere, Translate, Triangle},
    ray::Ray,
    wavelength::Wavelength,
    Direction, Displacement, Float, HitRecord, Position,
};

use enum_dispatch::enum_dispatch;
use rand::rngs::SmallRng;

/// Enumeration of all runtime entities that can be intersected aka "hit" by a [Ray].
#[enum_dispatch(HitableTrait)]
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub enum Hitable<'scene> {
    Boxy(Boxy<'scene>),
    BVHNode(BVHNode<'scene>),
    ConstantMedium(ConstantMedium<'scene>),
    MovingSphere(MovingSphere<'scene>),
    Quad(Quad<'scene>),
    RotateY(RotateY<'scene>),
    Sphere(Sphere<'scene>),
    #[cfg(feature = "stl")]
    STL(STL<'scene>),
    #[cfg(feature = "gl_tf")]
    GLTF(GLTF<'scene>),
    Translate(Translate<'scene>),
    Triangle(Triangle<'scene>),
    Empty(Empty),
    #[cfg(feature = "gl_tf")]
    GLTFTriangle(GLTFTriangle<'scene>),
}

// TODO: remove horrible hack
#[derive(Debug, Clone)]
/// Empty hitable. Cannot be hit. Exists only as an internal workaround.
pub struct Empty {}

impl HitableTrait for Empty {
    fn hit(
        &self,
        _ray: &Ray,
        _distance_min: Float,
        _distance_max: Float,
        _rng: &mut SmallRng,
    ) -> Option<HitRecord> {
        None
    }

    fn bounding_box(&self) -> Option<&AABB> {
        None
    }

    fn pdf_value(
        &self,
        _origin: Position,
        _direction: Direction,
        _wavelength: Wavelength,
        _time: Float,
        _rng: &mut SmallRng,
    ) -> Float {
        0.0
    }
}

#[enum_dispatch]
/// The main trait for entities that can be intersect aka "hit" by a [Ray].
pub trait HitableTrait {
    #[must_use]
    /// The main intersection method.
    fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        rng: &mut SmallRng,
    ) -> Option<HitRecord>;

    #[must_use]
    /// Returns the bounding box of the entity.
    fn bounding_box(&self) -> Option<&AABB>;

    #[must_use]
    /// Probability density function value method, used for multiple importance sampling.
    fn pdf_value(
        &self,
        origin: Position,
        direction: Direction,
        wavelength: Wavelength,
        time: Float,
        rng: &mut SmallRng,
    ) -> Float;

    #[must_use]
    /// Random point on the entity, used for multiple importance sampling.
    fn random(&self, _origin: Position, _rng: &mut SmallRng) -> Displacement {
        unimplemented!(
            "HitableTrait::random called for a Hitable that has no implementation for it!"
        );
    }
}

/// Returns a tuple of `(front_face, normal)`. Used in lieu of `set_face_normal` in the Ray Tracing for the Rest Of Your Life book.
#[must_use]
pub fn get_orientation(ray: &Ray, outward_normal: Direction) -> (bool, Direction) {
    let front_face = ray.direction.dot(&outward_normal) < 0.0;
    let normal = if front_face {
        outward_normal
    } else {
        -outward_normal
    };

    (front_face, normal)
}
