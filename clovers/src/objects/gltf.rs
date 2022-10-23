//! GLTF format support for the renderer

use gltf::image::Data;
use rand::rngs::SmallRng;
use rand::Rng;

use crate::{
    aabb::AABB,
    hitable::{get_orientation, HitRecord, HitableTrait},
    interval::Interval,
    materials::{gltf::GLTFMaterial, Material},
    ray::Ray,
    Float, Vec3, EPSILON_RECT_THICKNESS, EPSILON_SHADOW_ACNE,
};

/// Internal GLTF object representation after initialization.
#[derive(Debug, Clone)]
pub struct GLTFTriangle {
    /// Axis-aligned bounding box of the object
    pub aabb: AABB,
    /// Material of the object
    pub material: Material,
    q: Vec3,
    u: Vec3,
    v: Vec3,
    d: Float,
    w: Vec3,
    area: Float,
    normal: Vec3,
}

impl GLTFTriangle {
    #[must_use]
    /// Initialize a new GLTF object
    pub fn new(
        triangle: [Vec3; 3],
        tex_coords: [[Float; 2]; 3],
        material: &gltf::Material,
        images: &'static [Data],
    ) -> Self {
        // TODO: mostly adapted from Triangle, verify correctness!

        let [a, b, c] = triangle;
        let interval_x = Interval::new(a[0].min(b[0]).min(c[0]), a[0].max(b[0]).max(c[0]));
        let interval_y = Interval::new(a[1].min(b[1]).min(c[1]), a[1].max(b[1]).max(c[1]));
        let interval_z = Interval::new(a[2].min(b[2]).min(c[2]), a[2].max(b[2]).max(c[2]));
        let mut aabb: AABB = AABB::new(interval_x, interval_y, interval_z);
        aabb.pad();

        // TODO: Check orientation and make into a corner + edge vectors triangle
        let q = a;
        let u = b - q;
        let v = c - q;

        let n: Vec3 = u.cross(&v);
        let normal: Vec3 = n.normalize();
        // TODO: what is this?
        let d = -(normal.dot(&q));
        // TODO: what is this?
        let w: Vec3 = n / n.dot(&n);
        // Compared to quad, triangle has half the area
        let area = n.magnitude() / 2.0;

        let material: Material = Material::GLTF(GLTFMaterial::new(material, tex_coords, images));

        GLTFTriangle {
            aabb,
            material,
            q,
            u,
            v,
            d,
            w,
            area,
            normal,
        }
    }
}

impl HitableTrait for GLTFTriangle {
    fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        _rng: &mut SmallRng,
    ) -> Option<HitRecord> {
        // TODO: mostly adapted from Triangle, verify correctness!

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

    fn bounding_box(&self, _t0: Float, _t1: Float) -> Option<AABB> {
        Some(self.aabb)
    }

    fn pdf_value(&self, origin: Vec3, vector: Vec3, time: Float, rng: &mut SmallRng) -> Float {
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

    fn random(&self, origin: Vec3, rng: &mut SmallRng) -> Vec3 {
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

#[must_use]
fn hit_ab(a: Float, b: Float) -> bool {
    // Given the hit point in plane coordinates, return false if it is outside the
    // primitive, otherwise return true.
    // Triangle: a+b must be <=1.0
    (0.0..=1.0).contains(&a) && (0.0..=1.0).contains(&b) && (a + b <= 1.0)
}
