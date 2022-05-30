//! Bounding Volume Hierarchy Node.

use core::cmp::Ordering;

use rand::rngs::SmallRng;

use crate::{
    aabb::AABB,
    hitable::{HitRecord, Hitable, HitableList},
    ray::Ray,
    Box, Float, Vec,
};

/// Bounding Volume Hierarchy Node.
///
/// A node in a tree structure defining a hierarchy of objects in a scene: a node knows its bounding box, and has two children which are also BVHNodes. This is used for accelerating the ray-object intersection calculation in the ray tracer. See [Bounding Volume hierarchies](https://raytracing.github.io/books/RayTracingTheNextWeek.html)
#[derive(Debug, Clone)]
pub struct BVHNode {
    /// Left child of the BVHNode
    pub left: Box<Hitable>,
    /// Right child of the BVHNode
    pub right: Box<Hitable>,
    /// Bounding box containing both of the child nodes
    pub bounding_box: AABB,
}

impl BVHNode {
    /// Create a new BVHNode tree from a given list of [Objects](crate::objects::Object)
    pub fn from_list(
        mut objects: Vec<Hitable>,
        time_0: Float,
        time_1: Float,
        rng: &mut SmallRng,
    ) -> BVHNode {
        // Initialize two child nodes
        let left: Box<Hitable>;
        let right: Box<Hitable>;

        let comparators = [box_x_compare, box_y_compare, box_z_compare];

        // What is the axis with the largest span?
        // TODO: horribly inefficient, improve!
        let hlist: HitableList = objects.clone().into();
        let bounding: AABB = hlist
            .bounding_box(time_0, time_1)
            .expect("No bounding box for objects");
        let spans = [
            bounding.axis(0).size(),
            bounding.axis(1).size(),
            bounding.axis(2).size(),
        ];
        let largest = f32::max(f32::max(spans[0], spans[1]), spans[2]);
        let axis: usize = spans.iter().position(|&x| x == largest).unwrap();
        let comparator = comparators[axis];

        // How many objects do we have?
        let object_span = objects.len();

        if object_span == 1 {
            // If we only have one object, something has gone wrong.
            unreachable!("BVHNode from_list called with one object");
        } else if object_span == 2 {
            // If we are comparing two objects, perform the comparison
            // Insert the child nodes in order
            match comparator(&objects[0], &objects[1]) {
                Ordering::Less => {
                    left = Box::new(objects[0].clone());
                    right = Box::new(objects[1].clone());
                }
                Ordering::Greater => {
                    left = Box::new(objects[1].clone());
                    right = Box::new(objects[0].clone());
                }
                Ordering::Equal => {
                    // TODO: what should happen here?
                    panic!("Equal objects in BVHNode from_list");
                }
            }
        } else if object_span == 3 {
            // Three objects: create one bare object and one BVHNode with two objects
            objects.sort_by(|a, b| comparator(&*a, &*b));
            left = Box::new(objects[0].clone());
            right = Box::new(Hitable::BVHNode(BVHNode {
                left: Box::new(objects[1].clone()),
                right: Box::new(objects[2].clone()),
                bounding_box: AABB::surrounding_box(
                    // TODO: no unwrap?
                    objects[1].bounding_box(time_0, time_1).unwrap(),
                    objects[2].bounding_box(time_0, time_1).unwrap(),
                ),
            }))
        } else {
            // Otherwise, recurse
            objects.sort_by(|a, b| comparator(&*a, &*b));

            // Split the vector; divide and conquer
            let mid = object_span / 2;
            let objects_right = objects.split_off(mid);
            left = Box::new(Hitable::BVHNode(BVHNode::from_list(
                objects, time_0, time_1, rng,
            )));
            right = Box::new(Hitable::BVHNode(BVHNode::from_list(
                objects_right,
                time_0,
                time_1,
                rng,
            )));
        }

        let box_left = left.bounding_box(time_0, time_1);
        let box_right = right.bounding_box(time_0, time_1);

        // Generate a bounding box and BVHNode if possible
        if let (Some(box_left), Some(box_right)) = (box_left, box_right) {
            let bounding_box = AABB::surrounding_box(box_left, box_right);

            BVHNode {
                left,
                right,
                bounding_box,
            }
        } else {
            panic!(
                "No bounding box in bvh_node constructor. {:?} {:?}",
                box_left, box_right
            );
        }
    }

    /// The main `hit` function for a [BVHNode]. Given a [Ray](crate::ray::Ray), and an interval `distance_min` and `distance_max`, returns either `None` or `Some(HitRecord)` based on whether the ray intersects with the encased objects during that interval.
    pub fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        rng: &mut SmallRng,
    ) -> Option<HitRecord> {
        // If we do not hit the bounding box of current node, early return None
        if !self.bounding_box.hit(ray, distance_min, distance_max) {
            return None;
        }

        // Otherwise we have hit the bounding box of this node, recurse to child nodes
        let hit_left = self.left.hit(ray, distance_min, distance_max, rng);
        let hit_right = self.right.hit(ray, distance_min, distance_max, rng);

        // Did we hit neither of the child nodes, one of them, or both?
        // Return the closest thing we hit
        match (&hit_left, &hit_right) {
            (None, None) => None, // In theory, this case should not be reachable
            (None, Some(_)) => hit_right,
            (Some(_), None) => hit_left,
            (Some(left), Some(right)) => {
                if left.distance < right.distance {
                    return hit_left;
                }
                hit_right
            }
        }
    }

    /// Returns the axis-aligned bounding box [AABB] of the objects within this [BVHNode].
    pub fn bounding_box(&self, _t0: Float, _t11: Float) -> Option<AABB> {
        Some(self.bounding_box)
    }
}

fn box_compare(a: &Hitable, b: &Hitable, axis: usize) -> Ordering {
    // TODO: proper time support?
    let box_a: Option<AABB> = a.bounding_box(0.0, 1.0);
    let box_b: Option<AABB> = b.bounding_box(0.0, 1.0);

    if let (Some(box_a), Some(box_b)) = (box_a, box_b) {
        if box_a.axis(axis).min < box_b.axis(axis).min {
            Ordering::Less
        } else {
            // Default to greater, even if equal
            Ordering::Greater
        }
    } else {
        panic!("No bounding box to compare with.")
    }
}

fn box_x_compare(a: &Hitable, b: &Hitable) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Hitable, b: &Hitable) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Hitable, b: &Hitable) -> Ordering {
    box_compare(a, b, 2)
}
