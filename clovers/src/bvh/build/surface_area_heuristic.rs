//! Surface Area Heuristic for the BVH tree construction.
//!
//! Heavily inspired by the wonderful blog series <https://jacco.ompf2.com/2022/04/18/how-to-build-a-bvh-part-2-faster-rays/>.

#[cfg(feature = "tracing")]
use tracing::warn;

use crate::{
    aabb::AABB,
    bvh::BVHNode,
    hitable::{Empty, Hitable, HitableList, HitableTrait},
    Float,
};

use super::utils::vec_bounding_box;

/// Heavily inspired by the wonderful blog series <https://jacco.ompf2.com/2022/04/18/how-to-build-a-bvh-part-2-faster-rays/>.
pub fn build(hitables: Vec<Hitable>) -> BVHNode {
    // Initialize two child nodes
    let left: Box<Hitable>;
    let right: Box<Hitable>;

    let aabb = vec_bounding_box(&hitables).unwrap();
    let count = hitables.len();
    let (axis, position) = best_split(&hitables);

    // Possible leaf nodes
    if count == 1 {
        left = Box::new(hitables[0].clone());
        right = Box::new(Hitable::Empty(Empty {}));
        return BVHNode { left, right, aabb };
    } else if count == 2 {
        left = Box::new(hitables[0].clone());
        right = Box::new(hitables[1].clone());
        return BVHNode { left, right, aabb };
    }

    // Split and recurse
    let (hitables_left, hitables_right): (Vec<_>, Vec<_>) = hitables
        .into_iter()
        // NOTE: strict inequality; the object with the centroid at `pos` ends up in the right box
        .partition(|hitable| hitable.centroid()[axis] < position);

    // Avoid infinite recursion
    if hitables_left.is_empty() {
        left = Box::new(Hitable::Empty(Empty {}));
        right = Box::new(Hitable::HitableList(HitableList::new(hitables_right)));
        let aabb = right.aabb().unwrap().clone();

        return BVHNode { left, right, aabb };
    };
    if hitables_right.is_empty() {
        left = Box::new(Hitable::HitableList(HitableList::new(hitables_left)));
        right = Box::new(Hitable::Empty(Empty {}));
        let aabb = left.aabb().unwrap().clone();

        return BVHNode { left, right, aabb };
    };

    left = Box::new(Hitable::BVHNode(build(hitables_left)));
    right = Box::new(Hitable::BVHNode(build(hitables_right)));

    // Generate a bounding box and BVHNode if possible
    let box_left = left.aabb();
    let box_right = right.aabb();
    if let (Some(box_left), Some(box_right)) = (box_left, box_right) {
        let aabb = AABB::combine(box_left, box_right);
        BVHNode { left, right, aabb }
    } else {
        panic!("No bounding box in bvh_node constructor");
    }
}

fn best_split(hitables: &Vec<Hitable>) -> (usize, Float) {
    // TODO: configurable?
    const SPLIT_COUNT: u8 = 8;
    const SPLIT_COUNT_F: Float = SPLIT_COUNT as Float;

    let mut found = false;
    let mut best_axis = 0;
    let mut best_pos = 0.0;
    let mut best_cost = Float::INFINITY;

    for axis in 0..3 {
        // find the splitting bounds based on the centroids of the hitables
        // this is better than using the bounding box of the hitables
        // because the bounding box can be much larger due to the size of the objects
        let mut bounds_min = Float::INFINITY;
        let mut bounds_max = Float::NEG_INFINITY;
        for hitable in hitables {
            bounds_min = Float::min(bounds_min, hitable.centroid()[axis]);
            bounds_max = Float::max(bounds_max, hitable.centroid()[axis]);
        }

        #[allow(clippy::float_cmp)]
        if bounds_min == bounds_max {
            continue;
        };

        let scale = (bounds_max - bounds_min) / SPLIT_COUNT_F;
        for i in 0..SPLIT_COUNT {
            let candidate_pos = bounds_min + Float::from(i) * scale;
            let cost = evaluate_sah(hitables, axis, candidate_pos);
            if cost < best_cost {
                found = true;
                best_pos = candidate_pos;
                best_axis = axis;
                best_cost = cost;
            }
        }
    }

    // TODO: fix this, if possible!
    #[cfg(feature = "tracing")]
    if !found {
        warn!("best_split did not find an improved split, returning defaults!");
    }

    (best_axis, best_pos)
}

fn evaluate_sah(hitables: &Vec<Hitable>, axis: usize, pos: Float) -> Float {
    // determine triangle counts and bounds for this split candidate
    let mut left_box = AABB::default();
    let mut right_box = AABB::default();
    // 2 * 2^64 primitives should be enough
    let mut left_count = 0u64;
    let mut right_count = 0u64;
    for hitable in hitables {
        // NOTE: strict inequality; the object with the centroid at `pos` ends up in the right box
        if hitable.centroid()[axis] < pos {
            left_count += 1;
            left_box = AABB::combine(&left_box, hitable.aabb().unwrap()); // TODO: remove unwrap
        } else {
            right_count += 1;
            right_box = AABB::combine(&right_box, hitable.aabb().unwrap()); // TODO: remove unwrap
        }
    }
    #[allow(clippy::cast_precision_loss)]
    let cost: Float =
        left_count as Float * left_box.area() + right_count as Float * right_box.area();

    cost
}
