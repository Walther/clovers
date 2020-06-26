use crate::{hitable::AABB, Float, HitRecord, Hitable, Material, Ray, Vec3, PI};
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
    fn hit(&self, ray: &Ray, distance_min: Float, distance_max: Float) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            // First possible root
            let distance = (-b - discriminant.sqrt()) / a;
            if distance < distance_max && distance > distance_min {
                let position: Vec3 = ray.point_at_parameter(distance);
                let normal = (position - self.center) / self.radius;
                let (u, v) = self.get_uv(position, ray.time);
                return Some(HitRecord {
                    distance,
                    position,
                    normal,
                    u,
                    v,
                    material: Arc::clone(&self.material),
                });
            }
            // Second possible root
            let distance = (-b + discriminant.sqrt()) / a;
            if distance < distance_max && distance > distance_min {
                let position: Vec3 = ray.point_at_parameter(distance);
                let normal = (position - self.center) / self.radius;
                let (u, v) = self.get_uv(position, ray.time);
                return Some(HitRecord {
                    distance,
                    position,
                    normal,
                    u,
                    v,
                    material: Arc::clone(&self.material),
                });
            }
        }
        None
    }

    // TODO: might need to return Option<T> ?
    fn bounding_box(&self, t0: Float, t1: Float) -> Option<AABB> {
        let output_box = AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        );
        Some(output_box)
    }
}
