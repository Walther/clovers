use clovers::{
    colorize::colorize, normals::normal_map, ray::Ray, scenes::Scene, Float, RenderOpts,
};
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use palette::chromatic_adaptation::AdaptInto;
use palette::convert::IntoColorUnclamped;
use palette::white_point::E;
use palette::{IntoColor, LinSrgb, Srgb, Xyz};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;

/// The main drawing function, returns a `Vec<Srgb>` as a pixelbuffer.
pub fn draw(opts: RenderOpts, scene: &Scene) -> Vec<Srgb<u8>> {
    let width = opts.width as usize;
    let height = opts.height as usize;
    let bar = progress_bar(&opts);

    let pixelbuffer: Vec<Srgb<u8>> = (0..height)
        .into_par_iter()
        .map(|row_index| {
            let mut rng = SmallRng::from_entropy();
            let mut row = Vec::with_capacity(width);
            for index in 0..width {
                let index = index + row_index * width;
                if opts.normalmap {
                    row.push(render_pixel_normalmap(scene, &opts, index, &mut rng));
                } else {
                    row.push(render_pixel(scene, &opts, index, &mut rng));
                }
            }
            bar.inc(1);
            row
        })
        .flatten()
        .collect();

    pixelbuffer
}

// Render a single pixel, including possible multisampling
fn render_pixel(scene: &Scene, opts: &RenderOpts, index: usize, rng: &mut SmallRng) -> Srgb<u8> {
    let (x, y, width, height) = index_to_params(opts, index);
    let mut color: LinSrgb = LinSrgb::new(0.0, 0.0, 0.0);
    for _sample in 0..opts.samples {
        if let Some(s) = sample(scene, x, y, width, height, rng, opts.max_depth) {
            color += s
        }
    }
    color /= opts.samples as Float;
    let color: Srgb = color.into_color();
    let color: Srgb<u8> = color.into_format();
    color
}

// Render a single pixel in normalmap mode
fn render_pixel_normalmap(
    scene: &Scene,
    opts: &RenderOpts,
    index: usize,
    rng: &mut SmallRng,
) -> Srgb<u8> {
    let (x, y, width, height) = index_to_params(opts, index);
    let color: LinSrgb = sample_normalmap(scene, x, y, width, height, rng);
    let color: Srgb = color.into_color();
    let color: Srgb<u8> = color.into_format();
    color
}

// Get a single sample for a single pixel in the scene, normalmap mode.
fn sample_normalmap(
    scene: &Scene,
    x: Float,
    y: Float,
    width: Float,
    height: Float,
    rng: &mut SmallRng,
) -> LinSrgb {
    let u = x / width;
    let v = y / height;
    let ray: Ray = scene.camera.get_ray(u, v, rng);
    let color = normal_map(&ray, scene, rng);
    color.into_color()
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
) -> Option<LinSrgb> {
    let u = (x + rng.gen::<Float>()) / width;
    let v = (y + rng.gen::<Float>()) / height;
    let ray: Ray = scene.camera.get_ray(u, v, rng);
    let color: Xyz<E> = colorize(&ray, scene, 0, max_depth, rng);
    let color: Xyz = color.adapt_into();
    let color: LinSrgb = color.into_color_unclamped();
    if color.red.is_finite() && color.green.is_finite() && color.blue.is_finite() {
        return Some(color);
    }
    None
}

fn index_to_params(opts: &RenderOpts, index: usize) -> (Float, Float, Float, Float) {
    let x = (index % (opts.width as usize)) as Float;
    let y = (index / (opts.width as usize)) as Float;
    let width = opts.width as Float;
    let height = opts.height as Float;
    (x, y, width, height)
}

fn progress_bar(opts: &RenderOpts) -> ProgressBar {
    let bar = ProgressBar::new(opts.height as u64);
    if opts.quiet {
        bar.set_draw_target(ProgressDrawTarget::hidden())
    } else {
        bar.set_style(ProgressStyle::default_bar().template(
            "Elapsed:   {elapsed_precise}\nRows:      {bar} {pos}/{len}\nRemaining: {eta_precise}",
        ).unwrap());
    }
    bar
}
