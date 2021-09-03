//! Various literal objects and meta-object utilities for creating content in [Scenes](crate::scenes::Scene).

use crate::{
    hitable::{Hitable, HitableList},
    Box,
};

pub mod boxy; // avoid keyword
pub mod constant_medium;
pub mod flip_face;
pub mod moving_sphere;
pub mod quad;
pub mod rect;
pub mod rotate;
pub mod sphere;
#[cfg(feature = "stl")]
pub mod stl;
pub mod translate;
pub mod triangle;

use alloc::vec::Vec;
pub use boxy::*; // avoid keyword
pub use constant_medium::*;
pub use flip_face::*;
pub use moving_sphere::*;
pub use quad::*;
pub use rect::*;
pub use rotate::*;
pub use sphere::*;
#[cfg(feature = "stl")]
pub use stl::*;
pub use translate::*;
pub use triangle::*;

// TODO: This is kind of an ugly hack, having to double-implement various structures to have an external representation vs internal representation. How could this be made cleaner?

/// A list of objects. Allows multiple objects to be used e.g. in a Rotate or Translate object as the target.
pub type ObjectList = Vec<Object>;

#[derive(Debug)]
/// An object enum. TODO: for ideal clean abstraction, this should be a trait. However, that comes with some additional considerations, including e.g. performance.
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
pub enum Object {
    /// Boxy object initializer
    Boxy(BoxyInit),
    /// ConstantMedium object initializer
    ConstantMedium(ConstantMediumInit),
    /// FlipFace object initializer
    FlipFace(FlipFaceInit),
    /// MovingSphere object initializer
    MovingSphere(MovingSphere),
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
    STL(STL),
    /// Translate object initializer
    Translate(TranslateInit),
    /// Triangle object initializer
    Triangle(TriangleInit),
    /// XYRect object initializer
    XYRect(XYRectInit),
    /// XZRect object initializer
    XZRect(XZRectInit),
    /// YZRect object initializer
    YZRect(YZRectInit),
}

impl From<Object> for Hitable {
    fn from(obj: Object) -> Hitable {
        match obj {
            Object::Boxy(x) => Hitable::Boxy(Boxy::new(x.corner_0, x.corner_1, x.material)),
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
                x.center_0, x.center_1, x.time_0, x.time_1, x.radius, x.material,
            )),
            Object::ObjectList(x) => {
                // TODO: create a BVHNode instead?
                let mut hitable_list = HitableList::new();
                for obj in x {
                    hitable_list.add(obj.into())
                }
                Hitable::HitableList(hitable_list)
            }
            Object::Quad(x) => Hitable::Quad(Quad::new(x.q, x.u, x.v, x.material)),
            Object::RotateY(x) => {
                let obj = *x.object;
                let obj: Hitable = obj.into();
                Hitable::RotateY(RotateY::new(Box::new(obj), x.angle))
            }
            Object::Sphere(x) => Hitable::Sphere(Sphere::new(x.center, x.radius, x.material)),
            #[cfg(feature = "stl")]
            Object::STL(x) => {
                // TODO: create a BVHNode instead?
                let hitable_list: HitableList = x.into();
                Hitable::HitableList(hitable_list)
            }
            Object::Translate(x) => {
                let obj = *x.object;
                let obj: Hitable = obj.into();
                Hitable::Translate(Translate::new(Box::new(obj), x.offset))
            }
            Object::Triangle(x) => Hitable::Triangle(Triangle::new(x.q, x.u, x.v, x.material)),
            Object::XYRect(x) => {
                Hitable::XYRect(XYRect::new(x.x0, x.x1, x.y0, x.y1, x.k, x.material))
            }
            Object::XZRect(x) => {
                Hitable::XZRect(XZRect::new(x.x0, x.x1, x.z0, x.z1, x.k, x.material))
            }
            Object::YZRect(x) => {
                Hitable::YZRect(YZRect::new(x.y0, x.y1, x.z0, x.z1, x.k, x.material))
            }
        }
    }
}
