use crate::{
    hitable::{HitRecord, Hitable, AABB},
    materials::Material,
    ray::Ray,
    Float, Vec3, PI,
};
use rand::prelude::*;
use std::sync::Arc;

pub struct Sphere {
    center: Vec3,
    radius: Float,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: Float, material: Arc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }

    // Returns the U,V surface coordinates of a hitpoint
    pub fn get_uv(&self, hit_position: Vec3, _time: Float) -> (Float, Float) {
        let translated: Vec3 = (hit_position - self.center) / self.radius;
        let phi: Float = translated.z.atan2(translated.x);
        let theta: Float = translated.y.asin();
        let u: Float = 1.0 - (phi + PI) / (2.0 * PI);
        let v: Float = (theta + PI / 2.0) / PI;
        (u, v)
    }
}

impl Hitable for Sphere {
    fn hit(
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
            let distance: Float = (-half_b - root) / a;

            // First possible root
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
                    material: Arc::clone(&self.material),
                    front_face: false, // TODO: fix having to declare it before calling face_normal
                };
                record.set_face_normal(ray, outward_normal);
                return Some(record);
            }
            // Second possible root
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
                    material: Arc::clone(&self.material),
                    front_face: false, // TODO: fix having to declare it before calling face_normal
                };
                record.set_face_normal(ray, outward_normal);
                return Some(record);
            }
        }
        None
    }

    // TODO: might need to return Option<T> ?
    fn bounding_box(&self, _t0: Float, _t1: Float) -> Option<AABB> {
        let output_box = AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        );
        Some(output_box)
    }
}
