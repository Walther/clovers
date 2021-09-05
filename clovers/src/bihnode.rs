//! Bounding Interval Hierarchy Node.
//!
//! Tree structure for accelerating ray-object intersection checks.
//!
//! Wikipedia: [Bounding Interval Hierarchy](https://en.wikipedia.org/wiki/Bounding_interval_hierarchy)

use rand::{distributions::Standard, prelude::Distribution, rngs::SmallRng, Rng};

use crate::{
    aabb::AABB,
    hitable::{HitRecord, Hitable, HitableList},
    objects::Empty,
    ray::Ray,
    Box, Float, Vec,
};

#[derive(Clone, Copy, Debug, PartialEq)]
/// Axis-aligned infinite planes.
pub enum Axis {
    /// The plane along the XY axis
    XY,
    /// The plane along the XZ axis
    XZ,
    /// The plane along the YZ axis
    YZ,
}

// Helper method for going from a plane axis into an index of a Vec3
impl From<Axis> for usize {
    fn from(a: Axis) -> Self {
        match a {
            Axis::XY => 2,
            Axis::XZ => 1,
            Axis::YZ => 0,
        }
    }
}

// Helper method for enabling rng.gen::<Axis>()
impl Distribution<Axis> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Axis {
        match rng.gen_range(0..=2) {
            0 => Axis::XY,
            1 => Axis::XZ,
            _ => Axis::YZ,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
/// Bounding Interval Hierarchy node.
pub struct BIHNode {
    /// Left child of the BIHNode
    pub left: Box<Hitable>,
    /// Right child of the BIHNode
    pub right: Box<Hitable>,
    /// The axis along which the split of this node was done
    pub axis: Axis,
    // TODO: coordinate system choices?
    /// The maximum coordinate on the selected [Axis] where we have an object; left side
    pub max: Float,
    /// The minimum coordinate on the selected [Axis] where we have an object; right side
    pub min: Float,
}

impl BIHNode {
    /// Create a new BIHNode tree from a given list of [Objects](crate::objects::Object)
    pub fn from_list(
        objects: Vec<Hitable>,
        time_0: Float,
        time_1: Float,
        rng: &mut SmallRng,
    ) -> BIHNode {
        // Initialize two child nodes
        let left: Box<Hitable>;
        let right: Box<Hitable>;

        // TODO: fix temporary hack
        // Temporary: go via HitableList for convenience as methods have been implemented
        let mut hlist: HitableList = objects.clone().into();
        // Calculate an AABB of the current list
        let aabb = match hlist.bounding_box(time_0, time_1) {
            None => {
                // If there's no bounding box, return a leaf
                let empty = Box::new(Hitable::Empty(Empty::new()));
                return BIHNode {
                    left: empty.clone(),
                    right: empty,
                    axis: Axis::XY,
                    max: Float::MIN,
                    min: Float::MAX,
                };
            }
            Some(b) => b,
        };

        // Figure out the longest axis
        let axis = aabb.longest_axis(rng);
        dbg!(&axis);
        // Get the span
        let (min, max, mid) = aabb.min_max_mid(axis);

        // Recursion condition: how many objects were we given?
        if hlist.len() == 1 {
            // If we only have one object, create two child nodes with the same object.
            // We do not have an explicit leaf node in our tree.
            // TODO: this might not be smart, how to improve?
            left = Box::new(objects[0].clone());
            right = Box::new(objects[0].clone());
        } else if hlist.len() == 2 {
            // Split into two by the axis, return a node with plain objects as children
            let (left_list, right_list): (Vec<Hitable>, Vec<Hitable>) =
                hlist.split(axis, mid, time_0, time_1);
            left = Box::new(left_list[0].clone());
            right = Box::new(right_list[0].clone());
        } else {
            // Split into two lists, recurse
            // TODO: HitableList should probably ideally be more vec-like & avoid these conversions
            let (left_list, right_list): (Vec<Hitable>, Vec<Hitable>) =
                hlist.split(axis, mid, time_0, time_1);
            // Recursion time
            left = Box::new(Hitable::BIHNode(BIHNode::from_list(
                left_list, time_0, time_1, rng,
            )));
            right = Box::new(Hitable::BIHNode(BIHNode::from_list(
                right_list, time_0, time_1, rng,
            )));
        }

        BIHNode {
            left,
            right,
            axis,
            min,
            max,
        }
    }

    /// The main `hit` function for a [BIHNode]. Given a [Ray](crate::ray::Ray), and an interval `distance_min` and `distance_max`, returns either `None` or `Some(HitRecord)` based on whether the ray intersects with the encased objects during that interval.
    pub fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        rng: &mut SmallRng,
    ) -> Option<HitRecord> {
        // Fast check: does the ray hit this BIHNode at all?
        if !self.hit_bool(ray, distance_min, distance_max) {
            return None;
        }

        // Does the ray hit the child nodes?
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

    /// Given a [Ray], returns whether the ray hits the [BIHNode] or not. Should be significantly faster than the actual [hit](BIHNode::hit) method.
    pub fn hit_bool(&self, ray: &Ray, mut tmin: Float, mut tmax: Float) -> bool {
        // TODO: borrowed from AABB, is this correct?
        // Axis index, used for accessing the correct value in Vec3
        let a: usize = self.axis.into();
        let invd: Float = 1.0 / ray.direction[a];
        let mut t0: Float = (self.min - ray.origin[a]) * invd;
        let mut t1: Float = (self.max - ray.origin[a]) * invd;
        if invd < 0.0 {
            core::mem::swap(&mut t0, &mut t1);
        }
        tmin = if t0 > tmin { t0 } else { tmin };
        tmax = if t1 < tmax { t1 } else { tmax };
        if tmax <= tmin {
            return false;
        }
        true
    }

    /// Returns the axis-aligned bounding box [AABB] of the objects within this [BIHNode].
    pub fn bounding_box(&self, t0: Float, t1: Float) -> Option<AABB> {
        // TODO: implement a faster aabb constructor based on the intervals; currently extremely naive!
        let l_bb = self.left.bounding_box(t0, t1);
        let r_bb = self.right.bounding_box(t0, t1);
        match (l_bb, r_bb) {
            (None, None) => {
                panic!("Could not create a bounding box for bihnode")
            }
            (None, Some(r)) => Some(r),
            (Some(l), None) => Some(l),
            (Some(l), Some(r)) => Some(l + r),
        }
    }
}
