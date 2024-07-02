//! Bounding Volume Hierarchy acceleration structures and related utilities.

use crate::{aabb::AABB, hitable::Hitable, Box};

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

/// The choice of algorithms used for constructing the Bounding Volume Hierarchy tree
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BvhAlgorithm {
    /// Splitting method based on the longest axis of the current `AABB`
    LongestAxis,
}

impl<'scene> BVHNode<'scene> {
    /// Create a new `BVHNode` tree from a given list of [Object](crate::objects::Object)s
    #[must_use]
    pub fn from_list(bvh_algorithm: BvhAlgorithm, hitables: Vec<Hitable>) -> BVHNode {
        match bvh_algorithm {
            BvhAlgorithm::LongestAxis => build::longest_axis_midpoint(hitables),
        }
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
