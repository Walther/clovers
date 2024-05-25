//! A collection of objects, camera, and other things necessary to describe the environment you wish to render.

use alloc::boxed::Box;

use crate::{
    bvhnode::BVHNode,
    camera::{Camera, CameraInit},
    hitable::Hitable,
    materials::SharedMaterial,
    objects::{object_to_hitable, Object},
    Float, Vec,
};

use palette::Srgb;
#[cfg(feature = "traces")]
use tracing::info;

#[derive(Debug)]
/// A representation of the scene that is being rendered.
pub struct Scene<'scene> {
    /// Bounding-volume hierarchy of [Hitable] objects in the scene. This could, as currently written, be any [Hitable] - in practice, we place the root of the [`BVHNode`] tree here.
    pub hitables: BVHNode<'scene>,
    /// The camera object used for rendering the scene.
    pub camera: Camera,
    /// The background color to use when the rays do not hit anything in the scene.
    pub background_color: Srgb, // TODO: make into Texture or something?
    /// A [`BVHNode`] tree of prioritized objects - e.g. glass items or lights - that affect the biased sampling of the scene. Wrapped into a [Hitable] for convenience reasons (see various PDF functions).
    pub priority_hitables: Hitable<'scene>,
}

impl<'scene> Scene<'scene> {
    /// Creates a new [Scene] with the given parameters.
    #[must_use]
    pub fn new(
        time_0: Float,
        time_1: Float,
        camera: Camera,
        hitables: Vec<Hitable<'scene>>,
        priority_hitables: Vec<Hitable<'scene>>,
        background_color: Srgb,
    ) -> Scene<'scene> {
        Scene {
            hitables: BVHNode::from_list(hitables, time_0, time_1),
            camera,
            background_color,
            priority_hitables: Hitable::BVHNode(BVHNode::from_list(
                priority_hitables,
                time_0,
                time_1,
            )),
        }
    }
}

// TODO: better naming
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// A serialized representation of a [Scene].
pub struct SceneFile {
    time_0: Float,
    time_1: Float,
    background_color: Srgb,
    camera: CameraInit,
    objects: Vec<Object>,
    #[cfg_attr(feature = "serde-derive", serde(default))]
    materials: Vec<SharedMaterial>,
}

/// Initializes a new [Scene] instance by parsing the contents of a [`SceneFile`] structure and then using those details to construct the [Scene].
#[must_use]
pub fn initialize<'scene>(scene_file: SceneFile, width: u32, height: u32) -> Scene<'scene> {
    let time_0 = scene_file.time_0;
    let time_1 = scene_file.time_1;
    let background_color = scene_file.background_color;

    #[allow(clippy::cast_precision_loss)]
    let camera = Camera::new(
        scene_file.camera.look_from,
        scene_file.camera.look_at,
        scene_file.camera.up,
        scene_file.camera.vertical_fov,
        width as Float / height as Float,
        scene_file.camera.aperture,
        scene_file.camera.focus_distance,
        time_0,
        time_1,
    );
    let mut materials = scene_file.materials;
    materials.push(SharedMaterial::default());
    let materials = Box::leak(Box::new(materials));

    #[cfg(feature = "traces")]
    info!("Creating a flattened list from the objects");
    let mut hitables: Vec<Hitable> = Vec::new();
    let mut priority_hitables: Vec<Hitable> = Vec::new();

    // TODO: this isn't the greatest ergonomics, but it gets the job done for now
    for object in scene_file.objects {
        if match &object {
            Object::Boxy(i) => i.priority,
            Object::ConstantMedium(i) => i.priority,
            Object::MovingSphere(i) => i.priority,
            Object::ObjectList(i) => i.priority,
            Object::Quad(i) => i.priority,
            Object::RotateY(i) => i.priority,
            Object::Sphere(i) => i.priority,
            #[cfg(feature = "stl")]
            Object::STL(i) => i.priority,
            #[cfg(feature = "gl_tf")]
            Object::GLTF(i) => i.priority,
            Object::Translate(i) => i.priority,
            Object::Triangle(i) => i.priority,
        } {
            let hitable = object_to_hitable(object, materials);
            hitables.push(hitable.clone());
            priority_hitables.push(hitable);
        } else {
            let hitable = object_to_hitable(object, materials);
            hitables.push(hitable.clone());
        }
    }

    Scene::new(
        time_0,
        time_1,
        camera,
        hitables,
        priority_hitables,
        background_color,
    )
}
