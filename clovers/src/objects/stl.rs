//! STL utilities
// TODO: better docs

use alloc::string::String;
use nalgebra::Rotation3;
use std::fs::OpenOptions;

use crate::{hitable::Hitable, materials::Material, objects::Triangle, Float, Vec3};

/// STL structure. This gets converted into an internal representation using [Triangles](crate::objects::Triangle)
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
pub struct STL {
    /// Path of the .stl file
    pub path: String,
    /// Material to use for the .stl object
    pub material: Material,
    /// Scaling factor for the object
    pub scale: Float,
    /// Location of the object in the rendered scene
    pub center: Vec3,
    /// Rotation of the object. Described as three angles, `roll`, `pitch`, `yaw`, applied in that order.
    pub rotation: Vec3,
}

impl From<STL> for Vec<Hitable> {
    fn from(stl: STL) -> Self {
        // TODO: error handling!
        let mut file = OpenOptions::new().read(true).open(stl.path).unwrap();
        let mesh = stl_io::read_stl(&mut file).unwrap();
        let triangles = mesh.vertices;
        let mut hitable_list = Vec::new();
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
                stl.rotation[0].to_radians(),
                stl.rotation[1].to_radians(),
                stl.rotation[2].to_radians(),
            );
            let a: Vec3 = rotation * a;
            let b: Vec3 = rotation * b;
            let c: Vec3 = rotation * c;
            // Handle scaling and offset
            let a: Vec3 = a * stl.scale + stl.center;
            let b: Vec3 = b * stl.scale + stl.center;
            let c: Vec3 = c * stl.scale + stl.center;

            let triangle = Triangle::from_coordinates(a, b, c, stl.material);
            hitable_list.push(Hitable::Triangle(triangle));
        }

        hitable_list
    }
}
