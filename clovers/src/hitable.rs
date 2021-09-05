//! An abstraction for things that can be hit by [Rays](crate::ray::Ray).

#![allow(missing_docs)] // TODO: Lots of undocumented things for now

use core::cmp::Ordering;

use crate::{
    aabb::AABB,
    bihnode::{Axis, BIHNode},
    bvhnode::BVHNode,
    materials::Material,
    objects::{
        Boxy, ConstantMedium, Empty, FlipFace, MovingSphere, Quad, RotateY, Sphere, Translate,
        Triangle, XYRect, XZRect, YZRect,
    },
    ray::Ray,
    Float, Vec, Vec3,
};
use rand::rngs::SmallRng;
use rand::Rng;

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
#[derive(Clone, Copy, PartialEq)]
pub enum Hitable {
    BIHNode(BIHNode),
    Boxy(Boxy),
    BVHNode(BVHNode),
    ConstantMedium(ConstantMedium),
    Empty(Empty),
    FlipFace(FlipFace),
    HitableList(HitableList),
    MovingSphere(MovingSphere),
    Quad(Quad),
    RotateY(RotateY),
    Sphere(Sphere),
    Translate(Translate),
    Triangle(Triangle),
    XYRect(XYRect),
    XZRect(XZRect),
    YZRect(YZRect),
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
            Hitable::BIHNode(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::Boxy(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::BVHNode(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::ConstantMedium(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::Empty(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::FlipFace(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::HitableList(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::MovingSphere(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::Quad(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::RotateY(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::Sphere(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::Translate(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::Triangle(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::XYRect(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::XZRect(h) => h.hit(ray, distance_min, distance_max, rng),
            Hitable::YZRect(h) => h.hit(ray, distance_min, distance_max, rng),
        }
    }

    pub fn bounding_box(&self, t0: Float, t1: Float) -> Option<AABB> {
        match self {
            Hitable::BIHNode(h) => h.bounding_box(t0, t1),
            Hitable::Boxy(h) => h.bounding_box(t0, t1),
            Hitable::BVHNode(h) => h.bounding_box(t0, t1),
            Hitable::ConstantMedium(h) => h.bounding_box(t0, t1),
            Hitable::Empty(h) => h.bounding_box(t0, t1),
            Hitable::FlipFace(h) => h.bounding_box(t0, t1),
            Hitable::HitableList(h) => h.bounding_box(t0, t1),
            Hitable::MovingSphere(h) => h.bounding_box(t0, t1),
            Hitable::Quad(h) => h.bounding_box(t0, t1),
            Hitable::RotateY(h) => h.bounding_box(t0, t1),
            Hitable::Sphere(h) => h.bounding_box(t0, t1),
            Hitable::Translate(h) => h.bounding_box(t0, t1),
            Hitable::Triangle(h) => h.bounding_box(t0, t1),
            Hitable::XYRect(h) => h.bounding_box(t0, t1),
            Hitable::XZRect(h) => h.bounding_box(t0, t1),
            Hitable::YZRect(h) => h.bounding_box(t0, t1),
        }
    }

    pub fn pdf_value(&self, origin: Vec3, vector: Vec3, time: Float, rng: &mut SmallRng) -> Float {
        match self {
            // TODO: should BIHNode and BVHNode have pdf_value methods?
            // Hitable::BIHNode(h) => h.pdf_value(origin, vector, time, rng),
            Hitable::Boxy(h) => h.pdf_value(origin, vector, time, rng),
            // Hitable::BVHNode(h) => h.pdf_value(origin, vector, time, rng),
            Hitable::Empty(h) => h.pdf_value(origin, vector, time, rng),
            Hitable::HitableList(h) => h.pdf_value(origin, vector, time, rng),
            Hitable::Quad(h) => h.pdf_value(origin, vector, time, rng),
            Hitable::Sphere(h) => h.pdf_value(origin, vector, time, rng),
            Hitable::Triangle(h) => h.pdf_value(origin, vector, time, rng),
            Hitable::XYRect(h) => h.pdf_value(origin, vector, time, rng),
            Hitable::XZRect(h) => h.pdf_value(origin, vector, time, rng),
            Hitable::YZRect(h) => h.pdf_value(origin, vector, time, rng),
            _ => 0.0,
        }
    }

    pub fn random(&self, origin: Vec3, rng: &mut SmallRng) -> Vec3 {
        match self {
            // TODO: should BIHNode and BVHNode have random methods?
            // Hitable::BIHNode(h) => h.random(origin, rng),
            Hitable::Boxy(h) => h.random(origin, rng),
            // Hitable::BVHNode(h) => h.random(origin, rng),
            Hitable::Empty(h) => h.random(origin, rng),
            Hitable::HitableList(h) => h.random(origin, rng),
            Hitable::Quad(h) => h.random(origin, rng),
            Hitable::Sphere(h) => h.random(origin, rng),
            Hitable::Triangle(h) => h.random(origin, rng),
            Hitable::XYRect(h) => h.random(origin, rng),
            Hitable::XZRect(h) => h.random(origin, rng),
            Hitable::YZRect(h) => h.random(origin, rng),
            _ => {
                // TODO: what would be a good default?
                let random: Vec3 =
                    Vec3::new(rng.gen::<Float>(), rng.gen::<Float>(), rng.gen::<Float>());
                random.normalize()
            }
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
#[derive(Clone, Debug, PartialEq)]
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
        BVHNode::from_list(self.0, time_0, time_1, rng)
    }

    pub fn into_bih(self, time_0: f32, time_1: f32, rng: &mut SmallRng) -> BIHNode {
        BIHNode::from_list(self.0, time_0, time_1, rng)
    }

    // TODO: fixme, silly
    pub fn into_hitable(self) -> Hitable {
        Hitable::HitableList(self)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Split along the given axis, giving two lists of hitables
    // TODO: consider better split heuristics
    pub fn split(
        &mut self,
        axis: Axis,
        mid: f32,
        time_0: Float,
        time_1: Float,
    ) -> (Vec<Hitable>, Vec<Hitable>) {
        // Allocate Vecs for the split
        let mut left = Vec::new();
        let mut right = Vec::new();
        // Sort objects by the current axis
        self.0.sort_by(|a, b| box_compare(a, b, axis.into()));

        if self.is_empty() {
            panic!("HitableList was empty, cannot split")
        }

        if self.len() == 1 {
            // TODO: what should we do in case of a single-unit list?
            panic!("HitableList had only one object, cannot split")
        }

        if self.len() == 2 {
            // Two items, simple split. Right-handed coordinates, lower coordinate goes to right side.
            right.push(self.0[0].clone());
            left.push(self.0[1].clone());
        } else {
            // Generic part
            // TODO: clean up!

            // First, let's put at least one object in each list
            let first = self.0.remove(0);
            let last = self.0.pop().unwrap();
            right.push(first);
            left.push(last);

            // Then, split the rest
            for hitable in &self.0 {
                // Get the midpoint of the current hitable
                let (_min, _max, h_mid) = hitable
                    .bounding_box(time_0, time_1)
                    .unwrap()
                    .min_max_mid(axis);

                // Compare the hitable midpoint to the given midpoint
                // Minimum coordinates on right side, maximum coordinates on left side
                // TODO: coordinate system choices?
                if h_mid < mid {
                    right.push(hitable.clone())
                } else {
                    left.push(hitable.clone())
                }
            }
        }

        // TODO: this shouldn't be necessary?
        left.sort_by(|a, b| box_compare(a, b, axis.into()));
        right.sort_by(|a, b| box_compare(a, b, axis.into()));

        (left, right)
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

/// TODO: do these make sense? Unify with the [Axis](crate::bihnode::Axis) struct
pub(crate) fn box_compare(a: &Hitable, b: &Hitable, axis: usize) -> Ordering {
    let box_a: Option<AABB> = a.bounding_box(0.0, 0.0);
    let box_b: Option<AABB> = b.bounding_box(0.0, 0.0);

    if let (Some(box_a), Some(box_b)) = (box_a, box_b) {
        if box_a.min[axis] < box_b.min[axis] {
            Ordering::Less
        } else {
            // Default to greater, even if equal
            Ordering::Greater
        }
    } else {
        panic!("No bounding box to compare with.")
    }
}

pub(crate) fn box_x_compare(a: &Hitable, b: &Hitable) -> Ordering {
    box_compare(a, b, 0)
}

pub(crate) fn box_y_compare(a: &Hitable, b: &Hitable) -> Ordering {
    box_compare(a, b, 1)
}

pub(crate) fn box_z_compare(a: &Hitable, b: &Hitable) -> Ordering {
    box_compare(a, b, 2)
}

#[cfg(test)]
mod tests {
    use crate::{
        bihnode::Axis,
        hitable::Hitable,
        materials::{Lambertian, Material},
        objects::Sphere,
        Vec3,
    };

    use super::HitableList;

    #[test]
    fn split_4() {
        let time_0 = 0.0;
        let time_1 = 1.0;
        let mut hlist = HitableList::new();
        let sphere1 = Hitable::Sphere(Sphere::new(
            Vec3::new(11.0, 0.0, 0.0),
            1.0,
            Material::Lambertian(Lambertian::default()),
        ));
        hlist.0.push(sphere1.clone());
        let sphere2 = Hitable::Sphere(Sphere::new(
            Vec3::new(22.0, 0.0, 0.0),
            1.0,
            Material::Lambertian(Lambertian::default()),
        ));
        hlist.0.push(sphere2.clone());
        let sphere3 = Hitable::Sphere(Sphere::new(
            Vec3::new(33.0, 0.0, 0.0),
            1.0,
            Material::Lambertian(Lambertian::default()),
        ));
        hlist.0.push(sphere3.clone());
        let sphere4 = Hitable::Sphere(Sphere::new(
            Vec3::new(44.0, 0.0, 0.0),
            1.0,
            Material::Lambertian(Lambertian::default()),
        ));
        hlist.0.push(sphere4.clone());

        let (l, r) = hlist.split(Axis::YZ, 30.0, time_0, time_1);
        assert!(r[0] == sphere1);
        assert!(r[1] == sphere2);
        assert!(l[0] == sphere3);
        assert!(l[1] == sphere4);
    }
}
