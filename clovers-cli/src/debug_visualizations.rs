//! Alternative rendering methods for debug visualization purposes.

use clovers::{ray::Ray, scenes::Scene, Float, EPSILON_SHADOW_ACNE};
use palette::{chromatic_adaptation::AdaptInto, white_point::E, LinSrgb, Xyz};
use rand::rngs::SmallRng;

/// Visualizes the BVH traversal count - how many BVH nodes needed to be tested for intersection?
#[must_use]
pub fn bvh_testcount(ray: &Ray, scene: &Scene, rng: &mut SmallRng) -> Xyz<E> {
    let mut depth = 0;
    scene
        .bvh_root
        .testcount(&mut depth, ray, EPSILON_SHADOW_ACNE, Float::MAX, rng);

    bvh_testcount_to_color(depth)
}

#[must_use]
pub fn bvh_testcount_to_color(depth: usize) -> Xyz<E> {
    let color: LinSrgb = match depth {
        // under 256, grayscale
        0..=255 => {
            let depth = depth as Float / 255.0;
            LinSrgb::new(depth, depth, depth)
        }
        // more than 256, yellow
        256..=511 => LinSrgb::new(1.0, 1.0, 0.0),
        // more than 512, orange
        512..=1023 => LinSrgb::new(1.0, 0.5, 0.0),
        // more than 1024, red
        1024.. => LinSrgb::new(1.0, 0.0, 0.0),
    };

    color.adapt_into()
}

/// Visualizes the primitive traversal count - how many primitives needed to be tested for intersection?
#[must_use]
pub fn primitive_testcount(ray: &Ray, scene: &Scene, rng: &mut SmallRng) -> Xyz<E> {
    let mut depth = 0;
    scene
        .bvh_root
        .primitive_testcount(&mut depth, ray, EPSILON_SHADOW_ACNE, Float::MAX, rng);

    primitive_testcount_to_color(depth)
}

#[must_use]
pub fn primitive_testcount_to_color(depth: usize) -> Xyz<E> {
    let color: LinSrgb = match depth {
        // under 256, grayscale
        0..=255 => {
            let depth = depth as Float / 255.0;
            LinSrgb::new(depth, depth, depth)
        }
        // more than 256, yellow
        256..=511 => LinSrgb::new(1.0, 1.0, 0.0),
        // more than 512, orange
        512..=1023 => LinSrgb::new(1.0, 0.5, 0.0),
        // more than 1024, red
        1024.. => LinSrgb::new(1.0, 0.0, 0.0),
    };

    color.adapt_into()
}
