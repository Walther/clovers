use crate::{color::Color, hitable::Hitable, ray::Ray, Float, SHADOW_EPSILON};
use rand::prelude::ThreadRng;

/// The main coloring function
pub fn colorize(
    ray: &Ray,
    background_color: Color,
    world: &dyn Hitable,
    depth: u32,
    max_depth: u32,
    rng: ThreadRng,
) -> Color {
    let color: Color;

    if depth > max_depth {
        // Ray bounce limit reached, return background_color
        return background_color;
    }

    // Here, smoothing is used to avoid "shadow acne"
    match world.hit(&ray, SHADOW_EPSILON, Float::MAX, rng) {
        // Hit an object
        Some(hit_record) => {
            let emitted: Color =
                hit_record
                    .material
                    .emitted(hit_record.u, hit_record.v, hit_record.position);
            // Try to scatter and colorize the new ray
            match hit_record.material.scatter(&ray, &hit_record, rng) {
                // Got a scatter and attenuation
                Some((scattered, attenuation)) => {
                    color = emitted
                        + attenuation.component_mul(
                            // Recurse
                            &colorize(
                                &scattered,
                                background_color,
                                world,
                                depth + 1,
                                max_depth,
                                rng,
                            ),
                        );

                    return color;
                }
                // No scatter, emit only
                None => {
                    return emitted;
                }
            }
        }
        // Did not hit anything, return the background_color
        None => {
            // DEBUG
            // return Color::new(0.3, 0.0, 0.0);
            return background_color;
        }
    }
}
