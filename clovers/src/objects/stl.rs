//! STL utilities

use alloc::string::String;
use nalgebra::Rotation3;
use rand::prelude::SmallRng;
use std::fs::OpenOptions;

use crate::{
    aabb::AABB,
    bvhnode::BVHNode,
    hitable::{HitRecord, Hitable, HitableTrait},
    materials::{Material, MaterialInit, SharedMaterial},
    objects::Triangle,
    ray::Ray,
    wavelength::Wavelength,
    Float, Vec3,
};

/// Internal STL object representation after initialization. Contains the material for all triangles in it to avoid having n copies.
#[derive(Debug, Clone)]
pub struct STL<'scene> {
    /// Bounding Volume Hierarchy tree for the object
    pub bvhnode: BVHNode<'scene>,
    /// Material for the object
    pub material: &'scene Material,
    /// Axis-aligned bounding box of the object
    pub aabb: AABB,
}

impl<'scene> HitableTrait for STL<'scene> {
    /// Hit method for the STL object
    #[must_use]
    fn hit(
        &self,
        ray: &Ray,
        distance_min: f32,
        distance_max: f32,
        rng: &mut SmallRng,
    ) -> Option<HitRecord> {
        self.bvhnode.hit(ray, distance_min, distance_max, rng)
    }

    /// Return the axis-aligned bounding box for the object
    #[must_use]
    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<&AABB> {
        Some(&self.aabb)
    }

    /// Returns a probability density function value based on the object
    #[must_use]
    fn pdf_value(
        &self,
        origin: Vec3,
        vector: Vec3,
        wavelength: Wavelength,
        time: Float,
        rng: &mut SmallRng,
    ) -> Float {
        self.bvhnode
            .pdf_value(origin, vector, wavelength, time, rng)
    }

    /// Returns a random point on the ssurface of the object
    #[must_use]
    fn random(&self, origin: Vec3, rng: &mut SmallRng) -> Vec3 {
        self.bvhnode.random(origin, rng)
    }
}

/// STL structure. This gets converted into an internal representation using [Triangles](crate::objects::Triangle)
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
pub struct STLInit {
    /// Used for multiple importance sampling
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub priority: bool,
    /// Path of the .stl file
    pub path: String,
    /// Material to use for the .stl object
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub material: MaterialInit,
    /// Scaling factor for the object
    pub scale: Float,
    /// Location of the object in the rendered scene
    pub center: Vec3,
    /// Rotation of the object. Described as three angles, `roll`, `pitch`, `yaw`, applied in that order.
    pub rotation: Vec3,
}

#[must_use]
/// Initializes an STL
pub fn initialize_stl<'scene>(
    stl_init: STLInit,
    materials: &'scene [SharedMaterial],
) -> STL<'scene> {
    // TODO: error handling!
    let mut file = OpenOptions::new()
        .read(true)
        .open(stl_init.path.clone())
        .unwrap();
    let mesh = stl_io::read_stl(&mut file).unwrap();
    let triangles = mesh.vertices;
    let mut hitable_list = Vec::new();
    let material: &Material = match stl_init.material {
        MaterialInit::Shared(name) => &materials.iter().find(|m| m.name == name).unwrap().material,
        MaterialInit::Owned(m) => {
            // TODO: do not leak memory
            let material: &'scene Material = Box::leak(Box::new(m.into()));
            material
        }
    };

    for face in mesh.faces {
        // TODO: verify if this is the correct order / makes sense / gets correct directions and normals
        let a = triangles[face.vertices[0]];
        let b = triangles[face.vertices[1]];
        let c = triangles[face.vertices[2]];
        // TODO: better conversion between library format and own format
        let a: Vec3 = Vec3::new(a[0], a[1], a[2]);
        let b: Vec3 = Vec3::new(b[0], b[1], b[2]);
        let c: Vec3 = Vec3::new(c[0], c[1], c[2]);
        // Handle rotation
        let rotation = Rotation3::from_euler_angles(
            stl_init.rotation[0].to_radians(),
            stl_init.rotation[1].to_radians(),
            stl_init.rotation[2].to_radians(),
        );
        let a: Vec3 = rotation * a;
        let b: Vec3 = rotation * b;
        let c: Vec3 = rotation * c;
        // Handle scaling and offset
        let a: Vec3 = a * stl_init.scale + stl_init.center;
        let b: Vec3 = b * stl_init.scale + stl_init.center;
        let c: Vec3 = c * stl_init.scale + stl_init.center;

        let triangle = Triangle::from_coordinates(a, b, c, material);
        hitable_list.push(Hitable::Triangle(triangle));
    }

    // TODO: time
    let time_0 = 0.0;
    let time_1 = 1.0;

    let bvhnode = BVHNode::from_list(hitable_list, time_0, time_1);
    // TODO: remove unwrap
    let aabb = bvhnode.bounding_box(time_0, time_1).unwrap().clone();

    STL {
        bvhnode,
        material,
        aabb,
    }
}
