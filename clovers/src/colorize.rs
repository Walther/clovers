//! An opinionated colorize method. Given a [Ray](crate::ray::Ray) and a [Scene](crate::scenes::Scene), evaluates the ray's path and returns a color.

use crate::{
    color::Color,
    materials::MaterialType,
    pdf::{HitablePDF, MixturePDF, PDF},
    ray::Ray,
    scenes::Scene,
    Float, EPSILON_SHADOW_ACNE,
};
use rand::prelude::*;

/// The main coloring function. Sends a [Ray] to the [Scene], sees if it hits anything, and eventually returns a [Color]. Taking into account the [Material](crate::materials::Material) that is hit, the method recurses with various adjustments, with a new [Ray] started from the location that was hit.
pub fn colorize(ray: &Ray, scene: &Scene, depth: u32, max_depth: u32, rng: ThreadRng) -> Color {
    // Have we reached the maximum recursion i.e. ray bounce depth?
    if depth > max_depth {
        // Ray bounce limit reached, early return background_color
        return scene.background_color;
    }

    // Send the ray to the scene, and see if it hits anything.
    // distance_min is set to an epsilon to avoid "shadow acne" that can happen when set to zero
    let hit_record = match scene.objects.hit(ray, EPSILON_SHADOW_ACNE, Float::MAX, rng) {
        // If the ray hits nothing, early return the background color.
        None => return scene.background_color,
        // Hit something, continue
        Some(hit_record) => hit_record,
    };

    // Get the emitted color from the surface that we just hit
    let emitted: Color = hit_record.material.emit(
        ray,
        &hit_record,
        hit_record.u,
        hit_record.v,
        hit_record.position,
    );

    // Do we scatter?
    let scatter_record = match hit_record.material.scatter(ray, &hit_record, rng) {
        // No scatter, early return the emitted color only
        None => return emitted,
        // Got a scatter, continue
        Some(scatter_record) => scatter_record,
    };

    // We have scattered, check material type and recurse accordingly
    match scatter_record.material_type {
        MaterialType::Specular => {
            // If we hit a specular material, generate a specular ray, and multiply it with the value of the scatter_record.
            // Note that the `emitted` value from earlier is not used, as the scatter_record.attenuation has an appropriately adjusted color
            scatter_record.attenuation
                * colorize(
                    // a scatter_record from a specular material should always have this ray
                    &scatter_record.specular_ray.unwrap(),
                    scene,
                    depth + 1,
                    max_depth,
                    rng,
                )
        }
        MaterialType::Diffuse => {
            // Use a probability density function to figure out where to scatter a new ray
            // TODO: this weighed priority sampling should be adjusted or removed - doesn't feel ideal.
            let light_ptr = PDF::HitablePDF(HitablePDF::new(
                &scene.priority_objects,
                hit_record.position,
            ));
            let mixture_pdf = MixturePDF::new(light_ptr, scatter_record.pdf_ptr);

            let scattered = Ray::new(hit_record.position, mixture_pdf.generate(rng), ray.time);
            let pdf_val = mixture_pdf.value(scattered.direction, ray.time, rng);

            // Recurse
            let recurse = colorize(&scattered, scene, depth + 1, max_depth, rng);

            // Blend it all together
            emitted
                + scatter_record.attenuation
                    * hit_record
                        .material
                        .scattering_pdf(ray, &hit_record, &scattered, rng)
                    * recurse
                    / pdf_val
        }
    }
}
