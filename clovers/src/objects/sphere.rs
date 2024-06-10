//! A sphere object.

use crate::{
    aabb::AABB,
    hitable::{HitRecord, HitableTrait},
    materials::{Material, MaterialInit},
    onb::ONB,
    ray::Ray,
    wavelength::Wavelength,
    Direction, Float, Position, Vec3, EPSILON_SHADOW_ACNE, PI,
};
use nalgebra::Unit;
use rand::{rngs::SmallRng, Rng};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// `SphereInit` structure describes the necessary data for constructing a [Sphere]. Used with [serde] when importing [`SceneFile`](crate::scenes::SceneFile)s.
pub struct SphereInit {
    /// Used for multiple importance sampling
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub priority: bool,
    /// Center of the sphere.
    pub center: Position,
    /// Radius of the sphere.
    pub radius: Float,
    #[cfg_attr(feature = "serde-derive", serde(default))]
    /// Material of the sphere.
    pub material: MaterialInit,
}

#[derive(Debug, Clone)]
/// A sphere object.
pub struct Sphere<'scene> {
    center: Position,
    radius: Float,
    material: &'scene Material,
    aabb: AABB,
}

impl<'scene> Sphere<'scene> {
    /// Creates a new `Sphere` object with the given center, radius and material.
    #[must_use]
    pub fn new(center: Position, radius: Float, material: &'scene Material) -> Self {
        let aabb = AABB::new_from_coords(
            center - Position::new(radius, radius, radius),
            center + Position::new(radius, radius, radius),
        );
        Sphere {
            center,
            radius,
            material,
            aabb,
        }
    }

    /// Returns the U,V surface coordinates of a hitpoint
    #[must_use]
    pub fn get_uv(&self, hit_position: Position, _time: Float) -> (Float, Float) {
        let translated: Position = (hit_position - self.center) / self.radius;
        let phi: Float = translated.z.atan2(translated.x);
        let theta: Float = translated.y.asin();
        let u: Float = 1.0 - (phi + PI) / (2.0 * PI);
        let v: Float = (theta + PI / 2.0) / PI;
        (u, v)
    }
}

impl<'scene> HitableTrait for Sphere<'scene> {
    /// Hit method for the [Sphere] object. Returns a [`HitRecord`] if the given [Ray] intersects with the sphere at the given distance interval.
    #[must_use]
    fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        _rng: &mut SmallRng,
    ) -> Option<HitRecord> {
        let oc: Position = ray.origin - self.center;
        let a: Float = ray.direction.norm_squared();
        let half_b: Float = oc.dot(&ray.direction);
        let c: Float = oc.norm_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let root: Float = discriminant.sqrt();

            // First possible root
            let distance: Float = (-half_b - root) / a;
            if distance < distance_max && distance > distance_min {
                let position: Position = ray.evaluate(distance);
                let outward_normal = (position - self.center) / self.radius;
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
                let position = ray.evaluate(distance);
                let outward_normal = (position - self.center) / self.radius;
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

    /// Returns the axis-aligned bounding box [AABB] for the sphere.
    #[must_use]
    fn bounding_box(&self, _t0: Float, _t1: Float) -> Option<&AABB> {
        Some(&self.aabb)
    }

    /// Returns the probability density function for the sphere? TODO: what does this do again and how
    #[must_use]
    fn pdf_value(
        &self,
        origin: Position,
        direction: Direction,
        wavelength: Wavelength,
        time: Float,
        rng: &mut SmallRng,
    ) -> Float {
        let ray = Ray {
            origin,
            direction,
            time,
            wavelength,
        };
        match self.hit(&ray, EPSILON_SHADOW_ACNE, Float::INFINITY, rng) {
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
    // TODO: improve correctness & optimization!
    /// Utility function from Ray Tracing: The Rest of Your Life.
    #[must_use]
    fn random(&self, origin: Position, rng: &mut SmallRng) -> Position {
        let offset: Position = self.center - origin;
        let distance_squared: Float = offset.norm_squared();
        let uvw = ONB::build_from_w(Unit::new_normalize(offset));
        let vec = random_to_sphere(self.radius, distance_squared, rng);
        let vec = Unit::new_normalize(vec);
        *uvw.local(vec)
    }
}

/// Internal helper.
fn random_to_sphere(radius: Float, distance_squared: Float, rng: &mut SmallRng) -> Vec3 {
    let r1: Float = rng.gen();
    let r2: Float = rng.gen();
    let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * (1.0 - z * z).sqrt();
    let y = phi.sin() * (1.0 - z * z).sqrt();

    Vec3::new(x, y, z)
}
