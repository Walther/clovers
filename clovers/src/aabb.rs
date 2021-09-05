//! Axis-aligned bounding box.

use core::ops::Add;

use crate::{bihnode::Axis, ray::Ray, Float, Vec3, EPSILON_RECT_THICKNESS};

/// Axis-aligned bounding box Defined by two opposing corners, each of which are a [Vec3].
///
/// This is useful for creating bounding volume hierarchies, which is an optimization for reducing the time spent on calculating ray-object intersections.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
pub struct AABB {
    /// First corner of the axis-aligned bounding box.
    pub min: Vec3,
    /// Second, opposing corner of the axis-aligned bounding box.
    pub max: Vec3,
}

impl AABB {
    /// Creates a new axis-aligned bounding box from two coordinates.
    pub fn new(min: Vec3, max: Vec3) -> AABB {
        AABB { min, max }
    }

    /// Given a [Ray], returns whether the ray hits the bounding box or not. Based on ["An Optimized AABB Hit Method"](https://raytracing.github.io/books/RayTracingTheNextWeek.html)
    pub fn hit(&self, ray: &Ray, mut tmin: Float, mut tmax: Float) -> bool {
        for a in 0..3 {
            let invd = 1.0 / ray.direction[a];
            let mut t0: Float = (self.min[a] - ray.origin[a]) * invd;
            let mut t1: Float = (self.max[a] - ray.origin[a]) * invd;
            if invd < 0.0 {
                core::mem::swap(&mut t0, &mut t1);
            }
            tmin = if t0 > tmin { t0 } else { tmin };
            tmax = if t1 < tmax { t1 } else { tmax };
            if tmax <= tmin {
                return false;
            }
        }
        true
    }

    /// Given two axis-aligned bounding boxes, return a new [AABB] that contains both.
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

    /// Slightly increases the AABB size to make sure we don't have a zero-thickness one
    pub fn pad(&mut self) {
        // TODO: improved padding function
        self.min = Vec3::new(
            self.min.x - EPSILON_RECT_THICKNESS,
            self.min.y - EPSILON_RECT_THICKNESS,
            self.min.z - EPSILON_RECT_THICKNESS,
        );

        self.max = Vec3::new(
            self.max.x + EPSILON_RECT_THICKNESS,
            self.max.y + EPSILON_RECT_THICKNESS,
            self.max.z + EPSILON_RECT_THICKNESS,
        );
    }

    /// Helper method: which axis has the longest span?. Returns the [Axis] plane perpendicular to the line
    pub fn longest_axis(&self) -> Axis {
        let x_width = (self.max.x - self.min.x).abs();
        let y_width = (self.max.y - self.min.y).abs();
        let z_width = (self.max.z - self.min.z).abs();

        // TODO: can this be made cleaner?
        if x_width > y_width && x_width > z_width {
            return Axis::X;
        }

        if y_width > x_width && y_width > z_width {
            return Axis::Y;
        }

        if z_width > x_width && z_width > y_width {
            return Axis::Z;
        }

        // Everything was the same width. What's a good solution here?
        // TODO: better solutions?
        return Axis::X;
    }

    /// Helper method: get the minimum, maximum, and midpoint of the AABB on a given axis to split by. Note that the given [Axis] is a plane perpendicular to the line we are interested in.
    pub fn min_max_mid(&self, axis: Axis) -> (Float, Float, Float) {
        let (min, max, mid): (Float, Float, Float);
        match axis {
            Axis::Z => {
                min = self.min.z;
                max = self.max.z;
                mid = (max + min) / 2.0;
            }
            Axis::Y => {
                min = self.min.y;
                max = self.max.y;
                mid = (max + min) / 2.0;
            }
            Axis::X => {
                min = self.min.x;
                max = self.max.x;
                mid = (max + min) / 2.0;
            }
        }
        (min, max, mid)
    }
}

impl Add<AABB> for AABB {
    type Output = AABB;

