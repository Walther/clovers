//! A moving sphere object.

use crate::CloversRng;
use crate::{aabb::AABB, hitable::HitRecord, materials::Material, ray::Ray, Float, Vec3, PI};

#[derive(Clone)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// A moving sphere object. This is represented by one `radius`, two center points `center_0` `center_1`, two times `time_0` `time_1`, and a [Material]. Any [Rays](Ray) hitting the object will also have an internal `time` value, which will be used for determining the interpolated position of the sphere at that time. With lots of rays hitting every pixel but at randomized times, we get temporal multiplexing and an approximation of perceived motion blur.
pub struct MovingSphere {
    /// Center point of the sphere at time_0
    pub center_0: Vec3,
    /// Center point of the sphere at time_1
    pub center_1: Vec3,
    /// Time 0
    pub time_0: Float,
    /// Time 1
    pub time_1: Float,
    /// Radius of the sphere
    pub radius: Float,
    /// Material of the sphere
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub material: Material,
    /// Axis-aligned bounding box
    pub aabb: AABB,
}

impl MovingSphere {
    /// Creates a new `MovingSphere` object. See the struct documentation for more information: [MovingSphere].
    pub fn new(
        center_0: Vec3,
        center_1: Vec3,
        time_0: Float,
        time_1: Float,
        radius: Float,
        material: Material,
    ) -> Self {
        let box0: AABB = AABB::new_from_coords(
            center_0 - Vec3::new(radius, radius, radius),
            center_0 + Vec3::new(radius, radius, radius),
        );
        let box1: AABB = AABB::new_from_coords(
            center_1 - Vec3::new(radius, radius, radius),
            center_1 + Vec3::new(radius, radius, radius),
        );

        let aabb = AABB::surrounding_box(box0, box1);

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
    pub fn center(&self, time: Float) -> Vec3 {
        self.center_0
            + ((time - self.time_0) / (self.time_1 - self.time_0)) * (self.center_1 - self.center_0)
    }

    /// Returns the U,V surface coordinates of a hitpoint
    // TODO: verify this is up to date with the sphere get_uv methods and matches a surface coordinate instead of volumetric space cordinate
    pub fn get_uv(&self, hit_position: Vec3, time: Float) -> (Float, Float) {
        let translated: Vec3 = (hit_position - self.center(time)) / self.radius;
        let phi: Float = translated.z.atan2(translated.x);
        let theta: Float = translated.y.asin();
        let u: Float = 1.0 - (phi + PI) / (2.0 * PI);
        let v: Float = (theta + PI / 2.0) / PI;
        (u, v)
    }

    /// Hit method for the [MovingSphere] object. First gets the interpolated center position at the given time, then follows the implementation of [Sphere](crate::objects::Sphere) object's hit method.
    pub fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        _rng: &mut CloversRng,
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
                let position: Vec3 = ray.evaluate(distance);
                let outward_normal = (position - self.center(ray.time)) / self.radius;
                let (u, v) = self.get_uv(position, ray.time);
                let mut record = HitRecord {
                    distance,
                    position,
                    normal: outward_normal,
                    u,
                    v,
                    material: &self.material,
                    front_face: false, // TODO: fix having to declare it before calling face_normal
                };
                record.set_face_normal(ray, outward_normal);
                return Some(record);
            }
            // Second possible root
            let distance: Float = (-half_b + root) / a;
            if distance < distance_max && distance > distance_min {
                let position: Vec3 = ray.evaluate(distance);
                let outward_normal = (position - self.center(ray.time)) / self.radius;
                let (u, v) = self.get_uv(position, ray.time);
                let mut record = HitRecord {
                    distance,
                    position,
                    normal: outward_normal,
                    u,
                    v,
                    material: &self.material,
                    front_face: false, // TODO: fix having to declare it before calling face_normal
                };
                record.set_face_normal(ray, outward_normal);
                return Some(record);
            }
        }
        None
    }

    /// Returns the axis-aligned bounding box of the [MovingSphere] object. This is the maximum possible bounding box of the entire span of the movement of the sphere, calculated from the two center positions and the radius.
    pub fn bounding_box(&self, _t0: Float, _t1: Float) -> Option<AABB> {
        Some(self.aabb)
    }
}
