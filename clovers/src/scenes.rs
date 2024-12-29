//! A collection of objects, camera, and other things necessary to describe the environment you wish to render.

use crate::{bvh::BVHNode, camera::Camera, hitable::Hitable};

use palette::{white_point::E, Xyz};

#[derive(Debug)]
/// A representation of the scene that is being rendered.
pub struct Scene<'scene> {
    /// Bounding-volume hierarchy of [Hitable] objects in the scene. This could, as currently written, be any [Hitable] - in practice, we place the root of the [`BVHNode`] tree here.
    pub bvh_root: BVHNode<'scene>,
    /// The camera object used for rendering the scene.
    pub camera: Camera,
    /// The background color to use when the rays do not hit anything in the scene.
    pub background: Xyz<E>, // TODO: add support for environment maps / HDRI / skyboxes
    /// A [`BVHNode`] tree of priority objects - e.g. glass items or lights - for multiple importance sampling. Wrapped into a [Hitable] for convenience reasons (see various PDF functions).
    pub mis_bvh_root: Hitable<'scene>,
}
