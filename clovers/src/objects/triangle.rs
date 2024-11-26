//! A triangle object. Almost exact copy of [Quad](crate::objects::Quad), with an adjusted `hit_ab` method.
// TODO: better docs

use crate::hitable::HitableTrait;
use crate::interval::Interval;
use crate::materials::MaterialInit;
use crate::wavelength::Wavelength;
use crate::{
    aabb::AABB, materials::Material, ray::Ray, Float, HitRecord, Vec3, EPSILON_RECT_THICKNESS,
};
use crate::{Direction, Displacement, Position, EPSILON_SHADOW_ACNE};
use nalgebra::Unit;
use rand::rngs::SmallRng;
use rand::Rng;

/// Initialization structure for a triangle primitive
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
pub struct TriangleInit {
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

/// Triangle shape. Heavily based on [Quad](crate::objects::Quad) and may contain inaccuracies
#[derive(Clone, Debug)]
pub struct Triangle<'scene> {
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

impl<'scene> Triangle<'scene> {
    /// Creates a new triangle from a coordinate point and two side vectors relative to the point
    #[must_use]
    #[allow(clippy::many_single_char_names)]
    pub fn new(q: Position, u: Vec3, v: Vec3, material: &'scene Material) -> Triangle<'scene> {
        let n: Vec3 = u.cross(&v);
        let normal: Direction = Unit::new_normalize(n);
        // TODO: what is this?
        let d = -(normal.dot(&q));
        // TODO: what is this?
        let w: Vec3 = n / n.dot(&n);
        // Compared to quad, triangle has half the area
        let area = n.magnitude() / 2.0;
        // Compute the AABB using the absolute coordinates of all corners
        // TODO: refactor to prettier code
        let corner1 = q;
        let corner2 = q + u;
        let corner3 = q + v;
        let interval_x = Interval::new(
            corner1[0].min(corner2[0]).min(corner3[0]),
            corner1[0].max(corner2[0]).max(corner3[0]),
        );
        let interval_y = Interval::new(
            corner1[1].min(corner2[1]).min(corner3[1]),
            corner1[1].max(corner2[1]).max(corner3[1]),
        );
        let interval_z = Interval::new(
            corner1[2].min(corner2[2]).min(corner3[2]),
            corner1[2].max(corner2[2]).max(corner3[2]),
        );
        let mut aabb: AABB = AABB::new(interval_x, interval_y, interval_z);
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
    #[must_use]
    #[allow(clippy::many_single_char_names)]
    pub fn from_coordinates(
        a: Vec3,
        b: Vec3,
        c: Vec3,
        material: &'scene Material,
    ) -> Triangle<'scene> {
        // Coordinate transform: from absolute coordinates to relative coordinates
        let q: Position = a;
        let u: Vec3 = b - q;
        let v: Vec3 = c - q;
        Triangle::new(q, u, v, material)
    }
}

impl<'scene> HitableTrait for Triangle<'scene> {
    /// Hit method for the triangle
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
        let mut record = HitRecord {
            distance: t,
            position: intersection,
            u: alpha,
            v: beta,
            material: self.material,
            normal: self.normal,
            front_face: false,
        };
        record.set_face_normal(ray, self.normal);

        Some(record)
    }

    /// Returns the bounding box of the triangle
    #[must_use]
    fn aabb(&self) -> Option<&AABB> {
        // TODO: this is from quad and not updated!
        // although i guess a triangle's aabb is the same as the quad's aabb in worst case
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
        // TODO: this is from quad and not updated!
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

    /// Returns a random point on the triangle surface
    #[must_use]
    fn random(&self, origin: Position, rng: &mut SmallRng) -> Displacement {
        // Random square coordinate
        let mut a = rng.gen::<Float>();
        let mut b = rng.gen::<Float>();

        // If we're beyond the diagonal, rotate around a point by flipping on both axes
        if a + b > 1.0 {
            a = 1.0 - a;
            b = 1.0 - b;
        }

        let point: Position = self.q // world-coordinate corner + random distances along edge vectors
            + (a * self.u)
            + (b * self.v);

        point - origin
    }

    // TODO: correctness
    fn centroid(&self) -> Position {
        self.q + (self.u / 4.0) + (self.v / 4.0)
    }
}

#[must_use]
fn hit_ab(a: Float, b: Float) -> bool {
    // Given the hit point in plane coordinates, return false if it is outside the
    // primitive, otherwise return true.
    // Triangle: a+b must be <=1.0
    (0.0..=1.0).contains(&a) && (0.0..=1.0).contains(&b) && (a + b <= 1.0)
}

// TODO: proptest!
#[cfg(test)]
mod tests {
    use alloc::boxed::Box;
    use rand::SeedableRng;

    use crate::interval::Interval;

    use super::*;

    const TIME_0: Float = 0.0;
    const TIME_1: Float = 1.0;
    const RAY_POS: Ray = Ray {
        origin: Position::new(0.01, 0.01, -1.0),
        direction: Unit::new_unchecked(Vec3::new(0.0, 0.0, 1.0)),
        time: TIME_0,
        wavelength: 600,
    };

