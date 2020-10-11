use crate::{hitable::Hitable, textures::Texture, Float, Vec3};
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
    XZRect(XZRect),
    XYRect(XYRect),
    YZRect(YZRect),
    Sphere(Sphere),
    Boxy(BoxyInit),
    RotateY(RotateInit),
    Translate(TranslateInit),
    FlipFace(FlipFaceInit),
    ConstantMedium(ConstantMediumInit),
}

impl From<Object> for Hitable {
    fn from(obj: Object) -> Hitable {
        match obj {
            Object::XZRect(x) => Hitable::XZRect(x),
            Object::XYRect(x) => Hitable::XYRect(x),
            Object::YZRect(x) => Hitable::YZRect(x),
            Object::Sphere(x) => Hitable::Sphere(x),
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
                ConstantMedium::new(Arc::new(obj), x.density, *x.texture)
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FlipFaceInit {
    object: Box<Object>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RotateInit {
    object: Box<Object>,
    angle: Float,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TranslateInit {
    object: Box<Object>,
    offset: Vec3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConstantMediumInit {
    boundary: Box<Object>,
    density: Float,
    texture: Box<Texture>,
}
