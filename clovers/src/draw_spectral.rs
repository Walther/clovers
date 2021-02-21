use clovers::{
    color::{Color, Photon, Wavelength},
    pdf::{HitablePDF, MixturePDF},
    ray::Ray,
    scenes::Scene,
    Float, EPSILON_SHADOW_ACNE,
};
use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;
use rayon::prelude::*;

/// Get a single sample for a single pixel in the scene. Has slight jitter for antialiasing when multisampling.
fn sample(
    scene: &Scene,
    x: usize,
    y: usize,
    width: u32,
    height: u32,
    mut rng: ThreadRng,
    max_depth: u32,
) -> Option<Color> {
    let u = (x as Float + rng.gen::<Float>()) / width as Float;
    let v = (y as Float + rng.gen::<Float>()) / height as Float;
    let ray: Ray = scene.camera.get_ray(u, v, rng);
    let wavelength = rng.gen_range(300.0, 1000.0); // TODO: remove hardcoded guess
    let new_color = colorize_spectral(&ray, &wavelength, &scene, 0, max_depth, rng);
    // skip NaN and Infinity
    if new_color.r.is_finite() && new_color.g.is_finite() && new_color.b.is_finite() {
        return Some(new_color);
    }
    None
}

pub fn draw_spectral(
    width: u32,
    height: u32,
    samples: u32,
    max_depth: u32,
    gamma: Float,
    scene: Scene,
) -> Vec<Color> {
    // Progress bar
    let pixels = (width * height) as u64;
    let bar = ProgressBar::new(pixels);
    bar.set_draw_delta(pixels / 1000);
    bar.set_style(ProgressStyle::default_bar().template(
        "Elapsed: {elapsed_precise}\nPixels:  {bar} {pos}/{len}\nETA:     {eta_precise}",
    ));

    let black = Color::new(0.0, 0.0, 0.0);
    let mut pixelbuffer = vec![black; pixels as usize];

    pixelbuffer
        .par_iter_mut()
        .enumerate()
        .for_each(|(index, pixel)| {
            let x = index % width as usize;
            let y = index / width as usize;
            let rng = rand::thread_rng();
            let mut color: Color = Color::new(0.0, 0.0, 0.0);

            // Multisampling for antialiasing
            for _sample in 0..samples {
                match sample(&scene, x, y, width, height, rng, max_depth) {
                    Some(s) => color += s,
                    None => {}
                }
            }
            color /= samples as Float;

            color = color.gamma_correction(gamma);
            *pixel = color;

            bar.inc(1);
        });

    pixelbuffer
}

pub fn colorize_spectral(
    ray: &Ray,
    wavelength: &Wavelength,
    scene: &Scene,
    depth: u32,
    max_depth: u32,
    rng: ThreadRng,
    // TODO: do we want to return a Color or a Photon?
) -> Color {
    if depth > max_depth {
        // Ray bounce limit reached, return background color
        return scene.background_color;
    }

    // Here, smoothing is used to avoid "shadow acne"
    match scene
        .objects
        .hit(&ray, EPSILON_SHADOW_ACNE, Float::MAX, rng)
    {
        // If the ray hits nothing, return background color
        None => scene.background_color,

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
            match hit_record
                .material
                .scatter_spectral(&ray, &wavelength, &hit_record, rng)
            {
                // No scatter, emit only
                None => emitted, // TODO: photon turned into Color here
                // Got a scatter
                Some(scatter_record) => {
                    match scatter_record.material_type {
                        // If we hit a specular, return a specular ray
                        crate::materials::MaterialType::Specular => {
                            scatter_record.attenuation
                                * colorize_spectral(
                                    &scatter_record.specular_ray.unwrap(), // should always have a ray at this point
                                    wavelength,
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
                            let pdf_val = mixture_pdf.value(scattered.direction, ray.time, rng);

                            // recurse
                            let recurse = colorize_spectral(
                                &scattered,
                                wavelength,
                                scene,
                                depth + 1,
                                max_depth,
                                rng,
                            );

                            // Blend it all together
                            emitted
                                + scatter_record.attenuation
                                    * hit_record.material.scattering_pdf(
                                        ray,
                                        &hit_record,
                                        &scattered,
                                        rng,
                                    )
                                    * recurse
                                    / pdf_val
                        }
                    }
                }
            }
        }
    }
}
