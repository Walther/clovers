//! Bounding Volume Hierarchy Node.

use std::{cmp::Ordering, sync::Arc};

use rand::prelude::*;

use crate::{
    aabb::AABB,
    hitable::{HitRecord, Hitable},
    ray::Ray,
    Float,
};

/// Bounding Volume Hierarchy Node. A node in a tree structure defining a hierarchy of objects in a scene: a node knows its bounding box, and has two children which are also BVHNodes. This is used for accelerating the ray-object intersection calculation in the ray tracer. See [Bounding Volume hierarchies](https://raytracing.github.io/books/RayTracingTheNextWeek.html)
#[derive(Debug)]
pub struct BVHNode {
    left: Arc<Hitable>,
    right: Arc<Hitable>,
    bounding_box: AABB,
}

impl BVHNode {
    /// Create a new BVHNode tree from a given list of [Objects](crate::objects::Object)
    pub fn from_list(
        mut objects: Vec<Arc<Hitable>>,
        time_0: Float,
        time_1: Float,
        mut rng: ThreadRng,
    ) -> BVHNode {
        {
            let axis: usize = rng.gen_range(0, 2);
            let comparators = [box_x_compare, box_y_compare, box_z_compare];
            let comparator = comparators[axis];

            let object_span = objects.len();

            let left: Arc<Hitable>;
            let right: Arc<Hitable>;

            if object_span == 1 {
                // If we only have one object, return itself. Note: no explicit leaf type in our tree
                left = objects[0].clone();
                right = objects[0].clone();
            } else if object_span == 2 {
                // If we are comparing two objects, perform the comparison
                match comparator(&objects[0], &objects[1]) {
                    Ordering::Less => {
                        left = objects[0].clone();
                        right = objects[1].clone();
                    }
                    Ordering::Greater => {
                        left = objects[1].clone();
                        right = objects[0].clone();
                    }
                    Ordering::Equal => {
                        // TODO: what should happen here?
                        panic!("Equal objects in BVHNode from_list");
                    }
                }
            } else {
                // Otherwise, recurse
                objects.sort_by(|a, b| comparator(&*a, &*b));

                // Split the vector; divide and conquer
                let mid = object_span / 2;
                let objects_right = objects.split_off(mid);
                left = Arc::new(Hitable::BVHNode(BVHNode::from_list(
                    objects, time_0, time_1, rng,
                )));
                right = Arc::new(Hitable::BVHNode(BVHNode::from_list(
                    objects_right,
                    time_0,
                    time_1,
                    rng,
                )));
            }

            let box_left = left.bounding_box(time_0, time_1);
            let box_right = right.bounding_box(time_0, time_1);

            if box_left.is_none() || box_right.is_none() {
                panic!("No bounding box in bvh_node constructor");
            } else {
                let bounding_box = AABB::surrounding_box(box_left.unwrap(), box_right.unwrap());

                BVHNode {
                    left,
                    right,
                    bounding_box,
                }
            }
        }
    }

    pub fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        rng: ThreadRng,
    ) -> Option<HitRecord> {
        match self.bounding_box.hit(&ray, distance_min, distance_max) {
            false => None,
            true => {
                let hit_left = self.left.hit(&ray, distance_min, distance_max, rng);
                let hit_right = self.right.hit(&ray, distance_min, distance_max, rng);

                match &hit_left {
                    Some(left) => {
                        match &hit_right {
                            // Both hit
                            Some(right) => {
                                if left.distance < right.distance {
                                    hit_left
                                } else {
                                    hit_right
                                }
                            }
                            // Left hit
                            None => hit_left,
                        }
                    }
                    None => match &hit_right {
                        // Right hit
                        Some(_right) => hit_right,
                        // Neither hit
                        None => None,
                    },
                }
            }
        }
    }
    pub fn bounding_box(&self, _t0: Float, _t11: Float) -> Option<AABB> {
        Some(self.bounding_box)
    }
}

fn box_compare(a: &Hitable, b: &Hitable, axis: usize) -> Ordering {
    let box_a: Option<AABB> = a.bounding_box(0.0, 0.0);
    let box_b: Option<AABB> = b.bounding_box(0.0, 0.0);

    if box_a.is_none() || box_b.is_none() {
        panic!("No bounding box in BVHNode constructor.")
    } else if box_a.unwrap().min[axis] < box_b.unwrap().min[axis] {
        Ordering::Less
    } else {
        // Default to greater, even if equal
        Ordering::Greater
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
