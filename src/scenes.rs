use crate::{
    camera::Camera,
    color::Color,
    hitable::{Hitable, HitableList},
    Float,
};
use rand::prelude::*;
use serde::{Deserialize, Serialize};

pub mod cornell;
pub mod cornell_book3_final;
pub mod cornell_with_boxes;
pub mod cornell_with_glass_cube;
pub mod cornell_with_smoke;
pub mod cornell_with_sphere;
pub mod cornell_with_subsurface_sphere;
pub mod final_scene;
pub mod glass_spheres;
pub mod metal_spheres;
pub mod random_scene;
pub mod simple_light_lambertian;
pub mod two_spheres;

#[derive(Deserialize, Serialize)]
pub struct Scene {
    pub world: Hitable, // BVHNode
    pub camera: Camera,
    pub background: Color, // TODO: make into Texture or something?
}

impl Scene {
    fn new(
        world: HitableList,
        camera: Camera,
        time_0: Float,
        time_1: Float,
        background: Color,
        rng: ThreadRng,
    ) -> Scene {
        Scene {
            world: world.into_bvh(time_0, time_1, rng),
            camera,
            background,
        }
    }
}
