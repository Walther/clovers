//! Axis-aligned bounding box.

use core::ops::Add;

use crate::{interval::Interval, ray::Ray, Float, Vec3, EPSILON_RECT_THICKNESS};

/// Axis-aligned bounding box Defined by two opposing corners, each of which are a [Vec3].
///
/// This is useful for creating bounding volume hierarchies, which is an optimization for reducing the time spent on calculating ray-object intersections.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
pub struct AABB {
    /// The bounding interval on the X axis
    pub x: Interval,
    /// The bounding interval on the Y axis
    pub y: Interval,
    /// The bounding interval on the Z axis
    pub z: Interval,
}

impl AABB {
    /// Creates a new axis-aligned bounding box from three intervals
    #[must_use]
    pub fn new(interval_x: Interval, interval_y: Interval, interval_z: Interval) -> AABB {
        AABB {
            x: interval_x,
            y: interval_y,
            z: interval_z,
        }
    }

    /// Creates a new axis-aligned bounding box from two coordinates. Treats the two points a and b as extrema for the bounding box, so we don't require a particular minimum/maximum coordinate order.
    #[must_use]
    pub fn new_from_coords(a: Vec3, b: Vec3) -> AABB {
        AABB {
            x: Interval::new(a[0].min(b[0]), a[0].max(b[0])),
            y: Interval::new(a[1].min(b[1]), a[1].max(b[1])),
            z: Interval::new(a[2].min(b[2]), a[2].max(b[2])),
        }
    }

    /// Given a [Ray], returns whether the ray hits the bounding box or not. Based on ["An Optimized AABB Hit Method"](https://raytracing.github.io/books/RayTracingTheNextWeek.html)
    #[must_use]
    pub fn hit(&self, ray: &Ray, mut tmin: Float, mut tmax: Float) -> bool {
        // TODO: Create an improved hit method with more robust handling of zeroes. See https://github.com/RayTracing/raytracing.github.io/issues/927
        // Both methods below are susceptible for NaNs and infinities, and have subtly different edge cases.

        // "New method"
        // for axis in 0..3 {
        //     let a = (self.axis(axis).min - ray.origin[axis]) / ray.direction[axis];
        //     let b = (self.axis(axis).max - ray.origin[axis]) / ray.direction[axis];
        //     let t0: Float = a.min(b);
        //     let t1: Float = a.max(b);
        //     tmin = t0.max(tmin);
        //     tmax = t1.min(tmax);
        //     if tmax <= tmin {
        //         return false;
        //     }
        // }

        // "Old method"
        // for axis in 0..3 {
        //     let invd = 1.0 / ray.direction[axis];
        //     let mut t0: Float = (self.axis(axis).min - ray.origin[axis]) * invd;
        //     let mut t1: Float = (self.axis(axis).max - ray.origin[axis]) * invd;
        //     if invd < 0.0 {
        //         core::mem::swap(&mut t0, &mut t1);
        //     }
        //     tmin = if t0 > tmin { t0 } else { tmin };
        //     tmax = if t1 < tmax { t1 } else { tmax };
        //     if tmax <= tmin {
        //         return false;
        //     }
        // }

        // "My adjusted method" - possibly more zero-resistant?
        // TODO: validate
        for axis in 0..3 {
            // If ray direction component is 0, invd becomes infinity.
            // Ignore? False positive hit for aabb is probably better than false negative; the actual object can still be hit more accurately
            let invd = 1.0 / ray.direction[axis];
            if !invd.is_normal() {
                continue;
            }
            // If the value in parenthesis ends up as zero, 0*inf can be NaN
            let mut t0: Float = (self.axis(axis).min - ray.origin[axis]) * invd;
            let mut t1: Float = (self.axis(axis).max - ray.origin[axis]) * invd;
            if !t0.is_normal() || !t1.is_normal() {
                continue;
            }
            if invd < 0.0 {
                core::mem::swap(&mut t0, &mut t1);
            }
            tmin = if t0 > tmin { t0 } else { tmin };
            tmax = if t1 < tmax { t1 } else { tmax };
            if tmax <= tmin {
                return false;
            }
        }

        // If we have not missed on any axis, return true for the hit
        true
    }

    /// Given two axis-aligned bounding boxes, return a new [AABB] that contains both.
    #[must_use]
    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
        AABB {
            x: Interval::new_from_intervals(box0.x, box1.x),
            y: Interval::new_from_intervals(box0.y, box1.y),
            z: Interval::new_from_intervals(box0.z, box1.z),
        }
    }

    /// Make sure we don't have a zero-thickness AABB, padding if necessary.
    pub fn pad(&mut self) {
        // TODO: refactor
        let delta = EPSILON_RECT_THICKNESS;
        let new_x: Interval = if self.x.size() >= delta {
            self.x
        } else {
            self.x.expand(delta)
        };
        let new_y: Interval = if self.y.size() >= delta {
            self.y
        } else {
            self.y.expand(delta)
        };
        let new_z: Interval = if self.z.size() >= delta {
            self.z
        } else {
            self.z.expand(delta)
        };

        *self = AABB::new(new_x, new_y, new_z);
    }

    /// Returns the interval of the given axis.
    // TODO: this api is kind of annoying
    #[must_use]
    pub fn axis(&self, n: usize) -> Interval {
        match n {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("AABB::axis called with invalid parameter: {n:?}"),
        }
    }
}

impl Add<Vec3> for AABB {
    type Output = AABB;

    fn add(self, offset: Vec3) -> Self::Output {
        AABB::new(self.x + offset.x, self.y + offset.y, self.z + offset.z)
    }
}
