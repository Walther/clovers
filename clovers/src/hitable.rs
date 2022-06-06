//! An abstraction for things that can be hit by [Rays](crate::ray::Ray).

#![allow(missing_docs)] // TODO: Lots of undocumented things for now

use crate::{
    aabb::AABB,
    bvhnode::BVHNode,
    materials::Material,
    objects::{
        Boxy, ConstantMedium, FlipFace, MovingSphere, Quad, RotateY, Sphere, Translate, Triangle,
        STL,
    },
    random::random_in_unit_sphere,
    ray::Ray,
    Float, Vec3,
};
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
///
/// TODO: ideally, for cleaner abstraction, this could be a Trait. However, the performance implications might need deeper investigation and consideration...
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
    STL(STL),
    Translate(Translate),
    Triangle(Triangle),
    Empty(Empty),
}

// TODO: remove horrible hack
#[derive(Debug, Clone)]
pub struct Empty {}

impl Hitable {
    pub fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        rng: &mut SmallRng,
    ) -> Option<HitRecord> {
        match self {
            Hitable::Boxy(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::BVHNode(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::ConstantMedium(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::FlipFace(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::MovingSphere(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::Quad(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::RotateY(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::Sphere(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::STL(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::Translate(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::Triangle(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::Empty(_) => None,
        }
    }

    #[must_use]
    pub fn bounding_box(&self, t0: Float, t1: Float) -> Option<AABB> {
        match self {
            Hitable::Boxy(h) => h.bounding_box(t0, t1),
            Hitable::BVHNode(h) => h.bounding_box(t0, t1),
            Hitable::ConstantMedium(h) => h.bounding_box(t0, t1),
            Hitable::FlipFace(h) => h.bounding_box(t0, t1),
            Hitable::MovingSphere(h) => h.bounding_box(t0, t1),
            Hitable::Quad(h) => h.bounding_box(t0, t1),
            Hitable::RotateY(h) => h.bounding_box(t0, t1),
            Hitable::Sphere(h) => h.bounding_box(t0, t1),
            Hitable::STL(h) => h.bounding_box(t0, t1),
            Hitable::Translate(h) => h.bounding_box(t0, t1),
            Hitable::Triangle(h) => h.bounding_box(t0, t1),
            Hitable::Empty(_) => None,
        }
    }

    // TODO: handle all objects
    pub fn pdf_value(&self, origin: Vec3, vector: Vec3, time: Float, rng: &mut SmallRng) -> Float {
        match self {
            Hitable::Boxy(h) => h.pdf_value(origin, vector, time, rng),
            Hitable::BVHNode(h) => h.pdf_value(origin, vector, time, rng),
            Hitable::Quad(h) => h.pdf_value(origin, vector, time, rng),
            Hitable::Sphere(h) => h.pdf_value(origin, vector, time, rng),
            Hitable::Triangle(h) => h.pdf_value(origin, vector, time, rng),
            _ => 0.0,
        }
    }

    // TODO: handle all objects
    pub fn random(&self, origin: Vec3, rng: &mut SmallRng) -> Vec3 {
        match self {
            Hitable::Boxy(h) => h.random(origin, rng),
            Hitable::BVHNode(h) => h.random(origin, rng),
            Hitable::Quad(h) => h.random(origin, rng),
            Hitable::Sphere(h) => h.random(origin, rng),
            Hitable::Triangle(h) => h.random(origin, rng),
            _ => random_in_unit_sphere(rng), // TODO: remove temp hack >:(
        }
    }
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
