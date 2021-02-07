//! Various literal objects and meta-object utilities for creating content in [Scenes](crate::scenes::Scene).

use crate::hitable::Hitable;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub mod boxy; // avoid keyword
pub mod constant_medium;
pub mod flip_face;
pub mod moving_sphere;
pub mod rect;
pub mod rotate;
pub mod sphere;
pub mod translate;

pub use boxy::*; // avoid keyword
pub use constant_medium::*;
pub use flip_face::*;
pub use moving_sphere::*;
pub use rect::*;
pub use rotate::*;
pub use sphere::*;
pub use translate::*;

// TODO: This is kind of an ugly hack, having to double-implement various structures to have an external representation vs internal representation. How could this be made cleaner?

#[derive(Deserialize, Serialize, Debug)]
pub enum Object {
    XZRect(XZRectInit),
    XYRect(XYRectInit),
    YZRect(YZRectInit),
    Sphere(SphereInit),
    Boxy(BoxyInit),
    RotateY(RotateInit),
    Translate(TranslateInit),
    FlipFace(FlipFaceInit),
    ConstantMedium(ConstantMediumInit),
}

impl From<Object> for Hitable {
    fn from(obj: Object) -> Hitable {
        match obj {
            Object::XZRect(x) => XZRect::new(x.x0, x.x1, x.z0, x.z1, x.k, x.material),
            Object::XYRect(x) => XYRect::new(x.x0, x.x1, x.y0, x.y1, x.k, x.material),
            Object::YZRect(x) => YZRect::new(x.y0, x.y1, x.z0, x.z1, x.k, x.material),
            Object::Sphere(x) => Sphere::new(x.center, x.radius, x.material),
            Object::Boxy(x) => Boxy::new(x.corner_0, x.corner_1, x.material),
            Object::RotateY(x) => {
                let obj = *x.object;
                let obj: Hitable = obj.into();
                RotateY::new(Arc::new(obj), x.angle)
            }
            Object::Translate(x) => {
                let obj = *x.object;
                let obj: Hitable = obj.into();
                Translate::new(Arc::new(obj), x.offset)
            }
            Object::FlipFace(x) => {
                let obj = *x.object;
                let obj: Hitable = obj.into();
                FlipFace::new(obj)
            }
            Object::ConstantMedium(x) => {
                let obj = *x.boundary;
                let obj: Hitable = obj.into();
                ConstantMedium::new(Arc::new(obj), x.density, x.texture)
            }
        }
    }
}
