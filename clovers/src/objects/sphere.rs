use crate::{
    aabb::AABB,
    hitable::{HitRecord, Hitable},
    materials::Material,
    onb::ONB,
    random::random_to_sphere,
    ray::Ray,
    Float, Vec3, EPSILON_SHADOW_ACNE, PI,
};
use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct SphereInit {
    pub center: Vec3,
    pub radius: Float,
    #[serde(default)]
    pub material: Material,
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub struct Sphere {
    center: Vec3,
    radius: Float,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: Float, material: Material) -> Hitable {
        Hitable::Sphere(Sphere {
            center,
            radius,
            material,
        })
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

    pub fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        _rng: ThreadRng,
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
                let position: Vec3 = ray.point_at_parameter(distance);
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
                let position: Vec3 = ray.point_at_parameter(distance);
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

    pub fn bounding_box(&self, _t0: Float, _t1: Float) -> Option<AABB> {
        let output_box = AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        );
        Some(output_box)
    }

    pub fn pdf_value(&self, origin: Vec3, vector: Vec3, time: Float, rng: ThreadRng) -> Float {
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

    pub fn random(&self, origin: Vec3, rng: ThreadRng) -> Vec3 {
        let direction: Vec3 = self.center - origin;
        let distance_squared: Float = direction.norm_squared();
        let uvw = ONB::build_from_w(direction);
        uvw.local(random_to_sphere(self.radius, distance_squared, rng))
    }
}
