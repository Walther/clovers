//! An opinionated colorize method. Given a [Ray] and a [Scene], evaluates the ray's path and returns a color.

use crate::{
    hitable::HitableTrait,
    materials::MaterialType,
    pdf::{HitablePDF, MixturePDF, PDFTrait, PDF},
    ray::Ray,
    scenes::Scene,
    spectrum::spectrum_xyz_to_p,
    wavelength::{wavelength_into_xyz, Wavelength},
    Float, EPSILON_SHADOW_ACNE,
};
use palette::{
    chromatic_adaptation::AdaptInto, convert::IntoColorUnclamped, white_point::E, Clamp, LinSrgb,
    Xyz,
};
use rand::rngs::SmallRng;

/// The main coloring function. Sends a [`Ray`] to the [`Scene`], sees if it hits anything, and eventually returns a color. Taking into account the [Material](crate::materials::Material) that is hit, the method recurses with various adjustments, with a new [`Ray`] started from the location that was hit.
#[must_use]
pub fn colorize(
    ray: &Ray,
    scene: &Scene,
    depth: u32,
    max_depth: u32,
    rng: &mut SmallRng,
) -> Xyz<E> {
    let bg: Xyz = scene.background_color.into_color_unclamped();
    let bg: Xyz<E> = bg.adapt_into();
    // Have we reached the maximum recursion i.e. ray bounce depth?
    if depth > max_depth {
        // Ray bounce limit reached, early return background_color
        return bg;
    }

    // Send the ray to the scene, and see if it hits anything.
    // distance_min is set to an epsilon to avoid "shadow acne" that can happen when set to zero
    let Some(hit_record) = scene.objects.hit(ray, EPSILON_SHADOW_ACNE, Float::MAX, rng) else {
        // If the ray hits nothing, early return the background color.
        return bg;
    };

    // Get the emitted color from the surface that we just hit
    // TODO: spectral light sources!
    let emitted = hit_record.material.emit(
        ray,
        &hit_record,
        hit_record.u,
        hit_record.v,
        hit_record.position,
    );
    let emitted = adjust_emitted(emitted, ray.wavelength);

    // Do we scatter?
    let Some(scatter_record) = hit_record.material.scatter(ray, &hit_record, rng) else {
        // No scatter, early return the emitted color only
        return emitted;
    };
    // We have scattered, and received an attenuation from the material.
    let attenuation = adjust_attenuation(scatter_record.attenuation, ray.wavelength);

    // Check the material type and recurse accordingly:
    match scatter_record.material_type {
        MaterialType::Specular => {
            // If we hit a specular material, generate a specular ray, and multiply it with the attenuation
            let specular = colorize(
                // a scatter_record from a specular material should always have this ray
                &scatter_record.specular_ray.unwrap(),
                scene,
                depth + 1,
                max_depth,
                rng,
            );
            specular * attenuation
        }
        MaterialType::Diffuse => {
            // Use a probability density function to figure out where to scatter a new ray
            // TODO: this weighed priority sampling should be adjusted or removed - doesn't feel ideal.
            let light_ptr = PDF::HitablePDF(HitablePDF::new(
                &scene.priority_objects,
                hit_record.position,
            ));
            let mixture_pdf = MixturePDF::new(light_ptr, scatter_record.pdf_ptr);
            let scatter_ray = Ray {
                origin: hit_record.position,
                direction: mixture_pdf.generate(rng),
                time: ray.time,
                wavelength: ray.wavelength,
            };
            let pdf_val = mixture_pdf.value(scatter_ray.direction, ray.wavelength, ray.time, rng);
            if pdf_val <= 0.0 {
                // scattering impossible, prevent division by zero below
                // for more ctx, see https://github.com/RayTracing/raytracing.github.io/issues/979#issuecomment-1034517236
                return emitted;
            }

            // Calculate the PDF weighting for the scatter // TODO: understand the literature for this, and explain
            let Some(scattering_pdf) =
                hit_record
                    .material
                    .scattering_pdf(&hit_record, &scatter_ray, rng)
            else {
                // No scatter, only emit
                return emitted;
            };

            // Recurse for the scattering ray
            let recurse = colorize(&scatter_ray, scene, depth + 1, max_depth, rng);
            // Tint and weight it according to the PDF
            let scattered = attenuation * scattering_pdf * recurse / pdf_val;
            // Ensure positive color
            // let scattered = scattered.non_negative();
            // Blend it all together
            emitted + scattered
        }
    }
}

fn adjust_emitted(emitted: LinSrgb, wavelength: Wavelength) -> Xyz<E> {
    let tint: Xyz<E> = wavelength_into_xyz(wavelength);
    let emitted: Xyz = emitted.into_color_unclamped();
    let emitted: Xyz<E> = emitted.adapt_into();
    tint * emitted
}

fn adjust_attenuation(attenuation: LinSrgb, wavelength: Wavelength) -> Xyz<E> {
    let attenuation: Xyz = attenuation.into_color_unclamped();
    let attenuation: Xyz<E> = attenuation.adapt_into();
    let attenuation_factor = spectrum_xyz_to_p(wavelength, attenuation);
    let attenuation = attenuation * attenuation_factor;
    attenuation.clamp()
}
