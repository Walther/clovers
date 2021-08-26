use crate::{color::Color, colorize::colorize, ray::Ray, scenes, Float};
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use rand::rngs::SmallRng;
use rand::{thread_rng, Rng, SeedableRng};
use rayon::prelude::*;
use scenes::Scene;

/// The main drawing function, returns a Vec<Color> as a pixelbuffer.
pub fn draw(
    width: u32,
    height: u32,
    samples: u32,
    max_depth: u32,
    gamma: Float,
    quiet: bool,
    scene: Scene,
) -> Vec<Color> {
    // Progress bar
    let pixels = (width * height) as u64;
    let bar = ProgressBar::new(pixels);
    bar.set_draw_delta(pixels / 1000);

    if quiet {
        bar.set_draw_target(ProgressDrawTarget::hidden())
    } else {
        bar.set_style(ProgressStyle::default_bar().template(
            "Elapsed: {elapsed_precise}\nPixels:  {bar} {pos}/{len}\nETA:     {eta_precise}",
        ));
    }

    let black = Color::new(0.0, 0.0, 0.0);
    let mut pixelbuffer = vec![black; pixels as usize];

    pixelbuffer
        .par_iter_mut()
        .enumerate()
        .for_each(|(index, pixel)| {
            // Enumerate gives us an usize, width and height are u32. perform conversions
            let x = index % (width as usize);
            let y = index / (width as usize);

            // Convert most of these to Floats
            let x = x as Float;
            let y = y as Float;
            let width = width as Float;
            let height = height as Float;

            // Initialize a random number generator
            let mut thread_rng = thread_rng();

            // Initialize a mutable base color for the pixel
            let mut color: Color = Color::new(0.0, 0.0, 0.0);

            // Multisampling for antialiasing
            for _sample in 0..samples {
                let rng = SmallRng::from_rng(&mut thread_rng).unwrap();
                if let Some(s) = sample(&scene, x, y, width, height, rng, max_depth) {
                    color += s
                }
            }
            color /= samples as Float;

            // After multisampling, perform gamma correction and store final color into the pixel
            color = color.gamma_correction(gamma);
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
    mut rng: SmallRng,
    max_depth: u32,
) -> Option<Color> {
    let u = (x + rng.gen::<Float>()) / width;
    let v = (y + rng.gen::<Float>()) / height;
    let ray: Ray = scene.camera.get_ray(u, v, rng.clone());
    let new_color = colorize(&ray, scene, 0, max_depth, &mut rng);
    // skip NaN and Infinity
    if new_color.r.is_finite() && new_color.g.is_finite() && new_color.b.is_finite() {
        return Some(new_color);
    }
    None
}