    const RAY_NEG: Ray = Ray {
        origin: Position::new(-0.01, -0.01, -1.0),
        direction: Unit::new_unchecked(Vec3::new(0.0, 0.0, 1.0)),
        time: TIME_0,
        wavelength: 600,
    };

    #[test]
    fn xy_unit_triangle() {
        let mut rng = SmallRng::from_entropy();
        let material = Box::default();

        // Unit triangle at origin
        let triangle = Triangle::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            &material,
        );

        let aabb = triangle.aabb().expect("No AABB for the triangle");

        let expected_aabb = AABB::new(
            Interval::new(0.0, 1.0),
            Interval::new(0.0, 1.0),
            Interval::new(0.0, 0.0).expand(EPSILON_RECT_THICKNESS),
        );

        assert_eq!(aabb, &expected_aabb);

        let boxhit = aabb.hit(&RAY_POS, TIME_0, TIME_1);
        assert!(boxhit);

        let hit_record = triangle
            .hit(&RAY_POS, Float::NEG_INFINITY, Float::INFINITY, &mut rng)
            .expect("No hit record for triangle and ray");

        assert!(hit_record.distance - 1.0 <= Float::EPSILON);
        assert_eq!(hit_record.position, Vec3::new(0.01, 0.01, 0.0));
        assert_eq!(
            hit_record.normal,
            Unit::new_normalize(Vec3::new(0.0, 0.0, -1.0))
        );
        assert!(!hit_record.front_face);
    }

    #[test]
    fn yx_unit_triangle() {
        let mut rng = SmallRng::from_entropy();
        let material = Box::default();

        // Unit triangle at origin, u and v coords swapped
        let triangle = Triangle::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            &material,
        );

        let aabb = triangle.aabb().expect("No AABB for the triangle");

        let expected_aabb = AABB::new(
            Interval::new(0.0, 1.0),
            Interval::new(0.0, 1.0),
            Interval::new(0.0, 0.0).expand(EPSILON_RECT_THICKNESS),
        );

        assert_eq!(aabb, &expected_aabb);

        let boxhit = aabb.hit(&RAY_POS, TIME_0, TIME_1);
        assert!(boxhit);

        let hit_record = triangle
            .hit(&RAY_POS, Float::NEG_INFINITY, Float::INFINITY, &mut rng)
            .expect("No hit record for triangle and ray");

        assert!(hit_record.distance - 1.0 <= Float::EPSILON);
        assert_eq!(hit_record.position, Position::new(0.01, 0.01, 0.0));
        assert_eq!(
            hit_record.normal,
            Unit::new_normalize(Vec3::new(0.0, 0.0, -1.0))
        );
        assert!(hit_record.front_face);
    }

    #[test]
    fn neg_xy_unit_triangle() {
        let mut rng = SmallRng::from_entropy();
        let material: Box<Material> = Box::default();

        // Unit triangle at origin, u and v coords swapped
        let triangle = Triangle::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            &material,
        );

        let aabb = triangle.aabb().expect("No AABB for the triangle");

        let expected_aabb = AABB::new(
            Interval::new(-1.0, 0.0),
            Interval::new(-1.0, 0.0),
            Interval::new(0.0, 0.0).expand(EPSILON_RECT_THICKNESS),
        );

        assert_eq!(aabb, &expected_aabb);

        let boxhit = aabb.hit(&RAY_NEG, TIME_0, TIME_1);
        assert!(boxhit);

        let hit_record = triangle
            .hit(&RAY_NEG, Float::NEG_INFINITY, Float::INFINITY, &mut rng)
            .expect("No hit record for triangle and ray");

        assert!(hit_record.distance - 1.0 <= Float::EPSILON);
        assert_eq!(hit_record.position, Position::new(-0.01, -0.01, 0.0));
        assert_eq!(
            hit_record.normal,
            Unit::new_normalize(Vec3::new(0.0, 0.0, -1.0))
        );
        assert!(!hit_record.front_face);
    }

    #[test]
    fn neg_yx_unit_triangle() {
        let mut rng = SmallRng::from_entropy();
        let material: Box<Material> = Box::default();

        // Unit triangle at origin, u and v coords swapped
        let triangle = Triangle::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            &material,
        );

        let aabb = triangle.aabb().expect("No AABB for the triangle");

        let expected_aabb = AABB::new(
            Interval::new(-1.0, 0.0),
            Interval::new(-1.0, 0.0),
            Interval::new(0.0, 0.0).expand(EPSILON_RECT_THICKNESS),
        );

        assert_eq!(aabb, &expected_aabb);

        let boxhit = aabb.hit(&RAY_NEG, TIME_0, TIME_1);
        assert!(boxhit);

        let hit_record = triangle
            .hit(&RAY_NEG, Float::NEG_INFINITY, Float::INFINITY, &mut rng)
            .expect("No hit record for triangle and ray");

        assert!(hit_record.distance - 1.0 <= Float::EPSILON);
        assert_eq!(hit_record.position, Position::new(-0.01, -0.01, 0.0));
        assert_eq!(
            hit_record.normal,
            Unit::new_normalize(Vec3::new(0.0, 0.0, -1.0))
        );
        assert!(hit_record.front_face);
    }
}
