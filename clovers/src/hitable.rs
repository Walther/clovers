//! An abstraction for things that can be hit by [Rays](crate::ray::Ray).

#[cfg(feature = "gl_tf")]
use crate::objects::GLTFTriangle;

use crate::{
    aabb::AABB,
    bvh::{build::utils::vec_bounding_box, BVHNode},
    objects::{Boxy, ConstantMedium, MovingSphere, Quad, RotateY, Sphere, Translate, Triangle},
    ray::Ray,
    wavelength::Wavelength,
    Direction, Displacement, Float, HitRecord, Position, Vec3,
};

use enum_dispatch::enum_dispatch;
use rand::{rngs::SmallRng, seq::IteratorRandom};

/// Enumeration of all runtime entities that can be intersected aka "hit" by a [Ray].
#[enum_dispatch(HitableTrait)]
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub enum Hitable<'scene> {
    Boxy(Boxy<'scene>),
    BVHNode(BVHNode<'scene>),
    ConstantMedium(ConstantMedium<'scene>),
    MovingSphere(MovingSphere<'scene>),
    Quad(Quad<'scene>),
    RotateY(RotateY<'scene>),
    Sphere(Sphere<'scene>),
    Translate(Translate<'scene>),
    Triangle(Triangle<'scene>),
    Empty(Empty),
    #[cfg(feature = "gl_tf")]
    GLTFTriangle(GLTFTriangle<'scene>),
    HitableList(HitableList<'scene>),
}

// TODO: remove horrible hack
#[derive(Debug, Clone)]
/// Empty hitable. Cannot be hit. Exists only as an internal workaround.
pub struct Empty {}

impl HitableTrait for Empty {
    fn hit(
        &self,
        _ray: &Ray,
        _distance_min: Float,
        _distance_max: Float,
        _rng: &mut SmallRng,
    ) -> Option<HitRecord> {
        None
    }

    fn aabb(&self) -> Option<&AABB> {
        None
    }

    fn pdf_value(
        &self,
        _origin: Position,
        _direction: Direction,
        _wavelength: Wavelength,
        _time: Float,
        _rng: &mut SmallRng,
    ) -> Float {
        0.0
    }

    fn centroid(&self) -> Position {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

#[enum_dispatch]
/// The main trait for entities that can be intersect aka "hit" by a [Ray].
pub trait HitableTrait {
    #[must_use]
    /// The main intersection method.
    fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        rng: &mut SmallRng,
    ) -> Option<HitRecord>;

    #[must_use]
    /// Returns the bounding box of the entity.
    fn aabb(&self) -> Option<&AABB>;

    #[must_use]
    /// Probability density function value method, used for multiple importance sampling.
    fn pdf_value(
        &self,
        origin: Position,
        direction: Direction,
        wavelength: Wavelength,
        time: Float,
        rng: &mut SmallRng,
    ) -> Float;

    #[must_use]
    /// Random point on the entity, used for multiple importance sampling.
    fn random(&self, _origin: Position, _rng: &mut SmallRng) -> Displacement {
        unimplemented!(
            "HitableTrait::random called for a Hitable that has no implementation for it!"
        );
    }

    /// Returns the center point of the hitable
    #[must_use]
    fn centroid(&self) -> Position;
}

/// Returns a tuple of `(front_face, normal)`. Used in lieu of `set_face_normal` in the Ray Tracing for the Rest Of Your Life book.
#[must_use]
pub fn get_orientation(ray: &Ray, outward_normal: Direction) -> (bool, Direction) {
    let front_face = ray.direction.dot(&outward_normal) < 0.0;
    let normal = if front_face {
        outward_normal
    } else {
        -outward_normal
    };

    (front_face, normal)
}

/// A list of `Hitable`s, occasionally used as the leaf of `BVHNode` when further splitting is not possible or beneficial.
///
/// Hopefully temporary.
// TODO: remove?
#[derive(Debug, Clone)]
pub struct HitableList<'scene> {
    /// Hitables in the list
    pub hitables: Vec<Hitable<'scene>>,
    aabb: AABB,
}

impl<'scene> HitableList<'scene> {
    /// Creates a new [`HitableList`].
    ///
    /// # Panics
    /// This method may panic if no finite bounding box can be created for the given `hitables`.
    #[must_use]
    pub fn new(hitables: Vec<Hitable<'scene>>) -> Self {
        let aabb = vec_bounding_box(&hitables).expect("No bounding box for hitables");
        Self { hitables, aabb }
    }

    /// Recursively flattens the `HitableList` into a `Vec<Hitable>`
    #[must_use]
    pub fn flatten(self) -> Vec<Hitable<'scene>> {
        let mut flat: Vec<Hitable> = Vec::new();
        for hitable in &self.hitables {
            match hitable {
                Hitable::HitableList(l) => {
                    let mut flatten = l.clone().flatten();
                    flat.append(&mut flatten);
                }
                h => flat.push(h.clone()),
            }
        }
        flat
    }
}

// TODO: ideally, this impl should be removed entirely
impl HitableTrait for HitableList<'_> {
    #[must_use]
    fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        rng: &mut SmallRng,
    ) -> Option<HitRecord> {
        let mut distance = Float::INFINITY;
        let mut closest: Option<HitRecord> = None;
        for hitable in &self.hitables {
            let hit_record = hitable.hit(ray, distance_min, distance_max, rng)?;
            if hit_record.distance < distance {
                distance = hit_record.distance;
                closest = Some(hit_record);
            }
        }

        closest
    }

    #[must_use]
    fn aabb(&self) -> Option<&AABB> {
        Some(&self.aabb)
    }

    #[must_use]
    fn pdf_value(
        &self,
        _origin: Position,
        _direction: Direction,
        _wavelength: Wavelength,
        _time: Float,
        _rng: &mut SmallRng,
    ) -> Float {
        // TODO: fix
        0.0
    }

    #[must_use]
    fn centroid(&self) -> Position {
        // TODO: ideally, this shouldn't be used at all!
        // Currently, this can be called when a `HitableList` is used as an object within a `Translate` or `RotateY`
        // Those should be removed too!
        self.aabb.centroid()
    }

    fn random(&self, origin: Position, rng: &mut SmallRng) -> Displacement {
        let hitable = self.hitables.iter().choose(rng).unwrap();
        hitable.random(origin, rng)
    }
}
