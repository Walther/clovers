use crate::{
    hitable::{face_normal, HitRecord, Hitable, AABB},
    material::Material,
    Float, Vec3, SMOOTHING_EPSILON,
};
use std::sync::Arc;

pub struct XYRect {
    x0: Float,
    x1: Float,
    y0: Float,
    y1: Float,
    k: Float,
    material: Arc<dyn Material>,
}

impl XYRect {
    pub fn new(
        x0: Float,
        x1: Float,
        y0: Float,
        y1: Float,
        k: Float,
        material: Arc<dyn Material>,
    ) -> XYRect {
        XYRect {
            x0,
            x1,
            y0,
            y1,
            k,
            material: Arc::clone(&material),
        }
    }
}

impl Hitable for XYRect {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        distance_min: crate::Float,
        distance_max: crate::Float,
    ) -> Option<HitRecord> {
        let t = (self.k - ray.origin.z) / ray.direction.z;
        if t < distance_min || t > distance_max {
            return None;
        }
        let x = ray.origin.x + t * ray.direction.x;
        let y = ray.origin.y + t * ray.direction.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let u: Float = (x - self.x0) / (self.x1 - self.x0);
        let v: Float = (y - self.y0) / (self.y1 - self.y0);
        let outward_normal: Vec3 = Vec3::new(0.0, 0.0, 1.0);
        let normal: Vec3 = face_normal(ray, outward_normal);
        // rec.mat_ptr = mp; // what? not defined in the tutorial o.o
        let position = ray.point_at_parameter(t);
        return Some(HitRecord {
            distance: t,
            position,
            normal,
            material: Arc::clone(&self.material),
            u,
            v,
        });
    }
    fn bounding_box(&self, t0: crate::Float, t1: crate::Float) -> Option<crate::hitable::AABB> {
        // The bounding box must have non-zero width in each dimension, so pad the Z dimension a small amount.
        let output_box = AABB::new(
            Vec3::new(self.x0, self.y0, self.k - SMOOTHING_EPSILON),
            Vec3::new(self.x1, self.y1, self.k + SMOOTHING_EPSILON),
        );
        Some(output_box)
    }
}