    fn add(self, rhs: AABB) -> Self::Output {
        AABB::surrounding_box(self, rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        bihnode::Axis,
        hitable::{Hitable, HitableList},
        materials::{Lambertian, Material},
        objects::{Boxy, Sphere},
        Float, Vec3,
    };

    #[test]
    fn min_max_mid_x_axis() {
        let time_0: Float = 0.0;
        let time_1: Float = 1.0;
        let mut hlist = HitableList::new();
        let sphere1 = Hitable::Sphere(Sphere::new(
            Vec3::new(11.0, 0.0, 0.0),
            1.0,
            Material::Lambertian(Lambertian::default()),
        ));
        hlist.0.push(sphere1);
        let sphere2 = Hitable::Sphere(Sphere::new(
            Vec3::new(19.0, 0.0, 0.0),
            1.0,
            Material::Lambertian(Lambertian::default()),
        ));
        hlist.0.push(sphere2);
        let aabb = hlist.bounding_box(time_0, time_1).unwrap();
        let (min, max, mid) = aabb.min_max_mid(Axis::X);
        assert_eq!(min, 10.0);
        assert_eq!(max, 20.0);
        assert_eq!(mid, 15.0);
    }

    #[test]
    fn min_max_mid_y_axis() {
        let time_0: Float = 0.0;
        let time_1: Float = 1.0;
        let mut hlist = HitableList::new();
        let sphere1 = Hitable::Sphere(Sphere::new(
            Vec3::new(0.0, 11.0, 0.0),
            1.0,
            Material::Lambertian(Lambertian::default()),
        ));
        hlist.0.push(sphere1.clone());
        let sphere2 = Hitable::Sphere(Sphere::new(
            Vec3::new(0.0, 19.0, 0.0),
            1.0,
            Material::Lambertian(Lambertian::default()),
        ));
        hlist.0.push(sphere2.clone());
        let aabb = hlist.bounding_box(time_0, time_1).unwrap();
        let (min, max, mid) = aabb.min_max_mid(Axis::Y);
        assert_eq!(min, 10.0);
        assert_eq!(max, 20.0);
        assert_eq!(mid, 15.0);
    }

    #[test]
    fn min_max_mid_z_axis() {
        let time_0: Float = 0.0;
        let time_1: Float = 1.0;
        let mut hlist = HitableList::new();
        let sphere1 = Hitable::Sphere(Sphere::new(
            Vec3::new(0.0, 0.0, 11.0),
            1.0,
            Material::Lambertian(Lambertian::default()),
        ));
        hlist.0.push(sphere1.clone());
        let sphere2 = Hitable::Sphere(Sphere::new(
            Vec3::new(0.0, 0.0, 19.0),
            1.0,
            Material::Lambertian(Lambertian::default()),
        ));
        hlist.0.push(sphere2.clone());
        let aabb = hlist.bounding_box(time_0, time_1).unwrap();
        let (min, max, mid) = aabb.min_max_mid(Axis::Z);
        assert_eq!(min, 10.0);
        assert_eq!(max, 20.0);
        assert_eq!(mid, 15.0);
    }

    #[test]
    fn min_max_mid_negatives() {
        let time_0: Float = 0.0;
        let time_1: Float = 1.0;
        let mut hlist = HitableList::new();
        let sphere1 = Hitable::Sphere(Sphere::new(
            Vec3::new(-11.0, 0.0, 0.0),
            1.0,
            Material::Lambertian(Lambertian::default()),
        ));
        hlist.0.push(sphere1);
        let sphere2 = Hitable::Sphere(Sphere::new(
            Vec3::new(-19.0, 0.0, 0.0),
            1.0,
            Material::Lambertian(Lambertian::default()),
        ));
        hlist.0.push(sphere2);
        let aabb = hlist.bounding_box(time_0, time_1).unwrap();
        let (min, max, mid) = aabb.min_max_mid(Axis::X);
        assert_eq!(min, -20.0);
        assert_eq!(max, -10.0);
        assert_eq!(mid, -15.0);
    }

    #[test]
    fn longest_axis_x() {
        let time_0 = 0.0;
        let time_1 = 1.0;
        let boxy = Boxy::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(4.0, 1.0, 1.0),
            Material::default(),
        );
        let aabb = boxy.bounding_box(time_0, time_1).unwrap();
        let axis = aabb.longest_axis();
        assert_eq!(axis, Axis::X);
    }

    #[test]
    fn longest_axis_y() {
        let time_0 = 0.0;
        let time_1 = 1.0;
        let boxy = Boxy::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 4.0, 1.0),
            Material::default(),
        );
        let aabb = boxy.bounding_box(time_0, time_1).unwrap();
        let axis = aabb.longest_axis();
        assert_eq!(axis, Axis::Y);
    }

    #[test]
    fn longest_axis_z() {
        let time_0 = 0.0;
        let time_1 = 1.0;
        let boxy = Boxy::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 1.0, 4.0),
            Material::default(),
        );
        let aabb = boxy.bounding_box(time_0, time_1).unwrap();
        let axis = aabb.longest_axis();
        assert_eq!(axis, Axis::Z);
    }

    #[test]
    fn longest_axis_negatives() {
        let time_0 = 0.0;
        let time_1 = 1.0;
        let boxy = Boxy::new(
            Vec3::new(-4.0, -2.0, -2.0),
            Vec3::new(-1.0, -1.0, -1.0),
            Material::default(),
        );
        let aabb = boxy.bounding_box(time_0, time_1).unwrap();
        let axis = aabb.longest_axis();
        assert_eq!(axis, Axis::X);
    }
}
