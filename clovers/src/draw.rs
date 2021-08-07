use crate::{color::Color, colorize::colorize, ray::Ray, scenes, Float};
use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;
use rayon::prelude::*;
use scenes::Scene;
use tracing::*;

/// The main drawing function, returns a Vec<Color> as a pixelbuffer.
pub fn draw(
    width: u32,
    height: u32,
    samples: u32,
    max_depth: u32,
    gamma: Float,
    scene: Scene,
) -> Vec<Color> {
    trace_span!("draw").in_scope(|| {
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
                        Some(s) => {
                            trace!("ray_color");
                            color += s;
                        }
                        None => {
                            trace!("ray_none");
                        }
                    }
                }
                color /= samples as Float;

                color = color.gamma_correction(gamma);
                *pixel = color;

                bar.inc(1);
            });

        return pixelbuffer;
    })
}

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
    let new_color = colorize(&ray, &scene, 0, max_depth, rng);
    // skip NaN and Infinity
    if new_color.r.is_finite() && new_color.g.is_finite() && new_color.b.is_finite() {
        return Some(new_color);
    }
    None
}
