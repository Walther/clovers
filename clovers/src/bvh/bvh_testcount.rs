use crate::{hitable::Hitable, ray::Ray, Float};

use super::BVHNode;

impl<'scene> BVHNode<'scene> {
    /// Alternate hit method that maintains a test count for the BVH traversals.
    pub fn bvh_testcount(
        &'scene self,
        depth: &mut usize,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
    ) {
        *depth += 1;
        if !self.bounding_box.hit(ray, distance_min, distance_max) {
            return;
        }

        Self::bvh_testcount_recurse_condition(&self.left, depth, ray, distance_min, distance_max);
        Self::bvh_testcount_recurse_condition(&self.right, depth, ray, distance_min, distance_max);
    }

    fn bvh_testcount_recurse_condition(
        bvhnode: &'scene Hitable, // BVHNode
        depth: &mut usize,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
    ) {
        match bvhnode {
            Hitable::BVHNode(bvh) => bvh.bvh_testcount(depth, ray, distance_min, distance_max),
            Hitable::STL(s) => s
                .bvhnode
                .bvh_testcount(depth, ray, distance_min, distance_max),
            Hitable::GLTF(g) => g
                .bvhnode
                .bvh_testcount(depth, ray, distance_min, distance_max),
            _ => (),
        }
    }
}
