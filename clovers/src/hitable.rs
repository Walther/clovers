//! An abstraction for things that can be hit by [Rays](crate::ray::Ray).

#![allow(missing_docs)] // TODO: Lots of undocumented things for now

use crate::{
    aabb::AABB,
    bvhnode::BVHNode,
    materials::Material,
    objects::{
        Boxy, ConstantMedium, FlipFace, MovingSphere, Quad, RotateY, Sphere, Translate, Triangle,
    },
    ray::Ray,
    Float, Vec, Vec3,
};
use rand::rngs::SmallRng;
use rand::Rng;

#[cfg(feature = "traces")]
use tracing::info;

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
    HitableList(HitableList),
    MovingSphere(MovingSphere),
    Quad(Quad),
    RotateY(RotateY),
    Sphere(Sphere),
    Translate(Translate),
    Triangle(Triangle),
}

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
            Hitable::HitableList(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::MovingSphere(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::Quad(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::RotateY(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::Sphere(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::Translate(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::Triangle(h) => h.hit(ray, distance_min, distance_max, rng),
        }
    }

    pub fn bounding_box(&self, t0: Float, t1: Float) -> Option<AABB> {
        match self {
            Hitable::Boxy(h) => h.bounding_box(t0, t1),
            Hitable::BVHNode(h) => h.bounding_box(t0, t1),
            Hitable::ConstantMedium(h) => h.bounding_box(t0, t1),
            Hitable::FlipFace(h) => h.bounding_box(t0, t1),
            Hitable::HitableList(h) => h.bounding_box(t0, t1),
            Hitable::MovingSphere(h) => h.bounding_box(t0, t1),
            Hitable::Quad(h) => h.bounding_box(t0, t1),
            Hitable::RotateY(h) => h.bounding_box(t0, t1),
            Hitable::Sphere(h) => h.bounding_box(t0, t1),
            Hitable::Translate(h) => h.bounding_box(t0, t1),
            Hitable::Triangle(h) => h.bounding_box(t0, t1),
        }
    }

    // TODO: does this actually handle all objects?
    pub fn pdf_value(&self, origin: Vec3, vector: Vec3, time: Float, rng: &mut SmallRng) -> Float {
        match self {
            Hitable::Boxy(h) => h.pdf_value(origin, vector, time, rng),
            Hitable::HitableList(h) => h.pdf_value(origin, vector, time, rng),
            Hitable::Quad(h) => h.pdf_value(origin, vector, time, rng),
            Hitable::Sphere(h) => h.pdf_value(origin, vector, time, rng),
            Hitable::Triangle(h) => h.pdf_value(origin, vector, time, rng),
            _ => 0.0,
        }
    }

    // TODO: does this actually handle all objects?
    pub fn random(&self, origin: Vec3, rng: &mut SmallRng) -> Vec3 {
        match self {
            Hitable::Boxy(h) => h.random(origin, rng),
            Hitable::HitableList(h) => h.random(origin, rng),
            Hitable::Quad(h) => h.random(origin, rng),
            Hitable::Sphere(h) => h.random(origin, rng),
            Hitable::Triangle(h) => h.random(origin, rng),
            _ => Vec3::new(rng.gen::<Float>(), rng.gen::<Float>(), rng.gen::<Float>()).normalize(),
        }
    }

    pub fn add(&mut self, object: Hitable) {
        match self {
            Hitable::HitableList(h) => h.add(object),
            _ => panic!("Cannot add to other types of Hitable"),
        }
    }
}

/// Helper struct for storing multiple `Hitable` objects. This list has a `Hitable` implementation too, returning the closest possible hit
#[derive(Debug, Clone)]
pub struct HitableList(pub Vec<Hitable>);

impl From<Vec<Hitable>> for HitableList {
    fn from(v: Vec<Hitable>) -> Self {
        HitableList(v)
    }
}

impl HitableList {
    pub fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        rng: &mut SmallRng,
    ) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest = distance_max;
        // TODO: with more objects, this may become a significant bottleneck?
        for hitable in self.0.iter() {
            if let Some(record) = hitable.hit(ray, distance_min, closest, rng) {
                closest = record.distance;
                hit_record = Some(record);
            }
        }
        hit_record
    }

    pub fn bounding_box(&self, t0: Float, t1: Float) -> Option<AABB> {
        if self.0.is_empty() {
            return None;
        }

        // Mutable AABB that we grow from zero
        let mut output_box: Option<AABB> = None;

        // Go through all the objects, and expand the AABB
        for object in self.0.iter() {
            // Check if the object has a box
            let bounding = match object.bounding_box(t0, t1) {
                // No box found for the object, early return.
                // Having even one unbounded object in a list makes the entire list unbounded!
                None => return None,
                // Box found
                Some(bounding) => bounding,
            };

            // Do we have an output_box already saved?
            match output_box {
                // If we do, expand it & recurse
                Some(old_box) => {
                    output_box = Some(AABB::surrounding_box(old_box, bounding));
                }
                // Otherwise, set output box to be the newly-found box
                None => {
                    output_box = Some(bounding);
                }
            }
        }

        // Return the final combined output_box
        output_box
    }
    pub fn pdf_value(&self, origin: Vec3, vector: Vec3, time: Float, rng: &mut SmallRng) -> Float {
        let weight = 1.0 / self.0.len() as Float;
        let mut sum = 0.0;

        self.0.iter().for_each(|object| {
            sum += weight * object.pdf_value(origin, vector, time, rng);
        });

        sum
    }

    pub fn random(&self, origin: Vec3, rng: &mut SmallRng) -> Vec3 {
        let int_size = self.0.len();
        self.0[rng.gen_range(0..int_size)].random(origin, rng)
    }

    pub fn new() -> HitableList {
        HitableList(Vec::new())
    }

    pub fn add(&mut self, object: Hitable) {
        self.0.push(object);
    }

    pub fn into_bvh(self, time_0: Float, time_1: Float, rng: &mut SmallRng) -> BVHNode {
        #[cfg(feature = "traces")]
        info!("Building the BVHNode tree");
        BVHNode::from_list(self.0, time_0, time_1, rng)
    }

    // TODO: fixme, silly
    pub fn into_hitable(self) -> Hitable {
        Hitable::HitableList(self)
    }
}

impl Default for HitableList {
    fn default() -> Self {
        Self::new()
    }
}

/// Returns a tuple of `(front_face, normal)`. Used in lieu of `set_face_normal` in the Ray Tracing for the Rest Of Your Life book.
pub fn get_orientation(ray: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
    let front_face = ray.direction.dot(&outward_normal) < 0.0;
    let normal = if front_face {
        outward_normal
    } else {
        -outward_normal
    };

    (front_face, normal)
}
