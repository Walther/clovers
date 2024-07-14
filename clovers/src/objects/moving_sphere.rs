//! A moving sphere object.

use crate::{
    aabb::AABB,
    hitable::HitableTrait,
    materials::{Material, MaterialInit},
    ray::Ray,
    wavelength::Wavelength,
    Direction, Float, HitRecord, Position, PI,
};
use nalgebra::Unit;
use rand::rngs::SmallRng;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// `SphereInit` structure describes the necessary data for constructing a [`Sphere`](super::Sphere).
pub struct MovingSphereInit {
    /// Used for multiple importance sampling
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub priority: bool,
    /// Center point of the sphere at `time_0`
    pub center_0: Position,
    /// Center point of the sphere at `time_1`
    pub center_1: Position,
    /// Radius of the sphere.
    pub radius: Float,
    #[cfg_attr(feature = "serde-derive", serde(default))]
    /// Material of the sphere.
    pub material: MaterialInit,
}

#[derive(Debug, Clone)]
/// A moving sphere object. This is represented by one `radius`, two center points `center_0` `center_1`, two times `time_0` `time_1`, and a [Material]. Any [Rays](Ray) hitting the object will also have an internal `time` value, which will be used for determining the interpolated position of the sphere at that time. With lots of rays hitting every pixel but at randomized times, we get temporal multiplexing and an approximation of perceived motion blur.
pub struct MovingSphere<'scene> {
    /// Center point of the sphere at `time_0`
    pub center_0: Position,
    /// Center point of the sphere at `time_1`
    pub center_1: Position,
    /// Time 0
    pub time_0: Float,
    /// Time 1
    pub time_1: Float,
    /// Radius of the sphere
    pub radius: Float,
    /// Material of the sphere
    pub material: &'scene Material,
    /// Axis-aligned bounding box
    pub aabb: AABB,
}

impl<'scene> MovingSphere<'scene> {
    /// Creates a new `MovingSphere` object. See the struct documentation for more information: [`MovingSphere`].
    #[must_use]
    pub fn new(
        center_0: Position,
        center_1: Position,
        time_0: Float,
        time_1: Float,
        radius: Float,
        material: &'scene Material,
    ) -> Self {
        let box0: AABB = AABB::new_from_coords(
            center_0 - Position::new(radius, radius, radius),
            center_0 + Position::new(radius, radius, radius),
        );
        let box1: AABB = AABB::new_from_coords(
            center_1 - Position::new(radius, radius, radius),
            center_1 + Position::new(radius, radius, radius),
        );

        let aabb = AABB::combine(&box0, &box1);

        MovingSphere {
            center_0,
            center_1,
            time_0,
            time_1,
            radius,
            material,
            aabb,
        }
    }

    /// Returns the interpolated center of the moving sphere at the given time.
    #[must_use]
    pub fn center(&self, time: Float) -> Position {
        self.center_0
            + ((time - self.time_0) / (self.time_1 - self.time_0)) * (self.center_1 - self.center_0)
    }

    /// Returns the U,V surface coordinates of a hitpoint
    // TODO: verify this is up to date with the sphere get_uv methods and matches a surface coordinate instead of volumetric space cordinate
    #[must_use]
    pub fn get_uv(&self, hit_position: Position, time: Float) -> (Float, Float) {
        let translated: Position = (hit_position - self.center(time)) / self.radius;
        let phi: Float = translated.z.atan2(translated.x);
        let theta: Float = translated.y.asin();
        let u: Float = 1.0 - (phi + PI) / (2.0 * PI);
        let v: Float = (theta + PI / 2.0) / PI;
        (u, v)
    }
}

impl<'scene> HitableTrait for MovingSphere<'scene> {
    /// Hit method for the [`MovingSphere`] object. First gets the interpolated center position at the given time, then follows the implementation of [Sphere](crate::objects::Sphere) object's hit method.
    #[must_use]
    fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        _rng: &mut SmallRng,
    ) -> Option<HitRecord> {
        let oc = ray.origin - self.center(ray.time);
        let a: Float = ray.direction.norm_squared();
        let half_b: Float = oc.dot(&ray.direction);
        let c: Float = oc.norm_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let root: Float = discriminant.sqrt();
            let distance: Float = (-half_b - root) / a;
            // First possible root
            if distance < distance_max && distance > distance_min {
                let position: Position = ray.evaluate(distance);
                let outward_normal = (position - self.center(ray.time)) / self.radius;
                let outward_normal = Unit::new_normalize(outward_normal);
                let (u, v) = self.get_uv(position, ray.time);
                let mut record = HitRecord {
                    distance,
                    position,
                    normal: outward_normal,
                    u,
                    v,
                    material: self.material,
                    front_face: false, // TODO: fix having to declare it before calling face_normal
                };
                record.set_face_normal(ray, outward_normal);
                return Some(record);
            }
            // Second possible root
            let distance: Float = (-half_b + root) / a;
            if distance < distance_max && distance > distance_min {
                let position: Position = ray.evaluate(distance);
                let outward_normal = (position - self.center(ray.time)) / self.radius;
                let outward_normal = Unit::new_normalize(outward_normal);
                let (u, v) = self.get_uv(position, ray.time);
                let mut record = HitRecord {
                    distance,
                    position,
                    normal: outward_normal,
                    u,
                    v,
                    material: self.material,
                    front_face: false, // TODO: fix having to declare it before calling face_normal
                };
                record.set_face_normal(ray, outward_normal);
                return Some(record);
            }
        }
        None
    }

    /// Returns the axis-aligned bounding box of the [`MovingSphere`] object. This is the maximum possible bounding box of the entire span of the movement of the sphere, calculated from the two center positions and the radius.
    #[must_use]
    fn bounding_box(&self) -> Option<&AABB> {
        Some(&self.aabb)
    }

    fn pdf_value(
        &self,
        _origin: Position,
        _direction: Direction,
        _wavelength: Wavelength,
        _time: Float,
        _rng: &mut SmallRng,
    ) -> Float {
        // TODO: fix
        0.0
    }

    fn centroid(&self) -> Position {
        // TODO: proper time support
        self.center(0.5)
    }
}
