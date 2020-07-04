use crate::{
    hitable::{HitRecord, Hitable, AABB},
    materials::Material,
    ray::Ray,
    Float, Vec3, RECT_EPSILON,
};
use rand::prelude::ThreadRng;
use std::sync::Arc;

// XY

pub struct XYRect {
    x0: Float,
    x1: Float,
    y0: Float,
    y1: Float,
    k: Float,
    material: Material,
}

impl XYRect {
    pub fn new(x0: Float, x1: Float, y0: Float, y1: Float, k: Float, material: Material) -> XYRect {
        XYRect {
            x0,
            x1,
            y0,
            y1,
            k,
            material: material,
        }
    }
}

impl Hitable for XYRect {
    fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        _rng: ThreadRng,
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
        let position = ray.point_at_parameter(t);
        let mut record = HitRecord {
            distance: t,
            position,
            normal: outward_normal,
            material: &self.material,
            u,
            v,
            front_face: false, // TODO: fix having to declare it before calling face_normal
        };
        record.set_face_normal(ray, outward_normal);
        return Some(record);
    }
    fn bounding_box(&self, _t0: crate::Float, _t1: crate::Float) -> Option<crate::hitable::AABB> {
        // The bounding box must have non-zero width in each dimension, so pad the Z dimension a small amount.
        let output_box = AABB::new(
            Vec3::new(self.x0, self.y0, self.k - RECT_EPSILON),
            Vec3::new(self.x1, self.y1, self.k + RECT_EPSILON),
        );
        Some(output_box)
    }
}

// XZ

pub struct XZRect {
    x0: Float,
    x1: Float,
    z0: Float,
    z1: Float,
    k: Float,
    material: Material,
}

impl XZRect {
    pub fn new(x0: Float, x1: Float, z0: Float, z1: Float, k: Float, material: Material) -> XZRect {
        XZRect {
            x0,
            x1,
            z0,
            z1,
            k,
            material: material,
        }
    }
}

impl Hitable for XZRect {
    fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        _rng: ThreadRng,
    ) -> Option<HitRecord> {
        let t = (self.k - ray.origin.y) / ray.direction.y;
        if t < distance_min || t > distance_max {
            return None;
        }
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let u: Float = (x - self.x0) / (self.x1 - self.x0);
        let v: Float = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal: Vec3 = Vec3::new(0.0, 1.0, 0.0);
        let position = ray.point_at_parameter(t);
        let mut record = HitRecord {
            distance: t,
            position,
            normal: outward_normal,
            material: &self.material,
            u,
            v,
            front_face: false, // TODO: fix having to declare it before calling face_normal
        };
        record.set_face_normal(ray, outward_normal);
        return Some(record);
    }
    fn bounding_box(&self, _t0: crate::Float, _t1: crate::Float) -> Option<crate::hitable::AABB> {
        // The bounding box must have non-zero width in each dimension, so pad the Z dimension a small amount.
        let output_box = AABB::new(
            Vec3::new(self.x0, self.k - RECT_EPSILON, self.z0),
            Vec3::new(self.x1, self.k + RECT_EPSILON, self.z1),
        );
        Some(output_box)
    }
}

// YZ

pub struct YZRect {
    y0: Float,
    y1: Float,
    z0: Float,
    z1: Float,
    k: Float,
    material: Material,
}

impl YZRect {
    pub fn new(y0: Float, y1: Float, z0: Float, z1: Float, k: Float, material: Material) -> YZRect {
        YZRect {
            y0,
            y1,
            z0,
            z1,
            k,
            material: material,
        }
    }
}

impl Hitable for YZRect {
    fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        _rng: ThreadRng,
    ) -> Option<HitRecord> {
        let t = (self.k - ray.origin.x) / ray.direction.x;
        if t < distance_min || t > distance_max {
            return None;
        }
        let y = ray.origin.y + t * ray.direction.y;
        let z = ray.origin.z + t * ray.direction.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let u: Float = (y - self.y0) / (self.y1 - self.y0);
        let v: Float = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal: Vec3 = Vec3::new(1.0, 0.0, 0.0);
        let position = ray.point_at_parameter(t);
        let mut record = HitRecord {
            distance: t,
            position,
            normal: outward_normal,
            material: &self.material,
            u,
            v,
            front_face: false, // TODO: fix having to declare it before calling face_normal
        };
        record.set_face_normal(ray, outward_normal);
        return Some(record);
    }
    fn bounding_box(&self, _t0: crate::Float, _t1: crate::Float) -> Option<crate::hitable::AABB> {
        // The bounding box must have non-zero width in each dimension, so pad the Z dimension a small amount.
        let output_box = AABB::new(
            Vec3::new(self.k - RECT_EPSILON, self.y0, self.z0),
            Vec3::new(self.k + RECT_EPSILON, self.y1, self.z1),
        );
        Some(output_box)
    }
}
