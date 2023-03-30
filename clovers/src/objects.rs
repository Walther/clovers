//! Various literal objects and meta-object utilities for creating content in [Scenes](crate::scenes::Scene).

use crate::{bvhnode::BVHNode, hitable::Hitable, materials::Material, Box};

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
pub struct ObjectList<'scene> {
    /// The encased [Object] list
    pub objects: Vec<Object<'scene>>,
}

#[derive(Clone, Debug)]
/// An object enum. TODO: for ideal clean abstraction, this should be a trait. However, that comes with some additional considerations, including e.g. performance.
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde-derive", serde(tag = "kind"))]
pub enum Object<'scene> {
    /// Boxy object initializer
    Boxy(BoxyInit<'scene>),
    /// ConstantMedium object initializer
    ConstantMedium(ConstantMediumInit<'scene>),
    /// FlipFace object initializer
    FlipFace(FlipFaceInit<'scene>),
    /// MovingSphere object initializer
    MovingSphere(MovingSphereInit<'scene>),
    /// ObjectList object initializer
    ObjectList(ObjectList<'scene>),
    /// Quad object initializer
    Quad(QuadInit<'scene>),
    /// RotateY object initializer
    RotateY(RotateInit<'scene>),
    /// Sphere object initializer
    Sphere(SphereInit<'scene>),
    #[cfg(feature = "stl")]
    /// STL object initializer
    STL(STLInit<'scene>),
    #[cfg(feature = "gl_tf")]
    /// GLTF object initializer
    GLTF(GLTFInit),
    /// Translate object initializer
    Translate(TranslateInit<'scene>),
    /// Triangle object initializer
    Triangle(TriangleInit<'scene>),
}

impl<'scene> From<Object<'scene>> for Hitable<'scene> {
    #[must_use]
    fn from(obj: Object<'scene>) -> Hitable<'scene> {
        match obj {
            Object::Boxy(x) => {
                // TODO: do not leak memory
                let material: &'scene Material = Box::leak(Box::new(x.material));
                Hitable::Boxy(Boxy::new(x.corner_0, x.corner_1, material))
            }
            Object::ConstantMedium(x) => {
                let obj = *x.boundary;
                let obj: Hitable = obj.into();
                Hitable::ConstantMedium(ConstantMedium::new(Box::new(obj), x.density, x.texture))
            }
            Object::FlipFace(x) => {
                let obj = *x.object;
                let obj: Hitable = obj.into();
                Hitable::FlipFace(FlipFace::new(obj))
            }
            Object::MovingSphere(x) => Hitable::MovingSphere(MovingSphere::new(
                // TODO: time
                x.center_0, x.center_1, 0.0, 1.0, x.radius, x.material,
            )),
            Object::ObjectList(x) => {
                let objects: Vec<Hitable> = x
                    .objects
                    .iter()
                    .map(|object| -> Hitable { object.clone().into() })
                    .collect();
                let bvh = BVHNode::from_list(objects, 0.0, 1.0);
                Hitable::BVHNode(bvh)
            }
            Object::Quad(x) => {
                // TODO: do not leak memory
                let material: &'scene Material = Box::leak(Box::new(x.material));
                Hitable::Quad(Quad::new(x.q, x.u, x.v, material))
            }
            Object::RotateY(x) => {
                let obj = *x.object;
                let obj: Hitable = obj.into();
                Hitable::RotateY(RotateY::new(Box::new(obj), x.angle))
            }
            Object::Sphere(x) => Hitable::Sphere(Sphere::new(x.center, x.radius, x.material)),
            #[cfg(feature = "stl")]
            Object::STL(x) => {
                // TODO: time
                // TODO: do not leak memory
                let x: &'scene STLInit = Box::leak(Box::new(x));
                Hitable::STL(STL::new(x, 0.0, 1.0))
            }
            #[cfg(feature = "gl_tf")]
            Object::GLTF(x) => {
                // TODO: time
                Hitable::GLTF(GLTF::new(x, 0.0, 1.0))
            }
            Object::Translate(x) => {
                let obj = *x.object;
                let obj: Hitable = obj.into();
                Hitable::Translate(Translate::new(Box::new(obj), x.offset))
            }
            Object::Triangle(x) => {
                // TODO: do not leak memory
                let material: &'scene Material = Box::leak(Box::new(x.material));
                Hitable::Triangle(Triangle::new(x.q, x.u, x.v, material))
            }
        }
    }
}
