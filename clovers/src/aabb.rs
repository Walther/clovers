//! Axis-aligned bounding box.

use core::ops::Add;

use crate::{interval::Interval, ray::Ray, Float, Position, Vec3, EPSILON_RECT_THICKNESS};

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
    pub fn new_from_coords(a: Position, b: Position) -> AABB {
        AABB {
            x: Interval::new(a[0].min(b[0]), a[0].max(b[0])),
            y: Interval::new(a[1].min(b[1]), a[1].max(b[1])),
            z: Interval::new(a[2].min(b[2]), a[2].max(b[2])),
        }
    }

    /// The inverse method for `AABB::new`: given an existing constructed `AABB`, returns the minimum coordinate and maximum coordinate on the opposing corners.
    #[must_use]
    pub fn bounding_positions(&self) -> (Position, Position) {
        (
            Position::new(self.x.min, self.y.min, self.z.min),
            Position::new(self.x.max, self.y.max, self.z.max),
        )
    }

    #[allow(clippy::doc_link_with_quotes)]
    /// Given a [Ray], returns whether the ray hits the bounding box or not. Current method based on the "Axis-aligned bounding box class" of the [Raytracing The Next Week book](https://raytracing.github.io/books/RayTracingTheNextWeek.html).
    #[must_use]
    pub fn hit(&self, ray: &Ray, mut tmin: Float, mut tmax: Float) -> bool {
        let ray_origin = ray.origin;
        let ray_dir = ray.direction;

        for axis in 0..3 {
            let ax = self.axis(axis);
            let adinv = 1.0 / ray_dir[axis];

            let t0 = (ax.min - ray_origin[axis]) * adinv;
            let t1 = (ax.max - ray_origin[axis]) * adinv;

            if t0 < t1 {
                if t0 > tmin {
                    tmin = t0;
                };
                if t1 < tmax {
                    tmax = t1;
                };
            } else {
                if t1 > tmin {
                    tmin = t1;
                };
                if t0 < tmax {
                    tmax = t0;
                };
            }

            if tmax <= tmin {
                return false;
            }
        }
        true
    }

    /// Given two axis-aligned bounding boxes, return a new [AABB] that contains both.
    #[must_use]
    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
        AABB {
            x: Interval::new_from_intervals(&box0.x, &box1.x),
            y: Interval::new_from_intervals(&box0.y, &box1.y),
            z: Interval::new_from_intervals(&box0.z, &box1.z),
        }
    }

    /// Make sure we don't have a zero-thickness AABB, padding if necessary.
    pub fn pad(&mut self) {
        // TODO: refactor
        let delta = EPSILON_RECT_THICKNESS;
        let new_x: Interval = if self.x.size() >= delta {
            self.x.clone()
        } else {
            self.x.expand(delta)
        };
        let new_y: Interval = if self.y.size() >= delta {
            self.y.clone()
        } else {
            self.y.expand(delta)
        };
        let new_z: Interval = if self.z.size() >= delta {
            self.z.clone()
        } else {
            self.z.expand(delta)
        };

        *self = AABB::new(new_x, new_y, new_z);
    }

    /// Returns the interval of the given axis.
    // TODO: this api is kind of annoying
    #[must_use]
    pub fn axis(&self, n: usize) -> &Interval {
        match n {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
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
