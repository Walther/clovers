use rand::{rngs::SmallRng, Rng};

use crate::{
    aabb::AABB,
    hitable::{HitRecord, Hitable, HitableTrait},
    ray::Ray,
    wavelength::Wavelength,
    Direction, Displacement, Float, Position,
};

use super::BVHNode;

impl<'scene> HitableTrait for BVHNode<'scene> {
    /// The main `hit` function for a [`BVHNode`]. Given a [Ray], and an interval `distance_min` and `distance_max`, returns either `None` or `Some(HitRecord)` based on whether the ray intersects with the encased objects during that interval.
    #[must_use]
    fn hit(
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

        // Check the distance to the bounding boxes
        let (left_aabb_distance, right_aabb_distance) =
            match (self.left.bounding_box(), self.right.bounding_box()) {
                // Early returns, if there's no bounding box
                (None, None) => return None,
                (Some(_l), None) => return self.left.hit(ray, distance_min, distance_max, rng),
                (None, Some(_r)) => return self.right.hit(ray, distance_min, distance_max, rng),
                // If we have bounding boxes, get the distances
                (Some(l), Some(r)) => (l.distance(ray), r.distance(ray)),
            };
        let (_closest_aabb_distance, furthest_aabb_distance) =
            match (left_aabb_distance, right_aabb_distance) {
                // Early return: neither child AABB can be hit with the ray
                (None, None) => return None,
                // Early return: only one child can be hit with the ray
                (Some(_d), None) => return self.left.hit(ray, distance_min, distance_max, rng),
                (None, Some(_d)) => return self.right.hit(ray, distance_min, distance_max, rng),
                // Default case: both children can be hit with the ray, check the distance
                (Some(l), Some(r)) => (Float::min(l, r), Float::max(l, r)),
            };

        // Check the closest first
        let (closest_bvh, furthest_bvh) = if left_aabb_distance < right_aabb_distance {
            (&self.left, &self.right)
        } else {
            (&self.right, &self.left)
        };
        let closest_bvh_hit = closest_bvh.hit(ray, distance_min, distance_max, rng);

        // Is the hit closer than the closest point of the other AABB?
        if let Some(ref hit_record) = closest_bvh_hit {
            if hit_record.distance < furthest_aabb_distance {
                return Some(hit_record.clone());
            }
        }
        // Otherwise, check the other child too
        let furthest_bvh_hit = furthest_bvh.hit(ray, distance_min, distance_max, rng);

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

    /// Returns the axis-aligned bounding box [AABB] of the objects within this [`BVHNode`].
    #[must_use]
    fn bounding_box(&self) -> Option<&AABB> {
        Some(&self.bounding_box)
    }

    /// Returns a probability density function value based on the children
    #[must_use]
    fn pdf_value(
        &self,
        origin: Position,
        direction: Direction,
        wavelength: Wavelength,
        time: Float,
        rng: &mut SmallRng,
    ) -> Float {
        match (&*self.left, &*self.right) {
            (_, Hitable::Empty(_)) => self
                .left
                .pdf_value(origin, direction, wavelength, time, rng),
            (Hitable::Empty(_), _) => self
                .right
                .pdf_value(origin, direction, wavelength, time, rng),
            (_, _) => {
                (self
                    .left
                    .pdf_value(origin, direction, wavelength, time, rng)
                    + self
                        .right
                        .pdf_value(origin, direction, wavelength, time, rng))
                    / 2.0
            }
        }
    }

    // TODO: improve correctness & optimization!
    /// Returns a random point on the surface of one of the children
    #[must_use]
    fn random(&self, origin: Position, rng: &mut SmallRng) -> Displacement {
        match (&*self.left, &*self.right) {
            (_, Hitable::Empty(_)) => self.left.random(origin, rng),
            (Hitable::Empty(_), _) => self.right.random(origin, rng),
            (_, _) => {
                if rng.gen::<bool>() {
                    self.left.random(origin, rng)
                } else {
                    self.right.random(origin, rng)
                }
            }
        }
    }
}
