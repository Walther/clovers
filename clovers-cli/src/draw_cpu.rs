use std::sync::Arc;

use crate::{color::Color, colorize::colorize, normals::normal_map, ray::Ray, scenes, Float};
use clovers::RenderOpts;
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use scenes::Scene;
use tokio::runtime::Runtime;

/// The main drawing function, returns a Vec<Color> as a pixelbuffer.
pub fn draw(opts: RenderOpts, scene: Scene) -> Vec<Color> {
    // Progress bar
    let pixels = (opts.width * opts.height) as u64;
    let bar = ProgressBar::new(pixels);

    if opts.quiet {
        bar.set_draw_target(ProgressDrawTarget::hidden())
    } else {
        bar.set_style(ProgressStyle::default_bar().template(
            "Elapsed: {elapsed_precise}\nPixels:  {bar} {pos}/{len}\nETA:     {eta_precise}",
        ).unwrap());
    }

    let black = Color::new(0.0, 0.0, 0.0);
    let mut pixelbuffer = vec![black; pixels as usize];
    let mut futurebuffer = Vec::new();
    let scene = Arc::new(scene); // TODO: is it possible to remove this Arc? with par_iter i only used a &scene, no Arc

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        // Multi-threading go brr
        for index in 0..pixels {
            let result = {
                let bar = bar.clone();
                let scene = scene.clone();
                // Expensive pixel rendering here
                tokio::spawn(async move {
                    // Enumerate gives us an usize, width and height are u32. perform conversions
                    let x = (index % (opts.width as u64)) as Float;
                    let y = (index / (opts.width as u64)) as Float;
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
                        color = normal_map(&ray, &scene.clone(), &mut rng);
                        return color;
                    }

                    // Multisampling for antialiasing
                    for _sample in 0..opts.samples {
                        if let Some(s) =
                            sample(&scene, x, y, width, height, &mut rng, opts.max_depth)
                        {
                            color += s
                        }
                    }
                    color /= opts.samples as Float;

                    // After multisampling, perform gamma correction and store final color into the pixel
                    color = color.gamma_correction(opts.gamma);
                    bar.inc(1);
                    color
                })
            };
            futurebuffer.push(result)
        }

        for (index, result) in futurebuffer.into_iter().enumerate() {
            match result.await {
                Ok(pixel) => {
                    pixelbuffer[index as usize] = pixel;
                }
                Err(_) => todo!(),
            }
        }
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
    // skip NaN and Infinity
    if new_color.r.is_finite() && new_color.g.is_finite() && new_color.b.is_finite() {
        return Some(new_color);
    }
    None
}
