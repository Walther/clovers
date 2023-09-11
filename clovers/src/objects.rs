//! Various literal objects and meta-object utilities for creating content in [Scenes](crate::scenes::Scene).

use crate::{
    bvhnode::BVHNode,
    hitable::Hitable,
    materials::{Material, MaterialInit, SharedMaterial},
    Box,
};

pub mod boxy; // avoid keyword
pub mod constant_medium;
pub mod flip_face;
#[cfg(feature = "gl_tf")]
pub mod gltf;
pub mod moving_sphere;
pub mod quad;
pub mod rotate;
pub mod sphere;
#[cfg(feature = "stl")]
pub mod stl;
pub mod translate;
pub mod triangle;

#[cfg(feature = "gl_tf")]
pub use self::gltf::*;
use alloc::vec::Vec;
pub use boxy::*; // avoid keyword
pub use constant_medium::*;
pub use flip_face::*;
pub use moving_sphere::*;
pub use quad::*;
pub use rotate::*;
pub use sphere::*;
#[cfg(feature = "stl")]
pub use stl::*;
pub use translate::*;
pub use triangle::*;

// TODO: This is kind of an ugly hack, having to double-implement various structures to have an external representation vs internal representation. How could this be made cleaner?

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// A list of objects. Allows multiple objects to be used e.g. in a Rotate or Translate object as the target.
pub struct ObjectList {
    /// The encased [Object] list
    pub objects: Vec<Object>,
}

#[derive(Clone, Debug)]
/// An object enum. TODO: for ideal clean abstraction, this should be a trait. However, that comes with some additional considerations, including e.g. performance.
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde-derive", serde(tag = "kind"))]
pub enum Object {
    /// Boxy object initializer
    Boxy(BoxyInit),
    /// ConstantMedium object initializer
    ConstantMedium(ConstantMediumInit),
    /// FlipFace object initializer
    FlipFace(FlipFaceInit),
    /// MovingSphere object initializer
    MovingSphere(MovingSphereInit),
    /// ObjectList object initializer
    ObjectList(ObjectList),
    /// Quad object initializer
    Quad(QuadInit),
    /// RotateY object initializer
    RotateY(RotateInit),
    /// Sphere object initializer
    Sphere(SphereInit),
    #[cfg(feature = "stl")]
    /// STL object initializer
    STL(STLInit),
    #[cfg(feature = "gl_tf")]
    /// GLTF object initializer
    GLTF(GLTFInit),
    /// Translate object initializer
    Translate(TranslateInit),
    /// Triangle object initializer
    Triangle(TriangleInit),
}

#[must_use]
/// Initializes an `Object` into a `Hitable`.
pub fn object_to_hitable<'scene>(
    obj: Object,
    materials: &'scene [SharedMaterial],
) -> Hitable<'scene> {
    // TODO: reduce repetition!

    match obj {
        Object::Boxy(x) => {
            let material = initialize_material(x.material, materials);
            Hitable::Boxy(Boxy::new(x.corner_0, x.corner_1, material))
        }
        Object::ConstantMedium(x) => {
            let obj = *x.boundary;
            let obj: Hitable = object_to_hitable(obj, materials);
            Hitable::ConstantMedium(ConstantMedium::new(Box::new(obj), x.density, x.texture))
        }
        Object::FlipFace(x) => {
            let obj = *x.object;
            let obj: Hitable = object_to_hitable(obj, materials);
            Hitable::FlipFace(FlipFace::new(obj))
        }
        Object::MovingSphere(x) => {
            let material = initialize_material(x.material, materials);
            Hitable::MovingSphere(MovingSphere::new(
                // TODO: time
                x.center_0, x.center_1, 0.0, 1.0, x.radius, material,
            ))
        }
        Object::ObjectList(x) => {
            let objects: Vec<Hitable> = x
                .objects
                .iter()
                .map(|object| -> Hitable { object_to_hitable(object.clone(), materials) })
                .collect();
            let bvh = BVHNode::from_list(objects, 0.0, 1.0);
            Hitable::BVHNode(bvh)
        }
        Object::Quad(x) => {
            let material: &Material = match x.material {
                MaterialInit::Owned(m) => {
                    // TODO: do not leak memory
                    let material: &'scene Material = Box::leak(Box::new(m));
                    material
                }
                MaterialInit::Shared(name) => {
                    &materials.iter().find(|m| m.name == name).unwrap().material
                }
            };
            Hitable::Quad(Quad::new(x.q, x.u, x.v, material))
        }
        Object::RotateY(x) => {
            let obj = *x.object;
            let obj: Hitable = object_to_hitable(obj, materials);
            Hitable::RotateY(RotateY::new(Box::new(obj), x.angle))
        }
        Object::Sphere(x) => {
            let material = initialize_material(x.material, materials);
            Hitable::Sphere(Sphere::new(x.center, x.radius, material))
        }
        #[cfg(feature = "stl")]
        Object::STL(stl_init) => Hitable::STL(initialize_stl(stl_init, materials)),
        #[cfg(feature = "gl_tf")]
        Object::GLTF(x) => {
            // TODO: time
            Hitable::GLTF(GLTF::new(x, 0.0, 1.0))
        }
        Object::Translate(x) => {
            let obj = *x.object;
            let obj: Hitable = object_to_hitable(obj, materials);
            Hitable::Translate(Translate::new(Box::new(obj), x.offset))
        }
        Object::Triangle(x) => {
            let material = initialize_material(x.material, materials);
            Hitable::Triangle(Triangle::new(x.q, x.u, x.v, material))
        }
    }
}

fn initialize_material<'scene>(
    material_init: MaterialInit,
    materials: &'scene [SharedMaterial],
) -> &Material {
    let material: &Material = match material_init {
        MaterialInit::Owned(m) => {
            // TODO: do not leak memory
            let material: &'scene Material = Box::leak(Box::new(m));
            material
        }
        MaterialInit::Shared(name) => &materials.iter().find(|m| m.name == name).unwrap().material,
    };
    material
}
