//! An opinionated colorize method. Given a [Ray] and a [Scene], evaluates the ray's path and returns a color.

use clovers::{
    hitable::HitableTrait,
    materials::MaterialType,
    pdf::{HitablePDF, MixturePDF, PDFTrait, PDF},
    ray::Ray,
    scenes::Scene,
    spectrum::spectrum_xyz_to_p,
    Float, EPSILON_SHADOW_ACNE,
};
use nalgebra::Unit;
use palette::{chromatic_adaptation::AdaptInto, convert::IntoColorUnclamped, white_point::E, Xyz};
use rand::rngs::SmallRng;

use crate::sampler::SamplerTrait;

/// The main path tracing function. Sends a [`Ray`] to the [`Scene`], sees if it hits anything, and eventually returns a spectral intensity. Taking into account the [Material](clovers::materials::Material) that is hit, the method recurses with various adjustments, with a new [`Ray`] started from the location that was hit.
#[must_use]
#[allow(clippy::only_used_in_recursion)] // TODO: use sampler in more places!
pub fn trace(
    ray: &Ray,
    scene: &Scene,
    depth: u32,
    max_depth: u32,
    rng: &mut SmallRng,
    sampler: &dyn SamplerTrait,
) -> Float {
    let wavelength = ray.wavelength;

    let bg: Xyz = scene.background_color.into_color_unclamped();
    let bg: Xyz<E> = bg.adapt_into();
    let bg: Float = spectrum_xyz_to_p(wavelength, bg);

    // Have we reached the maximum recursion i.e. ray bounce depth?
    if depth > max_depth {
        // Ray bounce limit reached, early return background_color
        return bg;
    }

    // Send the ray to the scene, and see if it hits anything.
    // distance_min is set to an epsilon to avoid "shadow acne" that can happen when set to zero
    let Some(hit_record) = scene
        .bvh_root
        .hit(ray, EPSILON_SHADOW_ACNE, Float::MAX, rng)
    else {
        // If the ray hits nothing, early return the background color.
        return bg;
    };

    // Get the emitted color from the surface that we just hit
    let emitted: Xyz<E> = hit_record.material.emit(ray, &hit_record);
    let emitted: Float = spectrum_xyz_to_p(wavelength, emitted);

    // Do we scatter?
    let Some(scatter_record) = hit_record.material.scatter(ray, &hit_record, rng) else {
        // No scatter, early return the emitted color only
        return emitted;
    };
    // We have scattered, and received an attenuation from the material
    let attenuation = spectrum_xyz_to_p(wavelength, scatter_record.attenuation);

    // Check the material type and recurse accordingly:
    match scatter_record.material_type {
        MaterialType::Specular => {
            // If we hit a specular material, generate a specular ray, and multiply it with the attenuation
            let specular = trace(
                // a scatter_record from a specular material should always have this ray
                &scatter_record.specular_ray.unwrap(),
                scene,
                depth + 1,
                max_depth,
                rng,
                sampler,
            );
            specular * attenuation
        }
        MaterialType::Diffuse => {
            // Multiple Importance Sampling:

            // Create a new PDF object from the priority hitables of the scene, given the current hit_record position
            let light_ptr =
                PDF::HitablePDF(HitablePDF::new(&scene.mis_bvh_root, hit_record.position));

            // Create a mixture PDF from the above + the PDF from the scatter_record
            let mixture_pdf = MixturePDF::new(light_ptr, scatter_record.pdf_ptr);

            // Generate a direction for the scattering ray to go towards, weighed by the mixture PDF
            let direction = Unit::new_normalize(mixture_pdf.generate(rng));

            // Create the ray
            let scatter_ray = Ray {
                origin: hit_record.position,
                direction,
                time: ray.time,
                wavelength: ray.wavelength,
            };

            // Get the distribution value for the PDF
            // TODO: improve correctness & optimization!
            let pdf_val = mixture_pdf.value(scatter_ray.direction, ray.wavelength, ray.time, rng);
            if pdf_val <= 0.0 {
                // scattering impossible, prevent division by zero below
                // for more ctx, see https://github.com/RayTracing/raytracing.github.io/issues/979#issuecomment-1034517236
                return emitted;
            }

            // Calculate the PDF weighting for the scatter
            // TODO: improve correctness & optimization!
            let Some(scattering_pdf) = hit_record
                .material
                .scattering_pdf(&hit_record, &scatter_ray)
            else {
                // No scatter, only emit
                return emitted;
            };

            // Recurse for the scattering ray
            let recurse = trace(&scatter_ray, scene, depth + 1, max_depth, rng, sampler);
            // Tint and weight it according to the PDF
            let scattered = attenuation * scattering_pdf * recurse / pdf_val;
            // Blend it all together
            emitted + scattered
        }
    }
}