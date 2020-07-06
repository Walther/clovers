use crate::{
    color::Color, hitable::Hitable, pdf::CosinePDF, ray::Ray, Float, Vec3, SHADOW_EPSILON,
};
use rand::prelude::*;

/// The main coloring function
pub fn colorize(
    ray: &Ray,
    background_color: Color,
    world: &Hitable,
    depth: u32,
    max_depth: u32,
    mut rng: ThreadRng,
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
            let emitted: Color = hit_record.material.emit(
                &hit_record,
                hit_record.u,
                hit_record.v,
                hit_record.position,
            );
            // Try to scatter and colorize the new ray
            match hit_record.material.scatter(&ray, &hit_record, rng) {
                // Got a scatter, albedo and pdf value
                Some((scattered, albedo, pdf)) => {
                    // Compute a probability density function value; where to scatter?
                    let cosine_pdf = CosinePDF::new(hit_record.normal);
                    let scattered =
                        Ray::new(hit_record.position, cosine_pdf.generate(rng), ray.time);
                    let pdf_val = cosine_pdf.value(scattered.direction);

                    // color = emitted + albedo * scatter_pdf * recurse / pdf
                    color = emitted
                        + (albedo
                            * hit_record.material.scattering_pdf(
                                ray,
                                &hit_record,
                                &scattered,
                                rng,
                            ))
                        .component_mul(
                            // Recurse
                            &colorize(
                                &scattered,
                                background_color,
                                world,
                                depth + 1,
                                max_depth,
                                rng,
                            ),
                        ) / pdf_val;

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
