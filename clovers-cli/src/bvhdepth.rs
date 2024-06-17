//! Alternative rendering method. Visualizes the BVH traversal's depth

use clovers::{ray::Ray, scenes::Scene, Float, EPSILON_SHADOW_ACNE};
use palette::LinSrgb;
use rand::rngs::SmallRng;

#[must_use]
pub fn bvh_depth(ray: &Ray, scene: &Scene, rng: &mut SmallRng) -> LinSrgb {
    let mut depth = 0;
    let depth =
        match scene
            .hitables
            .hit_depthcount(&mut depth, ray, EPSILON_SHADOW_ACNE, Float::MAX, rng)
        {
            (Some(_), depth) => depth,
            (None, depth) => depth,
        };

    bvh_depth_to_color(depth)
}

#[must_use]
pub fn bvh_depth_to_color(depth: usize) -> LinSrgb {
    let depth = depth as Float / 255.0;
    match depth {
        // under 256
        0.0..=1.0 => LinSrgb::new(depth, depth, depth),
        // more than 256
        1.0..=2.0 => LinSrgb::new(0.0, 0.0, 1.0),
        // more than 512
        2.0..=4.0 => LinSrgb::new(1.0, 0.0, 1.0),
        // more than 1024
        4.0.. => LinSrgb::new(1.0, 0.0, 0.0),
        // negative floats
        _ => LinSrgb::new(0.0, 0.0, 0.0),
    }
}
