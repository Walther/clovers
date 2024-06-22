use crate::{
    aabb::AABB,
    bvh::BVHNode,
    hitable::{Empty, Hitable},
};

use super::{
    box_x_compare, box_y_compare, box_z_compare, vec_bounding_box, HitableTrait, Ordering,
};

impl<'scene> BVHNode<'scene> {
    /// Create a new `BVHNode` tree from a given list of [Object](crate::objects::Object)s
    #[must_use]
    pub fn from_list(mut hitables: Vec<Hitable>) -> BVHNode {
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
            let bounding_box = left.bounding_box().unwrap().clone(); // TODO: remove unwrap
            return BVHNode {
                left,
                right,
                bounding_box,
            };
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
                bounding_box: AABB::surrounding_box(
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
            left = Box::new(Hitable::BVHNode(BVHNode::from_list(hitables)));
            right = Box::new(Hitable::BVHNode(BVHNode::from_list(hitables_right)));
        }

        let box_left = left.bounding_box();
        let box_right = right.bounding_box();

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
