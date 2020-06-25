use crate::{Float, Material, Ray, Vec3};
use std::sync::Arc;

pub struct HitRecord {
    pub distance: Float,
    pub position: Vec3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
}

pub trait Hitable: Sync + Send {
    /// The main function for checking whether an object is hit by a ray. If an object is hit, returns Some(HitRecord)
    fn hit(&self, ray: &Ray, distance_min: Float, distance_max: Float) -> Option<HitRecord>;
    fn bounding_box(&self, t0: Float, t1: Float) -> Option<AABB>;
}

/// Helper struct for storing multiple `Hitable` objects. This list has a `Hitable` implementation too, returning the closest possible hit
pub struct HitableList {
    pub hitables: Vec<Box<dyn Hitable>>,
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, distance_min: Float, distance_max: Float) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest = distance_max;
        for hitable in self.hitables.iter() {
            if let Some(record) = hitable.hit(&ray, distance_min, closest) {
                closest = record.distance;
                hit_record = Some(record);
            }
        }
        hit_record
    }

    fn bounding_box(&self, t0: Float, t1: Float) -> Option<AABB> {
        if self.hitables.is_empty() {
            return None;
        }

        let mut output_box: Option<AABB> = None;

        for object in self.hitables.iter() {
            // Check if the object has a box
            match object.bounding_box(t0, t1) {
                // No box found, early return.
                // Having even one unbounded object in a list makes the entire list unbounded
                None => return None,
                // Box found
                Some(bounding) =>
                // Do we have an output_box already saved?
                {
                    match output_box {
                        // If we do, expand it & recurse
                        Some(old_box) => {
                            output_box = Some(AABB::surrounding_box(old_box, bounding));
                        }
                        // Otherwise, set output box to be the newly-found box
                        None => {
                            output_box = Some(bounding);
                        }
                    }
                }
            }
        }

        return output_box;
    }
}

impl HitableList {
    pub fn new() -> HitableList {
        HitableList {
            hitables: Vec::new(),
        }
    }
    // TODO: figure out this helper
    //     pub fn add(&self, object: dyn Hitable) {
    //         self.hitables.push(Box::new(object));
    //     }
}

#[derive(Clone, Copy)]
pub struct AABB {
    min: Vec3,
    max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> AABB {
        AABB { min, max }
    }

    /// "An Optimized AABB Hit Method" https://raytracing.github.io/books/RayTracingTheNextWeek.html
    pub fn hit(&self, ray: &Ray, mut tmin: Float, mut tmax: Float) -> bool {
        for a in 0..3 {
            let invd = 1.0 / ray.direction[a];
            let mut t0 = (self.min[a] - ray.origin[a]) * invd;
            let mut t1 = (self.max[a] - ray.origin[a]) * invd;
            if invd < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            tmin = if t0 > tmin { t0 } else { tmin };
            tmax = if t1 < tmax { t1 } else { tmax };
            if tmax <= tmin {
                return false;
            }
        }
        return true;
    }

    pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
        let small: Vec3 = Vec3::new(
            (box0.min.x).min(box1.min.x),
            (box0.min.y).min(box1.min.y),
            (box0.min.z).min(box1.min.z),
        );

        let big: Vec3 = Vec3::new(
            (box0.max.x).max(box1.max.x),
            (box0.max.y).max(box1.max.y),
            (box0.max.z).max(box1.max.z),
        );

        AABB::new(small, big)
    }
}

pub struct BVHNode {
    left: Arc<dyn Hitable>,
    right: Arc<dyn Hitable>,
    bounding_box: AABB,
}

impl Hitable for BVHNode {
    fn hit(&self, ray: &Ray, distance_min: Float, distance_max: Float) -> Option<HitRecord> {
        match self.bounding_box.hit(ray, distance_min, distance_max) {
            false => None,
            true => {
                let hit_left = self.left.hit(ray, distance_min, distance_max);
                let hit_right = self.right.hit(ray, distance_min, distance_max);

                match &hit_left {
                    Some(left) => {
                        match &hit_right {
                            // Both hit
                            Some(right) => {
                                if left.distance < right.distance {
                                    return hit_left;
                                } else {
                                    return hit_right;
                                }
                            }
                            // Left hit
                            None => return hit_left,
                        }
                    }
                    None => match &hit_right {
                        // Right hit
                        Some(_right) => return hit_right,
                        // Neither hit
                        None => return None,
                    },
                }
            }
        }
    }
    fn bounding_box(&self, t0: Float, t1: Float) -> Option<AABB> {
        Some(self.bounding_box)
    }
}
