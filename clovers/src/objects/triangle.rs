//! A triangle object. Almost exact copy of [Quad](crate::objects::Quad), with an adjusted `hit_ab` method.
// TODO: better docs

use crate::EPSILON_SHADOW_ACNE;
use crate::{
    aabb::AABB, hitable::get_orientation, hitable::HitRecord, materials::Material, ray::Ray, Float,
    Vec3, EPSILON_RECT_THICKNESS,
};
use rand::rngs::SmallRng;
use rand::Rng;

/// Initialization structure for a triangle primitive
#[derive(Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
pub struct TriangleInit {
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

/// Triangle shape. Heavily based on [Quad](crate::objects::Quad) and may contain inaccuracies
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
pub struct Triangle {
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

impl Triangle {
    /// Creates a new triangle from a coordinate point and two side vectors relative to the point
    pub fn new(q: Vec3, u: Vec3, v: Vec3, material: Material) -> Triangle {
        let n: Vec3 = u.cross(&v);
        let normal: Vec3 = n.normalize();
        // TODO: what is this?
        let d = -(normal.dot(&q));
        // TODO: what is this?
        let w: Vec3 = n / n.dot(&n);
        // Compared to quad, triangle has half the area
        let area = n.magnitude() / 2.0;
        // TODO: more accurate AABB for triangle; this is from quad
        let mut aabb: AABB = AABB::new_from_coords(q, q + u + v);
        aabb.pad();

        Triangle {
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

    /// Creates a new triangle from three Cartesian space coordinates
    pub fn from_coordinates(a: Vec3, b: Vec3, c: Vec3, material: Material) -> Triangle {
        // Coordinate transform: from absolute coordinates to relative coordinates
        let q: Vec3 = a;
        let u: Vec3 = b - q;
        let v: Vec3 = c - q;
        Triangle::new(q, u, v, material)
    }

    /// Hit method for the triangle
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

    /// Returns the bounding box of the triangle
    pub fn bounding_box(&self, _t0: Float, _t1: Float) -> Option<AABB> {
        // TODO: this is from quad and not updated!
        // although i guess a triangle's aabb is the same as the quad's aabb in worst case
        Some(self.aabb)
    }

    /// Returns a probability density function value? // TODO: understand & explain
    pub fn pdf_value(&self, origin: Vec3, vector: Vec3, time: Float, rng: &mut SmallRng) -> Float {
        // TODO: this is from quad and not updated!
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

    /// Returns a random point on the triangle surface
    pub fn random(&self, origin: Vec3, rng: &mut SmallRng) -> Vec3 {
        let mut a = rng.gen::<Float>();
        let mut b = rng.gen::<Float>();
        if a + b > 1.0 {
            a = 1.0 - a;
            b = 1.0 - b;
        }

        let point: Vec3 = self.q + (a * self.u) + (b * self.v);

        point - origin
    }
}

fn hit_ab(a: Float, b: Float) -> bool {
    // Given the hit point in plane coordinates, return false if it is outside the
    // primitive, otherwise return true.
    // Triangle: a+b must be <=1.0
    (0.0..=1.0).contains(&a) && (0.0..=1.0).contains(&b) && (a + b <= 1.0)
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;

    use crate::interval::Interval;

    use super::*;

    #[test]
    fn xy_unit_triangle() {
        let time_0 = 0.0;
        let time_1 = 1.0;

        // Unit triangle at origin
        let xy_unit_triangle = Triangle::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Default::default(),
        );

        let ray = Ray::new(
            Vec3::new(0.0, 0.0, -1.0).normalize(),
            Vec3::new(0.0, 0.0, 1.0).normalize(),
            time_0,
        );

        let mut rng = SmallRng::from_entropy();

        let aabb = xy_unit_triangle
            .bounding_box(time_0, time_1)
            .expect("No AABB for the triangle");

        let expected_aabb = AABB::new(
            Interval::new(0.0, 1.0),
            Interval::new(0.0, 1.0),
            Interval::new(0.0, 0.0).expand(EPSILON_RECT_THICKNESS),
        );

        assert_eq!(aabb, expected_aabb);

        let boxhit = aabb.hit(&ray, time_0, time_1);
        assert!(boxhit);

        let hit_record = xy_unit_triangle
            .hit(&ray, Float::NEG_INFINITY, Float::INFINITY, &mut rng)
            .expect("No hit record for triangle and ray");

        assert!(hit_record.distance - 1.0 <= Float::EPSILON);
        assert_eq!(hit_record.position, Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(hit_record.normal, Vec3::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn yx_unit_triangle() {
        let time_0 = 0.0;
        let time_1 = 1.0;

        // Unit triangle at origin, u and v coords swapped
        let xy_unit_triangle = Triangle::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Default::default(),
        );

        let ray = Ray::new(
            Vec3::new(0.0, 0.0, -1.0).normalize(),
            Vec3::new(0.0, 0.0, 1.0).normalize(),
            time_0,
        );

        let mut rng = SmallRng::from_entropy();

        let aabb = xy_unit_triangle
            .bounding_box(time_0, time_1)
            .expect("No AABB for the triangle");

        let expected_aabb = AABB::new(
            Interval::new(0.0, 1.0),
            Interval::new(0.0, 1.0),
            Interval::new(0.0, 0.0).expand(EPSILON_RECT_THICKNESS),
        );

        assert_eq!(aabb, expected_aabb);

        let boxhit = aabb.hit(&ray, time_0, time_1);
        assert!(boxhit);

        let hit_record = xy_unit_triangle
            .hit(&ray, Float::NEG_INFINITY, Float::INFINITY, &mut rng)
            .expect("No hit record for triangle and ray");

        assert!(hit_record.distance - 1.0 <= Float::EPSILON);
        assert_eq!(hit_record.position, Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(hit_record.normal, Vec3::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn neg_xy_unit_triangle() {
        let time_0 = 0.0;
        let time_1 = 1.0;

        // Unit triangle at origin, u and v coords swapped
        let neg_xy_unit_triangle = Triangle::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Default::default(),
        );

        let ray = Ray::new(
            Vec3::new(0.0, 0.0, -1.0).normalize(),
            Vec3::new(0.0, 0.0, 1.0).normalize(),
            time_0,
        );

        let mut rng = SmallRng::from_entropy();

        let aabb = neg_xy_unit_triangle
            .bounding_box(time_0, time_1)
            .expect("No AABB for the triangle");

        let expected_aabb = AABB::new(
            Interval::new(-1.0, 0.0),
            Interval::new(-1.0, 0.0),
            Interval::new(0.0, 0.0).expand(EPSILON_RECT_THICKNESS),
        );

        assert_eq!(aabb, expected_aabb);

        let boxhit = aabb.hit(&ray, time_0, time_1);
        assert!(boxhit);

        let hit_record = neg_xy_unit_triangle
            .hit(&ray, Float::NEG_INFINITY, Float::INFINITY, &mut rng)
            .expect("No hit record for triangle and ray");

        assert!(hit_record.distance - 1.0 <= Float::EPSILON);
        assert_eq!(hit_record.position, Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(hit_record.normal, Vec3::new(0.0, 0.0, -1.0));
    }
}
