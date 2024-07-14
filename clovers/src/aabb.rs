//! Axis-aligned bounding box.

use core::ops::Add;

use crate::{interval::Interval, ray::Ray, Float, Position, Vec3, EPSILON_RECT_THICKNESS};

/// Axis-aligned bounding box Defined by two opposing corners, each of which are a [Vec3].
///
/// This is useful for creating bounding volume hierarchies, which is an optimization for reducing the time spent on calculating ray-object intersections.
#[derive(Clone, Debug, PartialEq, Default)]
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
    pub fn combine(box0: &AABB, box1: &AABB) -> AABB {
        AABB {
            x: Interval::combine(&box0.x, &box1.x),
            y: Interval::combine(&box0.y, &box1.y),
            z: Interval::combine(&box0.z, &box1.z),
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

    /// Distance of a `Ray` to the bounding box.
    ///
    /// Returns `None` if the `AABB` is not hit, whether it is passed by the ray, or is behind the ray origin considering the ray direction.
    ///
    /// Based on the `IntersectAABB` method described at <https://jacco.ompf2.com/2022/04/18/how-to-build-a-bvh-part-2-faster-rays/>.
    #[allow(clippy::similar_names)]
    #[must_use]
    pub fn distance(&self, ray: &Ray) -> Option<Float> {
        let (box_min, box_max) = self.bounding_positions();
        let (mut tmin, mut tmax);
        let tx1 = (box_min.x - ray.origin.x) / ray.direction.x;
        let tx2 = (box_max.x - ray.origin.x) / ray.direction.x;
        tmin = Float::min(tx1, tx2);
        tmax = Float::max(tx1, tx2);
        let ty1 = (box_min.y - ray.origin.y) / ray.direction.y;
        let ty2 = (box_max.y - ray.origin.y) / ray.direction.y;
        tmin = Float::max(tmin, Float::min(ty1, ty2));
        tmax = Float::min(tmax, Float::max(ty1, ty2));
        let tz1 = (box_min.z - ray.origin.z) / ray.direction.z;
        let tz2 = (box_max.z - ray.origin.z) / ray.direction.z;
        tmin = Float::max(tmin, Float::min(tz1, tz2));
        let tmax = Float::min(tmax, Float::max(tz1, tz2));
        if tmax >= tmin /* && tmin < ray.t */ && tmax > 0.0 {
            return Some(tmin);
        };

        None
    }

    /// Returns the area of this [`AABB`].
    #[must_use]
    pub fn area(&self) -> Float {
        let (min, max) = self.bounding_positions();
        let extent: Vec3 = max - min;
        2.0 * (extent.x * extent.y + extent.y * extent.z + extent.x * extent.z)
    }

    /// Returns the centroid of this [`AABB`].
    #[must_use]
    pub fn centroid(&self) -> Position {
        Position::new(self.x.center(), self.y.center(), self.z.center())
    }
}

impl Add<Vec3> for AABB {
    type Output = AABB;

    fn add(self, offset: Vec3) -> Self::Output {
        AABB::new(self.x + offset.x, self.y + offset.y, self.z + offset.z)
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    #[test]
    fn area_cube() {
        let aabb = AABB::new(
            Interval::new(0.0, 1.0),
            Interval::new(0.0, 1.0),
            Interval::new(0.0, 1.0),
        );
        let area = aabb.area();
        let expected = 6.0;
        assert_eq!(area, expected);
    }

    #[test]
    fn area_cuboid_positive() {
        let aabb = AABB::new(
            Interval::new(0.0, 1.0),
            Interval::new(0.0, 2.0),
            Interval::new(0.0, 3.0),
        );
        let area = aabb.area();
        let expected = 22.0;
        assert_eq!(area, expected);
    }

    #[test]
    fn area_cuboid_negative() {
        let aabb = AABB::new(
            Interval::new(-1.0, 0.0),
            Interval::new(-2.0, 0.0),
            Interval::new(-3.0, 0.0),
        );
        let area = aabb.area();
        let expected = 22.0;
        assert_eq!(area, expected);
    }

    #[test]
    fn centroid() {
        let aabb = AABB::new(
            Interval::new(0.0, 1.0),
            Interval::new(0.0, 1.0),
            Interval::new(0.0, 1.0),
        );
        let centroid = aabb.centroid();
        let expected = Position::new(0.5, 0.5, 0.5);
        assert_eq!(centroid, expected);
    }

    #[test]
    fn default() {
        let aabb = AABB::default();
        let centroid = aabb.centroid();
        let expected = Position::new(0.0, 0.0, 0.0);
        assert_eq!(centroid, expected);
    }
}
