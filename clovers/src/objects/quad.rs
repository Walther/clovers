//! A quadrilateral object.
// TODO: better docs

use crate::EPSILON_SHADOW_ACNE;
use crate::{
    aabb::AABB, hitable::get_orientation, hitable::HitRecord, materials::Material, ray::Ray, Float,
    Vec3, EPSILON_RECT_THICKNESS,
};
use rand::rngs::SmallRng;
use rand::Rng;

/// Initialization structure for a Quad object.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
pub struct QuadInit {
    /// Corner point
    pub q: Vec3,
    /// Vector describing the u side
    pub u: Vec3,
    /// Vector describing the v side
    pub v: Vec3,
    /// Material of the surface
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub material: Material,
}

/// Quadrilateral shape. This can be an arbitrary parallelogram, not just a rectangle.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
pub struct Quad {
    /// Corner point
    pub q: Vec3,
    /// Vector describing the u side
    pub u: Vec3,
    /// Vector describing the v side
    pub v: Vec3,
    /// Material of the surface
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub material: Material,
    /// Area of the surface
    pub area: Float,
    /// Normal vector of the surface
    pub normal: Vec3,
    /// What is this? // TODO: understand, explain
    pub d: Float,
    /// What is this? // TODO: understand, explain
    pub w: Vec3,
    /// Bounding box of the surface
    pub aabb: AABB,
}

impl Quad {
    /// Creates a new quad
    pub fn new(q: Vec3, u: Vec3, v: Vec3, material: Material) -> Quad {
        let n: Vec3 = u.cross(&v);
        let normal: Vec3 = n.normalize();
        // TODO: what is this?
        let d = -(normal.dot(&q));
        // TODO: what is this?
        let w: Vec3 = n / n.dot(&n);
        let area = n.magnitude();
        // TODO: does this need .pad() method from book?
        let aabb = AABB::new(q, q + u + v);

        Quad {
            q,
            u,
            v,
            material,
            area,
            normal,
            d,
            w,
            aabb,
        }
    }

    /// Hit method for the quad rectangle
    pub fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        _rng: &mut SmallRng,
    ) -> Option<HitRecord> {
        let denom = self.normal.dot(&ray.direction);

        // No hit if the ray is parallel to the plane.
        if denom.abs() < EPSILON_RECT_THICKNESS {
            return None;
        }

        // Return false if the hit point parameter t is outside the ray interval
        let t = (-self.d - self.normal.dot(&ray.origin)) / denom;
        if t < distance_min || t > distance_max {
            return None;
        }

        // Determine the hit point lies within the planar shape using its plane coordinates.
        let intersection: Vec3 = ray.evaluate(t);
        let planar_hitpt_vector: Vec3 = intersection - self.q;
        let alpha: Float = self.w.dot(&planar_hitpt_vector.cross(&self.v));
        let beta: Float = self.w.dot(&self.u.cross(&planar_hitpt_vector));

        // Do we hit a coordinate within the surface of the plane?
        if !hit_ab(alpha, beta) {
            return None;
        }

        // Ray hits the 2D shape; set the rest of the hit record and return

        let (front_face, normal) = get_orientation(ray, self.normal);

        Some(HitRecord {
            distance: t,
            position: intersection,
            normal,
            u: alpha,
            v: beta,
            material: &self.material,
            front_face,
        })
    }

    /// Returns the bounding box of the quad
    pub fn bounding_box(&self, _t0: Float, _t1: Float) -> Option<AABB> {
        Some(self.aabb)
    }

    /// Returns a probability density function value? // TODO: understand & explain
    pub fn pdf_value(&self, origin: Vec3, vector: Vec3, time: Float, rng: &mut SmallRng) -> Float {
        match self.hit(
            &Ray::new(origin, vector, time),
            EPSILON_SHADOW_ACNE,
            Float::INFINITY,
            rng,
        ) {
            Some(hit_record) => {
                let distance_squared =
                    hit_record.distance * hit_record.distance * vector.norm_squared();
                let cosine = vector.dot(&hit_record.normal).abs() / vector.magnitude();

                distance_squared / (cosine * self.area)
            }
            None => 0.0,
        }
    }

    /// Returns a random point on the quadrilateral surface
    pub fn random(&self, origin: Vec3, rng: &mut SmallRng) -> Vec3 {
        let point: Vec3 = self.q + (rng.gen::<Float>() * self.u) + (rng.gen::<Float>() * self.v);
        point - origin
    }
}

fn hit_ab(a: Float, b: Float) -> bool {
    // Given the hit point in plane coordinates, return false if it is outside the
    // primitive, otherwise return true.
    (0.0..=1.0).contains(&a) && (0.0..=1.0).contains(&b)
}