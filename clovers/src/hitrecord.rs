//! Represents a ray-object intersection, with plenty of data about the intersection.

#[cfg(not(target_arch = "spirv"))] // TODO: gpu support
use crate::materials::Material;

use crate::{materials::GPUMaterial, ray::Ray, Float, Vec3};

/// Represents a ray-object intersection, with plenty of data about the intersection.
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[cfg(not(target_arch = "spirv"))] // TODO: gpu support
pub struct HitRecord<'a> {
    /// Distance from the ray origin to the hitpoint
    pub distance: Float,
    /// 3D coordinate of the hitpoint
    pub position: Vec3,
    /// Surface normal from the hitpoint
    pub normal: Vec3,
    /// U surface coordinate of the hitpoint
    pub u: Float,
    /// V surface coordinate of the hitpoint
    pub v: Float,
    /// Reference to the material at the hitpoint
    pub material: &'a Material,
    /// Is the hitpoint at the front of the surface
    pub front_face: bool,
}

#[cfg(not(target_arch = "spirv"))] // TODO: gpu support
impl<'a> HitRecord<'a> {
    /// Helper function for getting normals pointing at the correct direction. TODO: consider removal?
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}

/// Represents a ray-object intersection, with plenty of data about the intersection.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GPUHitRecord {
    /// Distance from the ray origin to the hitpoint
    pub distance: Float,
    /// 3D coordinate of the hitpoint
    pub position: Vec3,
    /// Surface normal from the hitpoint
    pub normal: Vec3,
    /// U surface coordinate of the hitpoint
    pub u: Float,
    /// V surface coordinate of the hitpoint
    pub v: Float,
    /// Material at the hitpoint
    pub material: GPUMaterial,
    /// Is the hitpoint at the front of the surface
    pub front_face: u32, // TODO: fix manual bool. This is due to upstream issue in rust-gpu -> naga
}

impl GPUHitRecord {
    /// Helper function for getting normals pointing at the correct direction. TODO: consider removal?
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        if ray.direction.dot(outward_normal) < 0.0 {
            self.front_face = 1
        } else {
            self.front_face = 0
        }
        if self.front_face == 1 {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}
