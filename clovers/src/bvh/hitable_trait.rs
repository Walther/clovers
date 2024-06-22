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

        // Otherwise we have hit the bounding box of this node, recurse to child nodes
        let hit_left = self.left.hit(ray, distance_min, distance_max, rng);
        let hit_right = self.right.hit(ray, distance_min, distance_max, rng);

        // Did we hit neither of the child nodes, one of them, or both?
        // Return the closest thing we hit
        match (&hit_left, &hit_right) {
            (None, None) => None,
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
