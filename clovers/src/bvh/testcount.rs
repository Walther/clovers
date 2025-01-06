use rand::rngs::SmallRng;

use crate::{
    hitable::{Hitable, HitableTrait},
    ray::Ray,
    Float, HitRecord,
};

use super::BVHNode;

impl BVHNode<'_> {
    // NOTE: this must be kept in close alignment with the implementation of BVHNode::hit()!
    // TODO: maybe move the statistics counting to the method itself? Measure the impact?
    /// Alternate hit method that maintains a test count for the BVH traversals.
    pub fn testcount(
        &self,
        depth: &mut usize,
        ray: &Ray,
        distance_min: Float,
        mut distance_max: Float,
        rng: &mut SmallRng,
    ) -> Option<HitRecord> {
        *depth += 1;

        // If we do not hit the bounding box of current node, early return None
        if !self.aabb.hit(ray, distance_min, distance_max) {
            return None;
        }

        // Check the distance to the bounding boxes
        let (left_aabb_distance, right_aabb_distance) = match (self.left.aabb(), self.right.aabb())
        {
            // Early returns, if there's no bounding box
            (None, None) => return None,
            (Some(_l), None) => {
                return recurse(&self.left, depth, ray, distance_min, distance_max, rng)
            }
            (None, Some(_r)) => {
                return recurse(&self.right, depth, ray, distance_min, distance_max, rng)
            }
            // If we have bounding boxes, get the distances
            (Some(l), Some(r)) => (l.distance(ray), r.distance(ray)),
        };
        let (_closest_aabb_distance, furthest_aabb_distance) =
            match (left_aabb_distance, right_aabb_distance) {
                // Early return: neither child AABB can be hit with the ray
                (None, None) => return None,
                // Early return: only one child can be hit with the ray
                (Some(_d), None) => {
                    return recurse(&self.left, depth, ray, distance_min, distance_max, rng)
                }
                (None, Some(_d)) => {
                    return recurse(&self.right, depth, ray, distance_min, distance_max, rng)
                }
                // Default case: both children can be hit with the ray, check the distance
                (Some(l), Some(r)) => (Float::min(l, r), Float::max(l, r)),
            };

        // Check the closest first
        let (closest_bvh, furthest_bvh) = if left_aabb_distance < right_aabb_distance {
            (&self.left, &self.right)
        } else {
            (&self.right, &self.left)
        };
        let closest_bvh_hit = recurse(closest_bvh, depth, ray, distance_min, distance_max, rng);

        // Do we hit the closer AABB?
        if let Some(ref hit_record) = closest_bvh_hit {
            // Update distance_max with the distance of confirmed hit
            distance_max = hit_record.distance;
            // Is the hit closer than the closest point of the other AABB?
            if hit_record.distance < furthest_aabb_distance {
                return Some(hit_record.clone());
            }
        }
        // Otherwise, check the other child too
        let furthest_bvh_hit = recurse(furthest_bvh, depth, ray, distance_min, distance_max, rng);

        // Did we hit neither of the child nodes, one of them, or both?
        // Return the closest thing we hit
        match (&closest_bvh_hit, &furthest_bvh_hit) {
            (None, None) => None,
            (None, Some(_)) => furthest_bvh_hit,
            (Some(_), None) => closest_bvh_hit,
            (Some(left), Some(right)) => {
                if left.distance < right.distance {
                    return closest_bvh_hit;
                }
                furthest_bvh_hit
            }
        }
    }
}

fn recurse<'scene>(
    bvhnode: &'scene Hitable, // BVHNode
    depth: &mut usize,
    ray: &Ray,
    distance_min: Float,
    distance_max: Float,
    rng: &mut SmallRng,
) -> Option<HitRecord<'scene>> {
    match bvhnode {
        Hitable::BVHNode(bvh) => bvh.testcount(depth, ray, distance_min, distance_max, rng),
        hitable => hitable.hit(ray, distance_min, distance_max, rng),
    }
}
