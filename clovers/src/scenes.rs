//! A collection of objects, camera, and other things necessary to describe the environment you wish to render.

use crate::{
    camera::{Camera, CameraInit},
    color::Color,
    hitable::{Hitable, HitableList},
    objects::Object,
    Float,
};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

// TODO: convert these to json or other
// pub mod cornell;
// pub mod cornell_book3_final;
// pub mod cornell_with_boxes;
// pub mod cornell_with_glass_cube;
// pub mod cornell_with_smoke;
// pub mod cornell_with_sphere;
// pub mod cornell_with_subsurface_sphere;
// pub mod final_scene;
// pub mod glass_spheres;
// pub mod metal_spheres;
// pub mod random_scene;
// pub mod simple_light_lambertian;
// pub mod two_spheres;

pub struct Scene {
    pub objects: Hitable, // BVHNode
    pub camera: Camera,
    pub background_color: Color, // TODO: make into Texture or something?
    pub priority_objects: Hitable,
}

impl Scene {
    pub fn new(
        time_0: Float,
        time_1: Float,
        camera: Camera,
        objects: HitableList,
        priority_objects: HitableList,
        background_color: Color,
        rng: ThreadRng,
    ) -> Scene {
        Scene {
            objects: objects.into_bvh(time_0, time_1, rng),
            camera,
            background_color,
            priority_objects: priority_objects.into_hitable(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SceneFile {
    time_0: Float,
    time_1: Float,
    background_color: Color,
    camera: CameraInit,
    objects: Vec<Object>,
    priority_objects: Vec<Object>,
}

pub fn initialize(mut file: File, width: u32, height: u32) -> Result<Scene, std::io::Error> {
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    let scene_file: SceneFile = serde_json::from_str(&contents)?;
    let time_0 = scene_file.time_0;
    let time_1 = scene_file.time_1;
    let rng = thread_rng();
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
    let mut hitables = HitableList::new();
    for obj in scene_file.objects {
        hitables.add(obj.into());
    }

    let mut priority_objects = HitableList::new();
    for obj in scene_file.priority_objects {
        priority_objects.add(obj.into());
    }

    Ok(Scene::new(
        time_0,
        time_1,
        camera,
        hitables,
        priority_objects,
        background_color,
        rng,
    ))
}
