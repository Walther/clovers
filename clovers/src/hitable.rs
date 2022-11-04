//! An abstraction for things that can be hit by [Rays](crate::ray::Ray).

#![allow(missing_docs)] // TODO: Lots of undocumented things for now

#[cfg(feature = "stl")]
use crate::objects::STL;
#[cfg(feature = "gl_tf")]
use crate::objects::{GLTFTriangle, GLTF};

use crate::{
    aabb::AABB,
    bvhnode::BVHNode,
    materials::Material,
    objects::{
        Boxy, ConstantMedium, FlipFace, MovingSphere, Quad, RotateY, Sphere, Translate, Triangle,
    },
    random::random_in_unit_sphere,
    ray::Ray,
    Float, Vec3,
};

use enum_dispatch::enum_dispatch;
use rand::rngs::SmallRng;

/// Represents a ray-object intersection, with plenty of data about the intersection.
#[derive(Debug)]
pub struct HitRecord<'a> {
    /// Distance from the ray origin to the hitpoint
    pub distance: Float,
    /// 3D coordinate of the hitpoint
    pub position: Vec3,
    /// Surface normal from the hitpoint
    pub normal: Vec3,
    /// U surface coordinate of the hitpoint
    pub u: Float,
    /// V surface coordinate of the hitpoint
    pub v: Float,
    /// Reference to the material at the hitpoint
    pub material: &'a Material,
    /// Is the hitpoint at the front of the surface
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    /// Helper function for getting normals pointing at the correct direction. TODO: consider removal?
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(&outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}

/// An abstraction for things that can be hit by [Rays](crate::ray::Ray).
#[enum_dispatch(HitableTrait)]
#[derive(Debug, Clone)]
pub enum Hitable {
    Boxy(Boxy),
    BVHNode(BVHNode),
    ConstantMedium(ConstantMedium),
    FlipFace(FlipFace),
    MovingSphere(MovingSphere),
    Quad(Quad),
    RotateY(RotateY),
    Sphere(Sphere),
    #[cfg(feature = "stl")]
    STL(STL),
    #[cfg(feature = "gl_tf")]
    GLTF(GLTF),
    Translate(Translate),
    Triangle(Triangle),
    Empty(Empty),
    #[cfg(feature = "gl_tf")]
    GLTFTriangle(GLTFTriangle),
}

// TODO: remove horrible hack
#[derive(Debug, Clone)]
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

    fn bounding_box(&self, _t0: Float, _t1: Float) -> Option<AABB> {
        None
    }

    fn pdf_value(&self, _origin: Vec3, _vector: Vec3, _time: Float, _rng: &mut SmallRng) -> Float {
        0.0
    }

    fn random(&self, _origin: Vec3, rng: &mut SmallRng) -> Vec3 {
        random_in_unit_sphere(rng)
    }
}

#[enum_dispatch]
pub(crate) trait HitableTrait {
    #[must_use]
    fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        rng: &mut SmallRng,
    ) -> Option<HitRecord>;

    #[must_use]
    fn bounding_box(&self, t0: Float, t1: Float) -> Option<AABB>;

    #[must_use]
    fn pdf_value(&self, origin: Vec3, vector: Vec3, time: Float, rng: &mut SmallRng) -> Float;

    #[must_use]
    fn random(&self, origin: Vec3, rng: &mut SmallRng) -> Vec3;
}

/// Returns a tuple of `(front_face, normal)`. Used in lieu of `set_face_normal` in the Ray Tracing for the Rest Of Your Life book.
#[must_use]
pub fn get_orientation(ray: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
    let front_face = ray.direction.dot(&outward_normal) < 0.0;
    let normal = if front_face {
        outward_normal
    } else {
        -outward_normal
    };

    (front_face, normal)
}
