//! An opinionated colorize method. Given a [Ray] and a [Scene], evaluates the ray's path and returns a color.

use clovers::{
    hitable::HitableTrait,
    materials::MaterialType,
    pdf::{HitablePDF, MixturePDF, PDFTrait, PDF},
    ray::Ray,
    scenes::Scene,
    spectrum::spectral_powers,
    wavelength::{rotate_wavelength, WAVE_SAMPLE_COUNT},
    Float, EPSILON_SHADOW_ACNE,
};
use nalgebra::Unit;
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
) -> [Float; WAVE_SAMPLE_COUNT] {
    let hero = ray.wavelength;
    let wavelengths = rotate_wavelength(hero);

    let bg = spectral_powers(scene.background, wavelengths);

    // Have we reached the maximum recursion i.e. ray bounce depth?
    if depth > max_depth {
        // Ray bounce limit reached, early return zero emissivity
        return [0.0; WAVE_SAMPLE_COUNT];
    }

    // Send the ray to the scene, and see if it hits anything.
    // distance_min is set to an epsilon to avoid "shadow acne" that can happen when set to zero
    let Some(hit_record) = scene
        .bvh_root
        .hit(ray, EPSILON_SHADOW_ACNE, Float::MAX, rng)
    else {
        // If the ray hits nothing, early return the background color as emissivity
        return bg;
    };

    // Get the emitted color from the surface that we just hit
    let emitted =
        std::array::from_fn(|i| hit_record.material.emit(ray, wavelengths[i], &hit_record));

    // Do we scatter?
    let Some(scatter_record) = hit_record.material.scatter(ray, &hit_record, rng) else {
        // No scatter, early return the emitted color only
        return emitted;
    };
    // We have scattered, and received an attenuation from the material
    // Are we on a dispersive material? If so, terminate other wavelengths
    let attenuations = match hit_record.material.is_wavelength_dependent() {
        true => {
            let mut ret = [0.0; WAVE_SAMPLE_COUNT];
            ret[0] = hit_record.material.color(ray, hero, &hit_record);
            ret
        }
        false => {
            std::array::from_fn(|i| hit_record.material.color(ray, wavelengths[i], &hit_record))
        }
    };

    // Check the material type and recurse accordingly:
    match scatter_record.material_type {
        MaterialType::Specular => {
            // If we hit a specular material, recurse with a specular ray, and multiply it with the attenuation
            let scatter_ray = scatter_record.specular_ray.unwrap();
            let specular = trace(&scatter_ray, scene, depth + 1, max_depth, rng, sampler);
            std::array::from_fn(|i| specular[i] * attenuations[i])
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
            let mis_pdf_value = mixture_pdf.value(direction, hero, ray.time, rng);

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
            std::array::from_fn(|i| {
                emitted[i] + recurse[i] * attenuations[i] * scattering_pdf / mis_pdf_value
            })
        }
    }
}
