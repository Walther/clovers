//! A sphere object.

use crate::CloversRng;
use crate::{
    aabb::AABB, hitable::HitRecord, materials::Material, onb::ONB, random::random_to_sphere,
    ray::Ray, Float, Vec3, EPSILON_SHADOW_ACNE, PI,
};

#[cfg(target_arch = "spirv")]
use crate::FloatTrait;

#[derive(Clone, Copy)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// SphereInit structure describes the necessary data for constructing a [Sphere]. Used with [serde] when importing [SceneFiles](crate::scenes::SceneFile).
pub struct SphereInit {
    /// Center of the sphere.
    pub center: Vec3,
    /// Radius of the sphere.
    pub radius: Float,
    #[cfg_attr(feature = "serde-derive", serde(default))]
    /// Material of the sphere.
    pub material: Material,
}

#[derive(Clone, Copy)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// A sphere object.
pub struct Sphere {
    center: Vec3,
    radius: Float,
    material: Material,
    aabb: AABB,
}

impl Sphere {
    /// Creates a new `Sphere` object with the given center, radius and material.
    pub fn new(center: Vec3, radius: Float, material: Material) -> Self {
        let aabb = AABB::new_from_coords(
            center - Vec3::new(radius, radius, radius),
            center + Vec3::new(radius, radius, radius),
        );
        Sphere {
            center,
            radius,
            material,
            aabb,
        }
    }

    /// Returns the U,V surface coordinates of a hitpoint
    pub fn get_uv(&self, hit_position: Vec3, _time: Float) -> (Float, Float) {
        let translated: Vec3 = (hit_position - self.center) / self.radius;
        let phi: Float = translated.z.atan2(translated.x);
        let theta: Float = translated.y.asin();
        let u: Float = 1.0 - (phi + PI) / (2.0 * PI);
        let v: Float = (theta + PI / 2.0) / PI;
        (u, v)
    }

    /// Hit method for the [Sphere] object. Returns a [HitRecord] if the given [Ray] intersects with the sphere at the given distance interval.
    pub fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        _rng: &mut CloversRng,
    ) -> Option<HitRecord> {
        let oc: Vec3 = ray.origin - self.center;
        let a: Float = ray.direction.norm_squared();
        let half_b: Float = oc.dot(&ray.direction);
        let c: Float = oc.norm_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let root: Float = discriminant.sqrt();

            // First possible root
            let distance: Float = (-half_b - root) / a;
            if distance < distance_max && distance > distance_min {
                let position: Vec3 = ray.evaluate(distance);
                let outward_normal = (position - self.center) / self.radius;
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
                let outward_normal = (position - self.center) / self.radius;
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

    /// Returns the axis-aligned bounding box [AABB] for the sphere.
    pub fn bounding_box(&self, _t0: Float, _t1: Float) -> Option<AABB> {
        Some(self.aabb)
    }

    /// Returns the probability density function for the sphere? TODO: what does this do again and how
    pub fn pdf_value(
        &self,
        origin: Vec3,
        vector: Vec3,
        time: Float,
        rng: &mut CloversRng,
    ) -> Float {
        match self.hit(
            &Ray::new(origin, vector, time),
            EPSILON_SHADOW_ACNE,
            Float::INFINITY,
            rng,
        ) {
            None => 0.0,
            Some(_hit_record) => {
                let cos_theta_max = (1.0
                    - self.radius * self.radius / (self.center - origin).norm_squared())
                .sqrt();
                let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);

                1.0 / solid_angle
            }
        }
    }

    // TODO: understand, document
    /// Utility function from Ray Tracing: The Rest of Your Life. TODO: understand, document
    pub fn random(&self, origin: Vec3, rng: &mut CloversRng) -> Vec3 {
        let direction: Vec3 = self.center - origin;
        let distance_squared: Float = direction.norm_squared();
        let uvw = ONB::build_from_w(direction);
        uvw.local(random_to_sphere(self.radius, distance_squared, rng))
    }
}
