use crate::{
    color::Color,
    hitable::{Hitable, HitableList},
    materials::{DiffuseLight, Lambertian},
    objects::XZRect,
    pdf::{CosinePDF, HitablePDF, MixturePDF},
    ray::Ray,
    textures::SolidColor,
    Float, Vec3, SHADOW_EPSILON,
};
use rand::prelude::*;
use std::sync::Arc;

/// The main coloring function
pub fn colorize(
    ray: &Ray,
    background_color: Color,
    world: &Hitable,
    lights: Arc<Hitable>, // NOTE: possibly hitablelist, or bvhnode, or something new?
    depth: u32,
    max_depth: u32,
    rng: ThreadRng,
) -> Color {
    if depth > max_depth {
        // Ray bounce limit reached, return background_color
        return background_color;
    }

    // Here, smoothing is used to avoid "shadow acne"
    match world.hit(&ray, SHADOW_EPSILON, Float::MAX, rng) {
        // If the ray hits nothing, return the background color.
        None => background_color,

        // Hit something
        Some(hit_record) => {
            let emitted: Color = hit_record.material.emit(
                ray,
                &hit_record,
                hit_record.u,
                hit_record.v,
                hit_record.position,
            );

            // Do we scatter?
            match hit_record.material.scatter(&ray, &hit_record, rng) {
                // No scatter, emit only
                None => emitted,
                // Got a scatter
                Some(scatter_record) => {
                    match scatter_record.material_type {
                        // If we hit a specular, return a specular ray
                        crate::materials::MaterialType::Specular => {
                            return scatter_record.attenuation
                                * colorize(
                                    &scatter_record.specular_ray.unwrap(), // should always have a ray at this point
                                    background_color,
                                    world,
                                    lights,
                                    depth + 1,
                                    max_depth,
                                    rng,
                                );
                        }
                        crate::materials::MaterialType::Diffuse => {
                            let light_ptr =
                                HitablePDF::new(Arc::clone(&lights), hit_record.position);
                            let mixture_pdf = MixturePDF::new(light_ptr, scatter_record.pdf_ptr);

                            let scattered =
                                Ray::new(hit_record.position, mixture_pdf.generate(rng), ray.time);
                            let pdf_val = mixture_pdf.value(scattered.direction, ray.time, rng);

                            return emitted
                                + scatter_record.attenuation
                                    * hit_record.material.scattering_pdf(
                                        ray,
                                        &hit_record,
                                        &scattered,
                                        rng,
                                    )
                                    * colorize(
                                        &scattered,
                                        background_color,
                                        world,
                                        lights,
                                        depth + 1,
                                        max_depth,
                                        rng,
                                    )
                                    / pdf_val;
                        }
                    }
                }
            }
        } // BACKUP
          // // Hit an object
          // Some(hit_record) => {
          //     let emitted: Color = hit_record.material.emit(
          //         ray,
          //         &hit_record,
          //         hit_record.u,
          //         hit_record.v,
          //         hit_record.position,
          //     );
          //     // Try to scatter and colorize the new ray
          //     match hit_record.material.scatter(&ray, &hit_record, rng) {
          //         // Got a scatter
          //         Some(scatter_record) => {
          //             // If material is specular, cast a specular ray
          //             match scatter_record.material_type {
          //                 crate::materials::MaterialType::Specular => {
          //                     return scatter_record.attenuation
          //                         * colorize(
          //                             &scatter_record.specular_ray.unwrap(),
          //                             background_color,
          //                             world,
          //                             lights,
          //                             depth + 1,
          //                             max_depth,
          //                             rng,
          //                         );
          //                 }
          //                 _ => (),
          //             }

          //             // Compute a probability density function value; where to scatter?

          //             let light_ptr = HitablePDF::new(Arc::clone(&lights), hit_record.position);
          //             let mixture_pdf = MixturePDF::new(light_ptr, scatter_record.pdf_ptr);

          //             let scattered =
          //                 Ray::new(hit_record.position, mixture_pdf.generate(rng), ray.time);
          //             let pdf_val = mixture_pdf.value(scattered.direction, ray.time, rng);

          //             // Recurse; compute the color of the scattered ray
          //             let recurse = colorize(
          //                 &scattered,
          //                 background_color,
          //                 world,
          //                 lights,
          //                 depth + 1,
          //                 max_depth,
          //                 rng,
          //             );

          //             // Put it all together:
          //             let color: Color = emitted
          //                 + scatter_record.attenuation
          //                     * hit_record
          //                         .material
          //                         .scattering_pdf(ray, &hit_record, &scattered, rng)
          //                     * recurse //NOTE:  Should this _really_ be multiplication here?
          //                     / pdf_val;

          //             return color;
          //         }
          //         // No scatter, emit only
          //         None => {
          //             return emitted;
          //         }
          //     }
          // }
          // // Did not hit anything, return the background_color
          // None => {
          //     // DEBUG
          //     // return Color::new(0.3, 0.0, 0.0);
          //     return background_color;
          // }
    }
}
