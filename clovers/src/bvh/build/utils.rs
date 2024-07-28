// Internal helper functions

use core::cmp::Ordering;

use crate::{aabb::AABB, hitable::Hitable, hitable::HitableTrait};

pub(crate) fn box_compare(a: &Hitable, b: &Hitable, axis: usize) -> Ordering {
    let box_a: Option<&AABB> = a.aabb();
    let box_b: Option<&AABB> = b.aabb();

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

pub(crate) fn get_comparator(axis: usize) -> fn(&Hitable, &Hitable) -> Ordering {
    let comparators = [box_x_compare, box_y_compare, box_z_compare];
    comparators[axis]
}

// TODO: inefficient, O(n) *and* gets called at every iteration of BVHNode creation => quadratic behavior
#[must_use]
pub(crate) fn vec_bounding_box(vec: &Vec<Hitable>) -> Option<AABB> {
    if vec.is_empty() {
        return None;
    }

    // Mutable AABB that we grow from zero
    let mut output_box: Option<AABB> = None;

    // Go through all the objects, and expand the AABB
    for object in vec {
        // Check if the object has a box
        let Some(bounding) = object.aabb() else {
            // No box found for the object, early return.
            // Having even one unbounded object in a list makes the entire list unbounded!
            return None;
        };

        // Do we have an output_box already saved?
        match output_box {
            // If we do, expand it & recurse
            Some(old_box) => {
                output_box = Some(AABB::combine(&old_box, bounding));
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
