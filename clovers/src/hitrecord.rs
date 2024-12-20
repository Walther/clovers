//! The main data structure returned for every surface intersection.

use crate::{materials::MaterialTrait, ray::Ray, Direction, Float, Position};

/// Represents a ray-object intersection, with plenty of data about the intersection.
#[derive(Clone, Debug)]
pub struct HitRecord<'a> {
    /// Distance from the ray origin to the hitpoint
    pub distance: Float,
    /// 3D coordinate of the hitpoint
    pub position: Position,
    /// Surface normal from the hitpoint
    pub normal: Direction,
    /// U surface coordinate of the hitpoint
    pub u: Float,
    /// V surface coordinate of the hitpoint
    pub v: Float,
    /// Reference to the material at the hitpoint
    pub material: &'a dyn MaterialTrait,
    /// Is the hitpoint at the front of the surface
    pub front_face: bool,
}

impl HitRecord<'_> {
    /// Sets the face normal of this [`HitRecord`].
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Direction) {
        self.front_face = ray.direction.dot(&outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}
