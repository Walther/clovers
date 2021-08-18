use crate::{color::Color, colorize::colorize, ray::Ray, scenes, Float};
use crossbeam_utils::thread;
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use rand::prelude::*;
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

    // crossbeam attempt
    // TODO: this is ugly, make it prettier
    let num_threads = 16;
    let mut pixelbuffer: Vec<Color> = vec![black; pixels as usize];
    let chunk_size = pixelbuffer.len() / num_threads; // note be careful if it doesn't divide evenly
    let mut chunks = pixelbuffer.chunks_mut(chunk_size).enumerate();

    thread::scope(move |s| {
        let mut handles = vec![];
        for _ in 0..num_threads {
            let (chunk_index, chunk) = chunks.next().unwrap();
            let scene_clone = scene.clone();
            let handle = s.spawn(move |_| {
                // Initialize a thread-local random number generator
                let rng = rand::thread_rng();

                for pixel in 0..chunk_size {
                    let index = pixel + chunk_index * chunk_size;
                    // Enumerate gives us an usize, width and height are u32. perform conversions
                    let x = index % (width as usize);
                    let y = index / (width as usize);

                    // Convert most of these to Floats
                    let x = x as Float;
                    let y = y as Float;
                    let width = width as Float;
                    let height = height as Float;

                    // Initialize a mutable base color for the pixel
                    let mut color: Color = Color::new(0.0, 0.0, 0.0);

                    // Multisampling for antialiasing
                    for _sample in 0..samples {
                        if let Some(s) = sample(&scene_clone, x, y, width, height, rng, max_depth) {
                            color += s
                        }
                    }
                    color /= samples as Float;

                    // After multisampling, perform gamma correction and store final color into the pixel
                    color = color.gamma_correction(gamma);
                    chunk[pixel] = color;

                    // can't increment progress bar; closures and moves again
                    // bar.inc(1);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().expect("Thread panicked!");
        }
    })
    .unwrap();

    pixelbuffer
}

/// Get a single sample for a single pixel in the scene. Has slight jitter for antialiasing when multisampling.
fn sample(
    scene: &Scene,
    x: Float,
    y: Float,
    width: Float,
    height: Float,
    mut rng: ThreadRng,
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
