use crate::{color::Color, colorize::colorize, normals::normal_map, ray::Ray, scenes, Float};
use clovers::RenderOpts;
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use scenes::Scene;
use std::time::Duration;

/// The main drawing function, returns a `Vec<Color>` as a pixelbuffer.
pub fn draw(opts: RenderOpts, scene: &Scene) -> Vec<Color> {
    // Progress bar
    let pixels = (opts.width * opts.height) as u64;
    let bar = ProgressBar::new(pixels);

    if opts.quiet {
        bar.set_draw_target(ProgressDrawTarget::hidden())
    } else {
        bar.set_style(ProgressStyle::default_bar().template(
            "Elapsed: {elapsed_precise}\nPixels:  {bar} {pos}/{len}\nETA:     {eta_precise}",
        ).unwrap());
        bar.enable_steady_tick(Duration::from_millis(100));
    }

    let black = Color::new(0.0, 0.0, 0.0);
    let mut pixelbuffer = vec![black; pixels as usize];

    pixelbuffer
        .par_iter_mut()
        .enumerate()
        .for_each(|(index, pixel)| {
            // Enumerate gives us an usize, opts.width and opts.height are u32
            // Most internal functions expect a Float, perform conversions
            let x = (index % (opts.width as usize)) as Float;
            let y = (index / (opts.width as usize)) as Float;
            let width = opts.width as Float;
            let height = opts.height as Float;

            // Initialize a thread-local random number generator
            let mut rng = SmallRng::from_entropy();

            // Initialize a mutable base color for the pixel
            let mut color: Color = Color::new(0.0, 0.0, 0.0);

            if opts.normalmap {
                // If we are rendering just a normalmap, make it quick and early return
                let u = x / width;
                let v = y / height;
                let ray: Ray = scene.camera.get_ray(u, v, &mut rng);
                color = normal_map(&ray, scene, &mut rng);
                *pixel = color;
                return;
            }
            // Otherwise, do a regular render

            // Multisampling for antialiasing
            for _sample in 0..opts.samples {
                if let Some(s) = sample(scene, x, y, width, height, &mut rng, opts.max_depth) {
                    color += s
                }
            }
            color /= opts.samples as Float;

            // After multisampling, perform gamma correction and store final color into the pixel
            color = color.gamma_correction(opts.gamma);
            *pixel = color;

            bar.inc(1);
        });

    pixelbuffer
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
    if new_color.is_finite() {
        // helper when experimentally switching between XYZ and RGB in colorize.rs
        #[allow(clippy::useless_conversion)]
        return Some(new_color.into());
    }
    None
}
