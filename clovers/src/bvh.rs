//! Bounding Volume Hierarchy acceleration structures and related utilities.

use core::cmp::Ordering;

use crate::{
    aabb::AABB,
    hitable::{Hitable, HitableTrait},
    Box,
};

mod build;
mod hitable_trait;
mod primitive_testcount;
mod testcount;

/// Bounding Volume Hierarchy Node.
///
/// A node in a tree structure defining a hierarchy of objects in a scene: a node knows its bounding box, and has two children which are also `BVHNode`s. This is used for accelerating the ray-object intersection calculation in the ray tracer. See [Bounding Volume hierarchies](https://raytracing.github.io/books/RayTracingTheNextWeek.html)
#[derive(Debug, Clone)]
pub struct BVHNode<'scene> {
    /// Left child of the `BVHNode`
    pub left: Box<Hitable<'scene>>,
    /// Right child of the `BVHNode`
    pub right: Box<Hitable<'scene>>,
    /// Bounding box containing both of the child nodes
    pub bounding_box: AABB,
}

impl<'scene> BVHNode<'scene> {
    /// Create a new `BVHNode` tree from a given list of [Object](crate::objects::Object)s
    #[must_use]
    pub fn from_list(hitables: Vec<Hitable>) -> BVHNode {
        // TODO: more alternative build algorithms
        build::longest_axis_midpoint(hitables)
    }

    #[must_use]
    /// Returns the count of the nodes in the tree
    pub fn count(&self) -> usize {
        let leftsum = match &*self.left {
            Hitable::BVHNode(b) => b.count(),
            _ => 1,
        };
        let rightsum = match &*self.right {
            Hitable::BVHNode(b) => b.count(),
            _ => 1,
        };

        leftsum + rightsum
    }
}

// Internal helper functions

fn box_compare(a: &Hitable, b: &Hitable, axis: usize) -> Ordering {
    let box_a: Option<&AABB> = a.bounding_box();
    let box_b: Option<&AABB> = b.bounding_box();

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

// TODO: inefficient, O(n) *and* gets called at every iteration of BVHNode creation => quadratic behavior
#[must_use]
fn vec_bounding_box(vec: &Vec<Hitable>) -> Option<AABB> {
    if vec.is_empty() {
        return None;
    }

    // Mutable AABB that we grow from zero
    let mut output_box: Option<AABB> = None;

    // Go through all the objects, and expand the AABB
    for object in vec {
        // Check if the object has a box
        let Some(bounding) = object.bounding_box() else {
            // No box found for the object, early return.
            // Having even one unbounded object in a list makes the entire list unbounded!
            return None;
        };

        // Do we have an output_box already saved?
        match output_box {
            // If we do, expand it & recurse
            Some(old_box) => {
                output_box = Some(AABB::surrounding_box(&old_box, bounding));
            }
            // Otherwise, set output box to be the newly-found box
            None => {
                output_box = Some(bounding.clone());
            }
        }
    }

    // Return the final combined output_box
    output_box
}
