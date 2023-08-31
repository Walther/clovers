//! Bounding Volume Hierarchy Node.

use core::cmp::Ordering;

use rand::{rngs::SmallRng, Rng};

use crate::{
    aabb::AABB,
    hitable::{Empty, HitRecord, Hitable, HitableTrait},
    ray::Ray,
    Box, Float, Vec, Vec3,
};

/// Bounding Volume Hierarchy Node.
///
/// A node in a tree structure defining a hierarchy of objects in a scene: a node knows its bounding box, and has two children which are also `BVHNode`s. This is used for accelerating the ray-object intersection calculation in the ray tracer. See [Bounding Volume hierarchies](https://raytracing.github.io/books/RayTracingTheNextWeek.html)
#[derive(Debug, Clone)]
pub struct BVHNode<'scene> {
    /// Left child of the BVHNode
    pub left: Box<Hitable<'scene>>,
    /// Right child of the BVHNode
    pub right: Box<Hitable<'scene>>,
    /// Bounding box containing both of the child nodes
    pub bounding_box: AABB,
}

impl<'scene> BVHNode<'scene> {
    /// Create a new `BVHNode` tree from a given list of [Object](crate::objects::Object)s
    #[must_use]
    pub fn from_list(mut objects: Vec<Hitable>, time_0: Float, time_1: Float) -> BVHNode {
        // Initialize two child nodes
        let left: Box<Hitable>;
        let right: Box<Hitable>;

        let comparators = [box_x_compare, box_y_compare, box_z_compare];

        // What is the axis with the largest span?
        // TODO: horribly inefficient, improve!
        let bounding: AABB =
            vec_bounding_box(&objects, time_0, time_1).expect("No bounding box for objects");
        let spans = [
            bounding.axis(0).size(),
            bounding.axis(1).size(),
            bounding.axis(2).size(),
        ];
        let largest = f32::max(f32::max(spans[0], spans[1]), spans[2]);
        #[allow(clippy::float_cmp)] // TODO: better code for picking the largest axis...
        let axis: usize = spans.iter().position(|&x| x == largest).unwrap();
        let comparator = comparators[axis];

        // How many objects do we have?
        let object_span = objects.len();

        if object_span == 1 {
            // If we only have one object, add one and an empty object.
            // TODO: can this hack be removed?
            left = Box::new(objects[0].clone());
            right = Box::new(Hitable::Empty(Empty {}));
            let bounding_box = left.bounding_box(time_0, time_1).unwrap().clone(); // TODO: remove unwrap
            return BVHNode {
                left,
                right,
                bounding_box,
            };
        } else if object_span == 2 {
            // If we are comparing two objects, perform the comparison
            // Insert the child nodes in order
            match comparator(&objects[0], &objects[1]) {
                Ordering::Less => {
                    left = Box::new(objects[0].clone());
                    right = Box::new(objects[1].clone());
                }
                Ordering::Greater => {
                    left = Box::new(objects[1].clone());
                    right = Box::new(objects[0].clone());
                }
                Ordering::Equal => {
                    // TODO: what should happen here?
                    panic!("Equal objects in BVHNode from_list");
                }
            }
        } else if object_span == 3 {
            // Three objects: create one bare object and one BVHNode with two objects
            objects.sort_by(comparator);
            left = Box::new(objects[0].clone());
            right = Box::new(Hitable::BVHNode(BVHNode {
                left: Box::new(objects[1].clone()),
                right: Box::new(objects[2].clone()),
                bounding_box: AABB::surrounding_box(
                    // TODO: no unwrap?
                    objects[1].bounding_box(time_0, time_1).unwrap(),
                    objects[2].bounding_box(time_0, time_1).unwrap(),
                ),
            }));
        } else {
            // Otherwise, recurse
            objects.sort_by(comparator);

            // Split the vector; divide and conquer
            let mid = object_span / 2;
            let objects_right = objects.split_off(mid);
            left = Box::new(Hitable::BVHNode(BVHNode::from_list(
                objects, time_0, time_1,
            )));
            right = Box::new(Hitable::BVHNode(BVHNode::from_list(
                objects_right,
                time_0,
                time_1,
            )));
        }

        let box_left = left.bounding_box(time_0, time_1);
        let box_right = right.bounding_box(time_0, time_1);

        // Generate a bounding box and BVHNode if possible
        if let (Some(box_left), Some(box_right)) = (box_left, box_right) {
            let bounding_box = AABB::surrounding_box(box_left, box_right);

            BVHNode {
                left,
                right,
                bounding_box,
            }
        } else {
            panic!("No bounding box in bvh_node constructor");
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

impl<'scene> HitableTrait for BVHNode<'scene> {
    /// The main `hit` function for a [`BVHNode`]. Given a [Ray](crate::ray::Ray), and an interval `distance_min` and `distance_max`, returns either `None` or `Some(HitRecord)` based on whether the ray intersects with the encased objects during that interval.
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

    /// Returns the axis-aligned bounding box [AABB] of the objects within this [`BVHNode`].
    #[must_use]
    fn bounding_box(&self, _t0: Float, _t11: Float) -> Option<&AABB> {
        Some(&self.bounding_box)
    }

    /// Returns a probability density function value based on the children
    #[must_use]
    fn pdf_value(&self, origin: Vec3, vector: Vec3, time: f32, rng: &mut SmallRng) -> f32 {
        match (&*self.left, &*self.right) {
            (_, Hitable::Empty(_)) => self.left.pdf_value(origin, vector, time, rng),
            (Hitable::Empty(_), _) => self.right.pdf_value(origin, vector, time, rng),
            (_, _) => {
                (self.left.pdf_value(origin, vector, time, rng)
                    + self.right.pdf_value(origin, vector, time, rng))
                    / 2.0
            }
        }
    }

    /// Returns a random point on the surface of one of the children
    #[must_use]
    fn random(&self, origin: Vec3, rng: &mut SmallRng) -> Vec3 {
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

fn box_compare(a: &Hitable, b: &Hitable, axis: usize) -> Ordering {
    // TODO: proper time support?
    let box_a: Option<&AABB> = a.bounding_box(0.0, 1.0);
    let box_b: Option<&AABB> = b.bounding_box(0.0, 1.0);

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
fn vec_bounding_box(vec: &Vec<Hitable>, t0: Float, t1: Float) -> Option<AABB> {
    if vec.is_empty() {
        return None;
    }

    // Mutable AABB that we grow from zero
    let mut output_box: Option<AABB> = None;

    // Go through all the objects, and expand the AABB
    for object in vec {
        // Check if the object has a box
        let Some(bounding) = object.bounding_box(t0, t1) else {
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
