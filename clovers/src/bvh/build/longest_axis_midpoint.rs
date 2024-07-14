use core::cmp::Ordering;

use crate::{
    aabb::AABB,
    bvh::{BVHNode, BvhAlgorithm},
    hitable::{Empty, Hitable, HitableTrait},
};

pub fn build(mut hitables: Vec<Hitable>) -> BVHNode {
    let bvh_algorithm = BvhAlgorithm::LongestAxis;

    // Initialize two child nodes
    let left: Box<Hitable>;
    let right: Box<Hitable>;

    let comparators = [box_x_compare, box_y_compare, box_z_compare];

    // What is the axis with the largest span?
    // TODO: horribly inefficient, improve!
    let bounding: AABB = vec_bounding_box(&hitables).expect("No bounding box for objects");
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
    let object_span = hitables.len();

    if object_span == 1 {
        // If we only have one object, add one and an empty object.
        // TODO: can this hack be removed?
        left = Box::new(hitables[0].clone());
        right = Box::new(Hitable::Empty(Empty {}));
        let aabb = left.bounding_box().unwrap().clone(); // TODO: remove unwrap
        return BVHNode { left, right, aabb };
    } else if object_span == 2 {
        // If we are comparing two objects, perform the comparison
        // Insert the child nodes in order
        match comparator(&hitables[0], &hitables[1]) {
            Ordering::Less => {
                left = Box::new(hitables[0].clone());
                right = Box::new(hitables[1].clone());
            }
            Ordering::Greater => {
                left = Box::new(hitables[1].clone());
                right = Box::new(hitables[0].clone());
            }
            Ordering::Equal => {
                // TODO: what should happen here?
                panic!("Equal objects in BVHNode from_list");
            }
        }
    } else if object_span == 3 {
        // Three objects: create one bare object and one BVHNode with two objects
        hitables.sort_by(comparator);
        left = Box::new(hitables[0].clone());
        right = Box::new(Hitable::BVHNode(BVHNode {
            left: Box::new(hitables[1].clone()),
            right: Box::new(hitables[2].clone()),
            aabb: AABB::surrounding_box(
                // TODO: no unwrap?
                hitables[1].bounding_box().unwrap(),
                hitables[2].bounding_box().unwrap(),
            ),
        }));
    } else {
        // Otherwise, recurse
        hitables.sort_by(comparator);

        // Split the vector; divide and conquer
        let mid = object_span / 2;
        let hitables_right = hitables.split_off(mid);
        left = Box::new(Hitable::BVHNode(BVHNode::from_list(
            bvh_algorithm,
            hitables,
        )));
        right = Box::new(Hitable::BVHNode(BVHNode::from_list(
            bvh_algorithm,
            hitables_right,
        )));
    }

    let box_left = left.bounding_box();
    let box_right = right.bounding_box();

    // Generate a bounding box and BVHNode if possible
    if let (Some(box_left), Some(box_right)) = (box_left, box_right) {
        let aabb = AABB::surrounding_box(box_left, box_right);

        BVHNode { left, right, aabb }
    } else {
        panic!("No bounding box in bvh_node constructor");
    }
}

// Internal helper functions

fn box_compare(a: &Hitable, b: &Hitable, axis: usize) -> Ordering {
    let box_a: Option<&AABB> = a.bounding_box();
    let box_b: Option<&AABB> = b.bounding_box();

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
fn vec_bounding_box(vec: &Vec<Hitable>) -> Option<AABB> {
    if vec.is_empty() {
        return None;
    }

    // Mutable AABB that we grow from zero
    let mut output_box: Option<AABB> = None;

    // Go through all the objects, and expand the AABB
    for object in vec {
        // Check if the object has a box
        let Some(bounding) = object.bounding_box() else {
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
