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
                            // Use a probability density function to figure out where to scatter a new ray
                            let light_ptr =
                                HitablePDF::new(Arc::clone(&lights), hit_record.position);
                            let mixture_pdf = MixturePDF::new(light_ptr, scatter_record.pdf_ptr);

                            let scattered =
                                Ray::new(hit_record.position, mixture_pdf.generate(rng), ray.time);
                            let pdf_val = mixture_pdf.value(scattered.direction, ray.time, rng);

                            // recurse
                            let recurse = colorize(
                                &scattered,
                                background_color,
                                world,
                                lights,
                                depth + 1,
                                max_depth,
                                rng,
                            );

                            // Blend it all together
                            return emitted
                                + scatter_record.attenuation
                                    * hit_record.material.scattering_pdf(
                                        ray,
                                        &hit_record,
                                        &scattered,
                                        rng,
                                    )
                                    * recurse
                                    / pdf_val;
                        }
                    }
                }
            }
        }
    }
}
