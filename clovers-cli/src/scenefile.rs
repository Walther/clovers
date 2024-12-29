use std::boxed::Box;

use clovers::{
    bvh::{BVHNode, BvhAlgorithm},
    camera::{Camera, CameraInit},
    hitable::Hitable,
    materials::SharedMaterial,
    objects::{object_to_hitable, Object},
    scenes::Scene,
    Float, Vec,
};

use palette::{
    chromatic_adaptation::AdaptInto, convert::IntoColorUnclamped, white_point::E, Srgb, Xyz,
};
use tracing::info;

// TODO: better naming
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
/// A serialized representation of a [Scene].
pub struct SceneFile {
    time_0: Float,
    time_1: Float,
    background_color: Srgb,
    camera: CameraInit,
    objects: Vec<Object>,
    #[serde(default)]
    materials: Vec<SharedMaterial>,
}

impl SceneFile {
    /// Initializes a new [Scene] instance by parsing the contents of a [`SceneFile`] structure and then using those details to construct the [Scene].
    #[must_use]
    pub fn initialize<'scene>(
        scene_file: SceneFile,
        bvh_algorithm: BvhAlgorithm,
        width: u32,
        height: u32,
    ) -> Scene<'scene> {
        let time_0 = scene_file.time_0;
        let time_1 = scene_file.time_1;

        let background: Xyz = scene_file.background_color.into_color_unclamped();
        let background: Xyz<E> = background.adapt_into();

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

        info!("Creating a flattened list from the objects");
        let mut hitables: Vec<Hitable> = Vec::new();
        let mut priority_hitables: Vec<Hitable> = Vec::new();

        // TODO: this isn't the greatest ergonomics, but it gets the job done for now
        for object in scene_file.objects {
            let priority = match &object {
                Object::Boxy(i) => i.priority,
                Object::ConstantMedium(i) => i.priority,
                Object::MovingSphere(i) => i.priority,
                Object::ObjectList(i) => i.priority,
                Object::Quad(i) => i.priority,
                Object::RotateY(i) => i.priority,
                Object::Sphere(i) => i.priority,
                Object::STL(i) => i.priority,
                Object::PLY(i) => i.priority,
                Object::GLTF(i) => i.priority,
                Object::Translate(i) => i.priority,
                Object::Triangle(i) => i.priority,
            };
            let hitable = object_to_hitable(object, materials);

            match hitable {
                // Flatten any lists we got. Potential sources: `GLTF`, `STL`, `ObjectList`
                Hitable::HitableList(l) => {
                    let flattened = &mut l.flatten();
                    hitables.append(&mut flattened.clone());
                    if priority {
                        priority_hitables.append(&mut flattened.clone());
                    }
                }
                // Otherwise, push as-is
                _ => {
                    hitables.push(hitable.clone());
                    if priority {
                        priority_hitables.push(hitable.clone());
                    }
                }
            };
        }
        info!("All objects parsed into hitables");
        info!("Building the BVH root for hitables");
        let bvh_root = BVHNode::from_list(bvh_algorithm, hitables);
        info!("Building the MIS BVH root for priority hitables");
        let mis_bvh_root = Hitable::BVHNode(BVHNode::from_list(bvh_algorithm, priority_hitables));
        info!("BVH root nodes built");

        Scene {
            camera,
            bvh_root,
            mis_bvh_root,
            background,
        }
    }
}
