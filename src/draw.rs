use crate::{color::Color, colorize::colorize, ray::Ray, scenes, Float};
use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;
use rayon::prelude::*;
use scenes::Scene;

/// The main drawing function, returns a Vec<Color> as a pixelbuffer.
pub fn draw(
    width: u32,
    height: u32,
    samples: u32,
    max_depth: u32,
    gamma: Float,
    no_progress: bool,
    scene: Scene,
) -> Vec<Color> {
    // Progress bar
    let pixels = (width * height) as u64;

    let bar = ProgressBar::new(pixels);
    if !no_progress {
        bar.set_draw_delta(pixels / 1000);
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
            let x = index % width as usize;
            let y = index / width as usize;

            let mut rng = rand::thread_rng();
            let mut color: Color = Color::new(0.0, 0.0, 0.0);
            let mut u: Float;
            let mut v: Float;
            let mut ray: Ray;

            // Multisampling for antialiasing
            for _sample in 0..samples {
                u = (x as Float + rng.gen::<Float>()) / width as Float;
                v = (y as Float + rng.gen::<Float>()) / height as Float;
                ray = scene.camera.get_ray(u, v, rng);
                let new_color = colorize(&ray, &scene, 0, max_depth, rng);
                // skip NaN and Infinity
                if new_color.r.is_finite() && new_color.g.is_finite() && new_color.b.is_finite() {
                    color += new_color;
                }
            }
            color /= samples as Float;

            color = color.gamma_correction(gamma);
            *pixel = color;

            if !no_progress {
                bar.inc(1);
            }
        });

    pixelbuffer
}
