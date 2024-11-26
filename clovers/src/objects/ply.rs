//! PLY utilities

use alloc::string::String;
use nalgebra::Rotation3;
use ply_rs::{
    parser::Parser,
    ply::{self, Property},
};

use crate::{
    aabb::AABB,
    bvh::build::utils::vec_bounding_box,
    hitable::Hitable,
    materials::{Material, MaterialInit, SharedMaterial},
    objects::Triangle,
    Float, Position, Vec3,
};

/// Internal PLY object representation after initialization. Contains the material for all triangles in it to avoid having n copies.
#[derive(Debug, Clone)]
pub struct PLY<'scene> {
    /// Primitives of the `PLY` object, a list of `Triangle`s.
    pub hitables: Vec<Hitable<'scene>>,
    /// Material for the object
    pub material: &'scene Material,
    /// Axis-aligned bounding box of the object
    pub aabb: AABB,
}

/// PLY structure. This gets converted into an internal representation using [Triangles](crate::objects::Triangle)
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
pub struct PLYInit {
    /// Used for multiple importance sampling
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub priority: bool,
    /// Path of the .ply file
    pub path: String,
    /// Material to use for the .ply object
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
/// Initializes a PLY
///
/// # Panics
/// This method may panic if a shared material is referenced in the object description, but cannot be found in the materials list.
pub fn initialize_ply<'scene>(
    ply_init: PLYInit,
    materials: &'scene [SharedMaterial],
) -> PLY<'scene> {
    let material: &Material = match ply_init.material {
        MaterialInit::Shared(name) => &materials.iter().find(|m| m.name == name).unwrap().material,
        MaterialInit::Owned(m) => {
            // TODO: do not leak memory
            let material: &'scene Material = Box::leak(Box::new(m));
            material
        }
    };
    let mut hitables = Vec::new();

    // TODO: error handling!
    let mut f = std::fs::File::open(ply_init.path).unwrap();
    let parser = Parser::<ply::DefaultElement>::new();
    let ply = parser.read_ply(&mut f);
    let ply = ply.unwrap();

    let mut vertices = Vec::new();
    for vertex in &ply.payload["vertex"] {
        let x = property_to_float(&vertex["x"]);
        let y = property_to_float(&vertex["y"]);
        let z = property_to_float(&vertex["z"]);
        vertices.push(Vec3::new(x, y, z));
    }

    for face in &ply.payload["face"] {
        let indices = property_to_vec_u32(&face["vertex_indices"]);

        let a = vertices[indices[0]];
        let b = vertices[indices[1]];
        let c = vertices[indices[2]];

        let a: Vec3 = Vec3::new(a[0], a[1], a[2]);
        let b: Vec3 = Vec3::new(b[0], b[1], b[2]);
        let c: Vec3 = Vec3::new(c[0], c[1], c[2]);
        // Handle rotation
        let rotation = Rotation3::from_euler_angles(
            ply_init.rotation[0].to_radians(),
            ply_init.rotation[1].to_radians(),
            ply_init.rotation[2].to_radians(),
        );
        let a: Vec3 = rotation * a;
        let b: Vec3 = rotation * b;
        let c: Vec3 = rotation * c;
        // Handle scaling and offset
        let a: Vec3 = a * ply_init.scale + ply_init.center;
        let b: Vec3 = b * ply_init.scale + ply_init.center;
        let c: Vec3 = c * ply_init.scale + ply_init.center;

        let triangle = Triangle::from_coordinates(a, b, c, material);
        hitables.push(Hitable::Triangle(triangle));
    }
    // TODO: remove unwrap
    let aabb = vec_bounding_box(&hitables).unwrap();

    PLY {
        hitables,
        material,
        aabb,
    }
}

// TODO: better ergonomics?
#[allow(trivial_numeric_casts)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
fn property_to_float(p: &Property) -> Float {
    match *p {
        Property::Int(i) => i as Float,
        Property::UInt(u) => u as Float,
        Property::Float(f) => f as Float,
        Property::Double(f) => f as Float,
        // Unsupported
        Property::Char(_)
        | Property::UChar(_)
        | Property::Short(_)
        | Property::UShort(_)
        | Property::ListChar(_)
        | Property::ListUChar(_)
        | Property::ListShort(_)
        | Property::ListUShort(_)
        | Property::ListInt(_)
        | Property::ListUInt(_)
        | Property::ListFloat(_)
        | Property::ListDouble(_) => unimplemented!("PLY: unsupported property format {p:?}"),
    }
}

// TODO: better ergonomics?
#[allow(trivial_numeric_casts)]
fn property_to_vec_u32(p: &Property) -> Vec<usize> {
    match p {
        Property::Char(_)
        | Property::UChar(_)
        | Property::Short(_)
        | Property::UShort(_)
        | Property::Int(_)
        | Property::UInt(_)
        | Property::Float(_)
        | Property::Double(_)
        | Property::ListChar(_)
        | Property::ListFloat(_)
        | Property::ListDouble(_) => unimplemented!("PLY: unsupported property format {p:?}"),
        //
        Property::ListUChar(vec_u) => vec_u.iter().map(|&u| u.into()).collect(),
        Property::ListUShort(vec_u) => vec_u.iter().map(|&u| u.into()).collect(),
        Property::ListUInt(vec_u) => vec_u.iter().map(|&u| u as usize).collect(),
        //
        Property::ListShort(vec_i) => vec_i.iter().map(|&i| i.unsigned_abs().into()).collect(),
        Property::ListInt(vec_i) => vec_i.iter().map(|&i| i.unsigned_abs() as usize).collect(),
    }
}
