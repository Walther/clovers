use crate::{
    color::Color,
    pdf::{HitablePDF, MixturePDF},
    ray::Ray,
    scenes::Scene,
    Float, SHADOW_EPSILON,
};
use rand::prelude::*;

/// The main coloring function
pub fn colorize(ray: &Ray, scene: &Scene, depth: u32, max_depth: u32, rng: ThreadRng) -> Color {
    if depth > max_depth {
        // Ray bounce limit reached, return background_color
        return scene.background_color;
    }

    // Here, smoothing is used to avoid "shadow acne"
    match scene.objects.hit(&ray, SHADOW_EPSILON, Float::MAX, rng) {
        // If the ray hits nothing, return the background color.
        None => scene.background_color,

        // Hit something
        Some(hit_record) => {
            let mut emitted: Color = hit_record.material.emit(
                ray,
                &hit_record,
                hit_record.u,
                hit_record.v,
                hit_record.position,
            );

            if emitted.r.is_nan() || emitted.g.is_nan() || emitted.b.is_nan() {
                // TODO: figure out the source
                eprintln!("an emitted component was NaN; skipping");
                emitted = Color::new(0.0, 0.0, 0.0);
            }

            // Do we scatter?
            match hit_record.material.scatter(&ray, &hit_record, rng) {
                // No scatter, emit only
                None => emitted,
                // Got a scatter
                Some(scatter_record) => {
                    match scatter_record.material_type {
                        // If we hit a specular, return a specular ray
                        crate::materials::MaterialType::Specular => {
                            scatter_record.attenuation
                                * colorize(
                                    &scatter_record.specular_ray.unwrap(), // should always have a ray at this point
                                    scene,
                                    depth + 1,
                                    max_depth,
                                    rng,
                                )
                        }
                        crate::materials::MaterialType::Diffuse => {
                            // Use a probability density function to figure out where to scatter a new ray
                            let light_ptr =
                                HitablePDF::new(&scene.priority_objects, hit_record.position);
                            let mixture_pdf = MixturePDF::new(light_ptr, scatter_record.pdf_ptr);

                            let scattered =
                                Ray::new(hit_record.position, mixture_pdf.generate(rng), ray.time);
                            let mut pdf_val = mixture_pdf.value(scattered.direction, ray.time, rng);

                            if pdf_val == 0.0 {
                                // TODO: figure out the source
                                eprintln!("a pdf_val was zero, would divzero; skipping");
                                pdf_val = 1.0;
                            }

                            if pdf_val.is_nan() || pdf_val.is_nan() || pdf_val.is_nan() {
                                // TODO: figure out the source
                                eprintln!("a pdf_val was NaN; skipping");
                                pdf_val = 1.0;
                            }

                            // recurse
                            let mut recurse =
                                colorize(&scattered, scene, depth + 1, max_depth, rng);

                            if recurse.r.is_nan() || recurse.g.is_nan() || recurse.b.is_nan() {
                                // TODO: figure out the source
                                eprintln!("a recurse component was NaN; skipping");
                                recurse = Color::new(0.0, 0.0, 0.0);
                            }

                            // Blend it all together
                            let color = emitted
                                + scatter_record.attenuation
                                    * hit_record.material.scattering_pdf(
                                        ray,
                                        &hit_record,
                                        &scattered,
                                        rng,
                                    )
                                    * recurse
                                    / pdf_val;

                            if color.r.is_nan() || color.g.is_nan() || color.b.is_nan() {
                                // TODO: figure out the source
                                eprintln!("a color component was NaN; skipping");
                                return Color::new(0.0, 0.0, 0.0);
                            }

                            // Return blended color
                            color
                        }
                    }
                }
            }
        }
    }
}
