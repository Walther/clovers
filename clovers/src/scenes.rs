//! A collection of objects, camera, and other things necessary to describe the environment you wish to render.

use crate::{
    bvhnode::BVHNode,
    camera::{Camera, CameraInit},
    color::Color,
    hitable::{Hitable, HitableList},
    objects::Object,
    Float, Vec,
};
use rand::rngs::SmallRng;
use rand::SeedableRng;

#[derive(Debug)]
/// A representation of the scene that is being rendered.
pub struct Scene {
    /// Bounding-volume hierarchy of [Hitable] objects in the scene. This could, as currently written, be any [Hitable] - in practice, we place the root of the [BVHNode](crate::bvhnode::BVHNode) tree here.
    pub objects: BVHNode,
    /// The camera object used for rendering the scene.
    pub camera: Camera,
    /// The background color to use when the rays do not hit anything in the scene.
    pub background_color: Color, // TODO: make into Texture or something?
    /// A [BVHNode](crate::bvhnode::BVHNode) tree of prioritized objects - e.g. glass items or lights - that affect the biased sampling of the scene.
    pub priority_objects: Hitable,
}

impl Scene {
    /// Creates a new [Scene] with the given parameters.
    pub fn new(
        time_0: Float,
        time_1: Float,
        camera: Camera,
        objects: HitableList,
        priority_objects: HitableList,
        background_color: Color,
        rng: &mut SmallRng,
    ) -> Scene {
        Scene {
            objects: objects.into_bvh(time_0, time_1, rng),
            camera,
            background_color,
            // TODO: bvhnode for priority objects too?
            priority_objects: priority_objects.into_hitable(),
        }
    }
}

// TODO: better naming
#[derive(Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// A serialized representation of a [Scene].
pub struct SceneFile {
    time_0: Float,
    time_1: Float,
    background_color: Color,
    camera: CameraInit,
    objects: Vec<Object>,
    priority_objects: Vec<Object>,
}

/// Initializes a new [Scene] instance by parsing the contents of a [SceneFile] structure and then using those details to construct the [Scene].
pub fn initialize(scene_file: SceneFile, width: u32, height: u32) -> Scene {
    let time_0 = scene_file.time_0;
    let time_1 = scene_file.time_1;
    let mut rng = SmallRng::from_entropy();
    let background_color = scene_file.background_color;
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

    // TODO: enable optimization when the bug has been fixed
    // NOTE: currently breaks the rendering; lots of triangles will be missing from the teapot
    // let hitables = objects_to_flat_hitablelist(scene_file.objects);

    let mut hitables = HitableList::new();
    for obj in scene_file.objects {
        hitables.add(obj.into());
    }

    let mut priority_objects = HitableList::new();
    for obj in scene_file.priority_objects {
        priority_objects.add(obj.into());
    }

    Scene::new(
        time_0,
        time_1,
        camera,
        hitables,
        priority_objects,
        background_color,
        &mut rng,
    )
}

fn _objects_to_flat_hitablelist(objects: Vec<Object>) -> HitableList {
    let mut hitables = HitableList::new();
    for obj in objects {
        match obj {
            // For "list-like" objects, unwrap them to a flat list
            Object::ObjectList(list) => {
                for nested in list {
                    hitables.add(nested.into());
                }
            }
            #[cfg(feature = "stl")]
            Object::STL(s) => {
                let list: HitableList = s.into();
                for nested in list.0 {
                    hitables.add(nested);
                }
            }
            // Plain objects, just add them directly
            _ => {
                hitables.add(obj.into());
            }
        };
    }

    hitables
}
