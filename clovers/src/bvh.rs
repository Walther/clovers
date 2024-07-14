//! Bounding Volume Hierarchy acceleration structures and related utilities.

use std::time::Instant;

use build::{longest_axis_midpoint, surface_area_heuristic};
#[cfg(feature = "tracing")]
use tracing::info;

use crate::{aabb::AABB, hitable::Hitable, Box};

pub(crate) mod build;
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
    pub aabb: AABB,
}

/// The choice of algorithms used for constructing the Bounding Volume Hierarchy tree
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub enum BvhAlgorithm {
    /// Splitting method based on the longest axis of the current `AABB`
    #[default]
    LongestAxis,
    /// Splitting method based on the Surface Area Heuristic.
    ///
    /// Heavily inspired by the wonderful blog series <https://jacco.ompf2.com/2022/04/18/how-to-build-a-bvh-part-2-faster-rays/>.
    Sah,
}

impl<'scene> BVHNode<'scene> {
    /// Create a new `BVHNode` tree from a given list of [Object](crate::objects::Object)s
    #[must_use]
    pub fn from_list(bvh_algorithm: BvhAlgorithm, hitables: Vec<Hitable>) -> BVHNode {
        #[cfg(feature = "tracing")]
        info!(
            "BVH tree build starting for a list of {} hitables",
            hitables.len()
        );
        let start = Instant::now();
        let bvh = match bvh_algorithm {
            BvhAlgorithm::LongestAxis => longest_axis_midpoint(hitables),
            BvhAlgorithm::Sah => surface_area_heuristic(hitables),
        };
        let end = Instant::now();
        let duration = (end - start).as_millis();
        #[cfg(feature = "tracing")]
        info!("BVH tree build done in {duration} ms");
        bvh
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
