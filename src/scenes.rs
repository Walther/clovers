use crate::{
    camera::Camera,
    color::Color,
    hitable::{BVHNode, HitableList},
    Float,
};
use rand::prelude::*;

pub mod cornell;
pub mod cornell_flips;
pub mod glass_spheres;
pub mod metal_spheres;
pub mod random_scene;
pub mod simple_light_lambertian;
pub mod simple_light_perlin;
pub mod two_perlin_spheres;
pub mod two_spheres;

pub struct Scene {
    pub world: BVHNode,
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
