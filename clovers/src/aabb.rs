//! Axis-aligned bounding box.

use crate::{ray::Ray, Float, Vec3};

/// Axis-aligned bounding box Defined by two opposing corners, each of which are a [Vec3].
///
/// This is useful for creating bounding volume hierarchies, which is an optimization for reducing the time spent on calculating ray-object intersections.
#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    /// Creates a new axis-aligned bounding box from two coordinates.
    pub fn new(min: Vec3, max: Vec3) -> AABB {
        AABB { min, max }
    }

    /// Given a [Ray](crate::ray::Ray), returns whether the ray hits the bounding box or not. Based on ["An Optimized AABB Hit Method"](https://raytracing.github.io/books/RayTracingTheNextWeek.html)
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
        true
    }

    /// Given two axis-aligned bounding boxes, return a new AABB that contains both.
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
