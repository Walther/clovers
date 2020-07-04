use crate::{
    hitable::AABB,
    hitable::{HitRecord, Hitable},
    materials::Material,
    Float, Ray, Vec3, PI,
};
use rand::prelude::*;

pub struct MovingSphere {
    center_0: Vec3,
    center_1: Vec3,
    time_0: Float,
    time_1: Float,
    radius: Float,
    material: Material,
}

impl MovingSphere {
    pub fn new(
        center_0: Vec3,
        center_1: Vec3,
        time_0: Float,
        time_1: Float,
        radius: Float,
        material: Material,
    ) -> Self {
        MovingSphere {
            center_0,
            center_1,
            time_0,
            time_1,
            radius,
            material,
        }
    }
    pub fn center(&self, time: Float) -> Vec3 {
        return self.center_0
            + ((time - self.time_0) / (self.time_1 - self.time_0))
                * (self.center_1 - self.center_0);
    }

    // Returns the U,V surface coordinates of a hitpoint
    pub fn get_uv(&self, hit_position: Vec3, time: Float) -> (Float, Float) {
        let translated: Vec3 = (hit_position - self.center(time)) / self.radius;
        let phi: Float = translated.z.atan2(translated.x);
        let theta: Float = translated.y.asin();
        let u: Float = 1.0 - (phi + PI) / (2.0 * PI);
        let v: Float = (theta + PI / 2.0) / PI;
        (u, v)
    }
}

impl Hitable for MovingSphere {
    fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        _rng: ThreadRng,
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
                let position: Vec3 = ray.point_at_parameter(distance);
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
                let position: Vec3 = ray.point_at_parameter(distance);
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

    // TODO: might need to return Option<T> ?
    fn bounding_box(&self, t0: Float, t1: Float) -> Option<AABB> {
        let box0: AABB = AABB::new(
            self.center(t0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(t0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1: AABB = AABB::new(
            self.center(t1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(t1) + Vec3::new(self.radius, self.radius, self.radius),
        );

        Some(AABB::surrounding_box(box0, box1))
    }
}
