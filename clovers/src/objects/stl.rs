//! STL utilities

use alloc::string::String;
use nalgebra::Rotation3;
use std::fs::OpenOptions;

use crate::{
    aabb::AABB,
    bvh::build::utils::vec_bounding_box,
    hitable::Hitable,
    materials::{Material, MaterialInit, SharedMaterial},
    objects::Triangle,
    Float, Position, Vec3,
};

/// Internal STL object representation after initialization. Contains the material for all triangles in it to avoid having n copies.
#[derive(Debug, Clone)]
pub struct STL<'scene> {
    /// Primitives of the `STL` object. Most likely a list of `Triangle`s.
    pub hitables: Vec<Hitable<'scene>>,
    /// Material for the object
    pub material: &'scene Material,
    /// Axis-aligned bounding box of the object
    pub aabb: AABB,
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
    pub center: Position,
    /// Rotation of the object. Described as three angles, `roll`, `pitch`, `yaw`, applied in that order.
    pub rotation: Vec3,
}

#[must_use]
/// Initializes an STL
///
/// # Panics
/// This method may panic if the referenced .stl file cannot be opened or if it cannot be parsed.
pub fn initialize_stl<'scene>(
    stl_init: STLInit,
    materials: &'scene [SharedMaterial],
) -> STL<'scene> {
    // TODO: error handling!
    let mut file = OpenOptions::new().read(true).open(stl_init.path).unwrap();
    let mesh = stl_io::read_stl(&mut file).unwrap();
    let triangles = mesh.vertices;
    let mut hitables = Vec::new();
    let material: &Material = match stl_init.material {
        MaterialInit::Shared(name) => &materials.iter().find(|m| m.name == name).unwrap().material,
        MaterialInit::Owned(m) => {
            // TODO: do not leak memory
            let material: &'scene Material = Box::leak(Box::new(m));
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
        hitables.push(Hitable::Triangle(triangle));
    }
    // TODO: remove unwrap
    let aabb = vec_bounding_box(&hitables).unwrap();

    STL {
        hitables,
        material,
        aabb,
    }
}
