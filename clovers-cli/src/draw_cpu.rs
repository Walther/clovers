use crate::{color::Color, colorize::colorize, normals::normal_map, ray::Ray, scenes, Float};
use clovers::RenderOpts;
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use scenes::Scene;

/// The main drawing function, returns a Vec<Color> as a pixelbuffer.
pub fn draw(opts: RenderOpts, scene: Scene) -> Vec<Color> {
    // Setup
    let pixels = (opts.width * opts.height) as u64;
    let width = opts.width as Float;
    let height = opts.height as Float;
    let thread_count = std::thread::available_parallelism().unwrap().get() as u64;
    let chunk_size = (pixels / thread_count) as usize;

    // Progress bar
    let bar = ProgressBar::new(thread_count);

    if opts.quiet {
        bar.set_draw_target(ProgressDrawTarget::hidden())
    } else {
        bar.set_style(ProgressStyle::default_bar().template(
            "Elapsed: {elapsed_precise}\nChunks:  {bar} {pos}/{len}\nETA:     {eta_precise}",
        ).unwrap());
    }

    vec![(); pixels as usize]
        .par_chunks(chunk_size)
        .enumerate()
        .map(|(chunk_index, chunk)| {
            let mut rng = SmallRng::from_entropy();
            let mut chunk_buffer: Vec<Color> = Vec::with_capacity(chunk_size);

            for index in 0..chunk.len() {
                let index = (index + chunk_index * chunk_size) as u32;
                let x = (index % opts.width) as Float;
                let y = (index / opts.width) as Float;

                // Initialize a mutable base color for the pixel
                let mut color: Color = Color::new(0.0, 0.0, 0.0);

                if opts.normalmap {
                    // If we are rendering just a normalmap, make it quick and early return
                    let u = x / width;
                    let v = y / height;
                    let ray: Ray = scene.camera.get_ray(u, v, &mut rng);
                    color = normal_map(&ray, &scene, &mut rng);
                    chunk_buffer.push(color);
                    continue;
                }
                // Otherwise, do a regular render

                // Multisampling for antialiasing
                for _sample in 0..opts.samples {
                    if let Some(s) = sample(&scene, x, y, width, height, &mut rng, opts.max_depth) {
                        color += s
                    }
                }
                color /= opts.samples as Float;

                // After multisampling, perform gamma correction and store final color into the pixel
                color = color.gamma_correction(opts.gamma);
                chunk_buffer.push(color)
            }
            bar.inc(1);
            chunk_buffer
        })
        .flatten()
        .collect()
}

/// Get a single sample for a single pixel in the scene. Has slight jitter for antialiasing when multisampling.
fn sample(
    scene: &Scene,
    x: Float,
    y: Float,
    width: Float,
    height: Float,
    rng: &mut SmallRng,
    max_depth: u32,
) -> Option<Color> {
    let u = (x + rng.gen::<Float>()) / width;
    let v = (y + rng.gen::<Float>()) / height;
    let ray: Ray = scene.camera.get_ray(u, v, rng);
    let new_color = colorize(&ray, scene, 0, max_depth, rng);
    // skip NaN and Infinity
    if new_color.r.is_finite() && new_color.g.is_finite() && new_color.b.is_finite() {
        return Some(new_color);
    }
    None
}
