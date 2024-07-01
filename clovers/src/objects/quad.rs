//! A quadrilateral object.
// TODO: better docs

use crate::hitable::HitableTrait;
use crate::materials::MaterialInit;
use crate::wavelength::Wavelength;
use crate::{
    aabb::AABB, hitable::get_orientation, HitRecord, materials::Material, ray::Ray, Float,
    Vec3, EPSILON_RECT_THICKNESS,
};
use crate::{Direction, Displacement, Position, EPSILON_SHADOW_ACNE};
use nalgebra::Unit;
use rand::rngs::SmallRng;
use rand::Rng;

/// Initialization structure for a Quad object.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
pub struct QuadInit {
    /// Used for multiple importance sampling
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub priority: bool,
    /// Corner point
    pub q: Position,
    /// Vector describing the u side
    pub u: Vec3,
    /// Vector describing the v side
    pub v: Vec3,
    /// Material of the surface
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub material: MaterialInit,
}

/// Quadrilateral shape. This can be an arbitrary parallelogram, not just a rectangle.
#[derive(Clone, Debug)]
pub struct Quad<'scene> {
    /// Corner point
    pub q: Position,
    /// Vector describing the u side
    pub u: Vec3,
    /// Vector describing the v side
    pub v: Vec3,
    /// Material of the surface
    pub material: &'scene Material,
    /// Area of the surface
    pub area: Float,
    /// Normal vector of the surface
    pub normal: Direction,
    /// What is this? // TODO: understand, explain
    pub d: Float,
    /// What is this? // TODO: understand, explain
    pub w: Vec3,
    /// Bounding box of the surface
    pub aabb: AABB,
}

impl<'scene> Quad<'scene> {
    /// Creates a new quad
    #[must_use]
    pub fn new(q: Position, u: Vec3, v: Vec3, material: &'scene Material) -> Quad<'scene> {
        let n: Vec3 = u.cross(&v);
        let normal = Unit::new_normalize(n);
        // TODO: what is this?
        let d = -(normal.dot(&q));
        // TODO: what is this?
        let w: Vec3 = n / n.dot(&n);
        let area = n.magnitude();
        let mut aabb = AABB::new_from_coords(q, q + u + v);
        aabb.pad();

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
}

impl<'scene> HitableTrait for Quad<'scene> {
    /// Hit method for the quad rectangle
    #[must_use]
    fn hit(
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
        let intersection: Position = ray.evaluate(t);
        let planar_hitpt_vector: Position = intersection - self.q;
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
            material: self.material,
            front_face,
        })
    }

    /// Returns the bounding box of the quad
    #[must_use]
    fn bounding_box(&self) -> Option<&AABB> {
        Some(&self.aabb)
    }

    /// Returns a probability density function value? // TODO: understand & explain
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
            Some(hit_record) => {
                let distance_squared =
                    hit_record.distance * hit_record.distance * direction.norm_squared();
                let cosine = direction.dot(&hit_record.normal).abs() / direction.magnitude();

                distance_squared / (cosine * self.area)
            }
            None => 0.0,
        }
    }

    /// Returns a random point on the quadrilateral surface
    #[must_use]
    fn random(&self, origin: Position, rng: &mut SmallRng) -> Displacement {
        let point: Position = self.q // world-coordinate corner + random distances along edge vectors
                + (rng.gen::<Float>() * self.u)
                + (rng.gen::<Float>() * self.v);
        point - origin
    }
}

#[must_use]
fn hit_ab(a: Float, b: Float) -> bool {
    // Given the hit point in plane coordinates, return false if it is outside the
    // primitive, otherwise return true.
    (0.0..=1.0).contains(&a) && (0.0..=1.0).contains(&b)
}
