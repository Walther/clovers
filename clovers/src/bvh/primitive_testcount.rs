use rand::rngs::SmallRng;

use crate::{hitable::Hitable, ray::Ray, Float};

use super::BVHNode;

impl<'scene> BVHNode<'scene> {
    /// Alternate hit method that maintains a test count for the primitives
    pub fn primitive_testcount(
        &'scene self,
        count: &mut usize,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        rng: &mut SmallRng,
    ) {
        // If we do not hit the bounding box of current node, early return None
        if !self.aabb.hit(ray, distance_min, distance_max) {
            return;
        }

        // Otherwise we have hit the bounding box of this node, recurse to child nodes
        primitive_testcount_recurse_condition(
            &self.left,
            count,
            ray,
            distance_min,
            distance_max,
            rng,
        );

        primitive_testcount_recurse_condition(
            &self.right,
            count,
            ray,
            distance_min,
            distance_max,
            rng,
        );
    }
}

fn primitive_testcount_recurse_condition(
    bvhnode: &Hitable, // BVHNode
    count: &mut usize,
    ray: &Ray,
    distance_min: Float,
    distance_max: Float,
    rng: &mut SmallRng,
) {
    match bvhnode {
        // These recurse
        Hitable::BVHNode(x) => {
            x.primitive_testcount(count, ray, distance_min, distance_max, rng);
        }

        // These are counted
        Hitable::MovingSphere(_)
        | Hitable::Quad(_)
        | Hitable::Sphere(_)
        | Hitable::ConstantMedium(_)
        | Hitable::Triangle(_)
        | Hitable::GLTFTriangle(_)
        | Hitable::RotateY(_)
        | Hitable::Translate(_) => {
            // TODO: currently RotateY and Translate are counted wrong. They may contain more primitives!
            *count += 1;
        }
        Hitable::HitableList(l) => {
            *count += l.hitables.len();
        }
        Hitable::Boxy(_) => {
            *count += 6;
        }
        Hitable::Empty(_) => (),
    }
}
