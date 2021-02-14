use crate::{
    aabb::AABB,
    hitable::{HitRecord, Hitable},
    materials::Material,
    ray::Ray,
    Float, Vec3, EPSILON_RECT_THICKNESS, EPSILON_SHADOW_ACNE,
};
use rand::prelude::*;
use serde::{Deserialize, Serialize};

// XY

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct XYRectInit {
    pub x0: Float,
    pub x1: Float,
    pub y0: Float,
    pub y1: Float,
    pub k: Float,
    #[serde(default)]
    pub material: Material,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct XYRect {
    x0: Float,
    x1: Float,
    y0: Float,
    y1: Float,
    k: Float,
    material: Material,
}

impl XYRect {
    pub fn new(
        x0: Float,
        x1: Float,
        y0: Float,
        y1: Float,
        k: Float,
        material: Material,
    ) -> Hitable {
        Hitable::XYRect(XYRect {
            x0,
            x1,
            y0,
            y1,
            k,
            material,
        })
    }

    pub fn hit(
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
        Some(record)
    }
    pub fn bounding_box(&self, _t0: Float, _t1: Float) -> Option<AABB> {
        // The bounding box must have non-zero width in each dimension, so pad the Z dimension a small amount.
        let output_box = AABB::new(
            Vec3::new(self.x0, self.y0, self.k - EPSILON_RECT_THICKNESS),
            Vec3::new(self.x1, self.y1, self.k + EPSILON_RECT_THICKNESS),
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
            Some(hit_record) => {
                let area = (self.x1 - self.x0) * (self.y1 - self.y0); // NOTE: should this have an abs()?
                let distance_squared =
                    hit_record.distance * hit_record.distance * vector.norm_squared();
                let cosine = vector.dot(&hit_record.normal).abs() / vector.norm();

                distance_squared / (cosine * area)
            }
            None => 0.0,
        }
    }

    pub fn random(&self, origin: Vec3, mut rng: ThreadRng) -> Vec3 {
        let random_point = Vec3::new(
            rng.gen_range(self.x0, self.x1),
            rng.gen_range(self.y0, self.y1),
            self.k,
        );
        random_point - origin
    }
}

// XZ

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct XZRectInit {
    pub x0: Float,
    pub x1: Float,
    pub z0: Float,
    pub z1: Float,
    pub k: Float,
    #[serde(default)]
    pub material: Material,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct XZRect {
    x0: Float,
    x1: Float,
    z0: Float,
    z1: Float,
    k: Float,
    material: Material,
}

impl XZRect {
    pub fn new(
        x0: Float,
        x1: Float,
        z0: Float,
        z1: Float,
        k: Float,
        material: Material,
    ) -> Hitable {
        Hitable::XZRect(XZRect {
            x0,
            x1,
            z0,
            z1,
            k,
            material,
        })
    }

    pub fn hit(
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
        Some(record)
    }
    pub fn bounding_box(&self, _t0: Float, _t1: Float) -> Option<AABB> {
        // The bounding box must have non-zero width in each dimension, so pad the Z dimension a small amount.
        let output_box = AABB::new(
            Vec3::new(self.x0, self.k - EPSILON_RECT_THICKNESS, self.z0),
            Vec3::new(self.x1, self.k + EPSILON_RECT_THICKNESS, self.z1),
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
            Some(hit_record) => {
                let area = (self.x1 - self.x0) * (self.z1 - self.z0); // NOTE: should this have an abs()?
                let distance_squared =
                    hit_record.distance * hit_record.distance * vector.norm_squared();
                let cosine = vector.dot(&hit_record.normal).abs() / vector.norm();

                distance_squared / (cosine * area)
            }
            None => 0.0,
        }
    }

    pub fn random(&self, origin: Vec3, mut rng: ThreadRng) -> Vec3 {
        let random_point = Vec3::new(
            rng.gen_range(self.x0, self.x1),
            self.k,
            rng.gen_range(self.z0, self.z1),
        );
        random_point - origin
    }
}

// YZ

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct YZRectInit {
    pub y0: Float,
    pub y1: Float,
    pub z0: Float,
    pub z1: Float,
    pub k: Float,
    #[serde(default)]
    pub material: Material,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct YZRect {
    y0: Float,
    y1: Float,
    z0: Float,
    z1: Float,
    k: Float,
    material: Material,
}

impl YZRect {
    pub fn new(
        y0: Float,
        y1: Float,
        z0: Float,
        z1: Float,
        k: Float,
        material: Material,
    ) -> Hitable {
        Hitable::YZRect(YZRect {
            y0,
            y1,
            z0,
            z1,
            k,
            material,
        })
    }

    pub fn hit(
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
        Some(record)
    }
    pub fn bounding_box(&self, _t0: Float, _t1: Float) -> Option<AABB> {
        // The bounding box must have non-zero width in each dimension, so pad the Z dimension a small amount.
        let output_box = AABB::new(
            Vec3::new(self.k - EPSILON_RECT_THICKNESS, self.y0, self.z0),
            Vec3::new(self.k + EPSILON_RECT_THICKNESS, self.y1, self.z1),
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
            Some(hit_record) => {
                let area = (self.y1 - self.y0) * (self.z1 - self.z0); // NOTE: should this have an abs()?
                let distance_squared =
                    hit_record.distance * hit_record.distance * vector.norm_squared();
                let cosine = vector.dot(&hit_record.normal).abs() / vector.norm();

                distance_squared / (cosine * area)
            }
            None => 0.0,
        }
    }

    pub fn random(&self, origin: Vec3, mut rng: ThreadRng) -> Vec3 {
        let random_point = Vec3::new(
            self.k,
            rng.gen_range(self.y0, self.y1),
            rng.gen_range(self.z0, self.z1),
        );
        random_point - origin
    }
}
