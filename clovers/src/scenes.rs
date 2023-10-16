//! A collection of objects, camera, and other things necessary to describe the environment you wish to render.

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
    /// Bounding-volume hierarchy of [Hitable] objects in the scene. This could, as currently written, be any [Hitable] - in practice, we place the root of the [BVHNode] tree here.
    pub objects: BVHNode<'scene>,
    /// The camera object used for rendering the scene.
    pub camera: Camera,
    /// The background color to use when the rays do not hit anything in the scene.
    pub background_color: Srgb, // TODO: make into Texture or something?
    /// A [BVHNode] tree of prioritized objects - e.g. glass items or lights - that affect the biased sampling of the scene. Wrapped into a [Hitable] for convenience reasons (see various PDF functions).
    pub priority_objects: Hitable<'scene>,
}

impl<'scene> Scene<'scene> {
    /// Creates a new [Scene] with the given parameters.
    #[must_use]
    pub fn new(
        time_0: Float,
        time_1: Float,
        camera: Camera,
        objects: Vec<Hitable<'scene>>,
        priority_objects: Vec<Hitable<'scene>>,
        background_color: Srgb,
    ) -> Scene<'scene> {
        Scene {
            objects: BVHNode::from_list(objects, time_0, time_1),
            camera,
            background_color,
            priority_objects: Hitable::BVHNode(BVHNode::from_list(
                priority_objects,
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
    priority_objects: Vec<Object>,
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
    let hitables: Vec<Hitable> = objects_to_hitables(scene_file.objects, materials);
    let priority_objects: Vec<Hitable> =
        objects_to_hitables(scene_file.priority_objects, materials);

    Scene::new(
        time_0,
        time_1,
        camera,
        hitables,
        priority_objects,
        background_color,
    )
}

#[must_use]
fn objects_to_hitables(objects: Vec<Object>, materials: &[SharedMaterial]) -> Vec<Hitable<'_>> {
    let mut hitables = Vec::new();
    for obj in objects {
        hitables.push(object_to_hitable(obj, materials));
    }

    hitables
}
