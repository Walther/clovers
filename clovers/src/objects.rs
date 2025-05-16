//! Various literal objects and meta-object utilities for creating content in [Scenes](crate::scenes::Scene).

use crate::{
    Box,
    hitable::{Hitable, HitableList},
    materials::{Material, MaterialInit, SharedMaterial},
};

pub mod boxy; // avoid keyword
pub mod constant_medium;
#[cfg(feature = "gl_tf")]
pub mod gltf;
pub mod moving_sphere;
#[cfg(feature = "ply")]
pub mod ply;
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
pub use moving_sphere::*;
#[cfg(feature = "ply")]
pub use ply::*;
pub use quad::*;
pub use rotate::*;
pub use sphere::*;
#[cfg(feature = "stl")]
pub use stl::*;
#[cfg(feature = "traces")]
use tracing::warn;
pub use translate::*;
pub use triangle::*;

// TODO: This is kind of an ugly hack, having to double-implement various structures to have an external representation vs internal representation. How could this be made cleaner?

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// A list of objects. Allows multiple objects to be used e.g. in a Rotate or Translate object as the target.
pub struct ObjectList {
    /// Priority
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub priority: bool,
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
    /// `ConstantMedium` object initializer
    ConstantMedium(ConstantMediumInit),
    /// `MovingSphere` object initializer
    MovingSphere(MovingSphereInit),
    /// `ObjectList` object initializer
    ObjectList(ObjectList),
    /// Quad object initializer
    Quad(QuadInit),
    /// `RotateY` object initializer
    RotateY(RotateInit),
    /// Sphere object initializer
    Sphere(SphereInit),
    #[cfg(feature = "stl")]
    /// STL object initializer
    STL(STLInit),
    #[cfg(feature = "ply")]
    /// PLY object initializer
    PLY(PLYInit),
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
pub fn object_to_hitable(obj: Object, materials: &[SharedMaterial]) -> Hitable<'_> {
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
                .into_iter()
                .map(|object| -> Hitable { object_to_hitable(object, materials) })
                .collect();
            Hitable::HitableList(HitableList::new(objects))
        }
        Object::Quad(x) => {
            let material = initialize_material(x.material, materials);
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
        Object::STL(stl_init) => {
            let stl = initialize_stl(stl_init, materials);
            Hitable::HitableList(HitableList::new(stl.hitables))
        }
        #[cfg(feature = "ply")]
        Object::PLY(ply_init) => {
            let ply = initialize_ply(ply_init, materials);
            Hitable::HitableList(HitableList::new(ply.hitables))
        }
        #[cfg(feature = "gl_tf")]
        Object::GLTF(x) => {
            let gltf = GLTF::new(x);
            Hitable::HitableList(HitableList::new(gltf.hitables))
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
) -> &'scene Material {
    let material: &Material = match material_init {
        MaterialInit::Owned(m) => {
            // TODO: do not leak memory
            let material: &'scene Material = Box::leak(Box::new(m));
            material
        }
        MaterialInit::Shared(name) => {
            // Find material by name. If the name is not found, use the default material
            if let Some(m) = &materials.iter().find(|m| m.name == name) {
                &m.material
            } else {
                #[cfg(feature = "traces")]
                warn!(
                    "shared material `{}` not found, using default material",
                    name
                );
                &materials
                    .iter()
                    .find(|m| m.name.is_empty())
                    .unwrap()
                    .material
            }
        }
    };
    material
}
