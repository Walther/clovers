//! An opinionated method for drawing a scene using the CPU for rendering.

use clovers::wavelength::random_wavelength;
use clovers::Vec2;
use clovers::{ray::Ray, scenes::Scene, Float, RenderOpts};
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use palette::chromatic_adaptation::AdaptInto;
use palette::convert::IntoColorUnclamped;
use palette::white_point::E;
use palette::{LinSrgb, Srgb, Xyz};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;

use crate::colorize::colorize;
use crate::normals::normal_map;
use crate::sampler::blue::BlueSampler;
use crate::sampler::random::RandomSampler;
use crate::sampler::{Sample, Sampler, SamplerTrait};

/// The main drawing function, returns a `Vec<Srgb>` as a pixelbuffer.
pub fn draw(opts: RenderOpts, scene: &Scene, sampler: Sampler) -> Vec<Srgb<u8>> {
    let width = opts.width as usize;
    let height = opts.height as usize;
    let bar = progress_bar(&opts);

    let pixelbuffer: Vec<Srgb<u8>> = (0..height)
        .into_par_iter()
        .map(|row_index| {
            let mut sampler_rng = SmallRng::from_entropy();
            let mut sampler: Box<dyn SamplerTrait> = match sampler {
                Sampler::Blue => Box::new(BlueSampler::new(&opts)),
                Sampler::Random => Box::new(RandomSampler::new(&mut sampler_rng)),
            };

            let mut rng = SmallRng::from_entropy();
            let mut row = Vec::with_capacity(width);

            for index in 0..width {
                let index = index + row_index * width;
                if opts.normalmap {
                    row.push(render_pixel_normalmap(scene, &opts, index, &mut rng));
                } else {
                    row.push(render_pixel(scene, &opts, index, &mut rng, &mut *sampler));
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
fn render_pixel(
    scene: &Scene,
    opts: &RenderOpts,
    index: usize,
    rng: &mut SmallRng,
    sampler: &mut dyn SamplerTrait,
) -> Srgb<u8> {
    let (x, y, width, height) = index_to_params(opts, index);
    let pixel_location = Vec2::new(x, y);
    let canvas_size = Vec2::new(width, height);
    let max_depth = opts.max_depth;
    let mut pixel_color: Xyz<E> = Xyz::new(0.0, 0.0, 0.0);
    for sample in 0..opts.samples {
        let Sample {
            pixel_offset,
            lens_offset,
            time,
            wavelength,
        } = sampler.sample(x as i32, y as i32, sample as i32);
        let pixel_uv: Vec2 = Vec2::new(
            (pixel_location.x + pixel_offset.x) / canvas_size.x,
            (pixel_location.y + pixel_offset.y) / canvas_size.y,
        );
        // note get_ray wants uv 0..1 location
        let ray: Ray = scene
            .camera
            .get_ray(pixel_uv, lens_offset, time, wavelength);
        let sample_color: Xyz<E> = colorize(&ray, scene, 0, max_depth, rng, sampler);
        if sample_color.x.is_finite() && sample_color.y.is_finite() && sample_color.z.is_finite() {
            pixel_color += sample_color;
        }
    }
    pixel_color /= opts.samples as Float;
    let color: Srgb = pixel_color.adapt_into();
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
    let color: LinSrgb = {
        let pixel_location = Vec2::new(x / width, y / height);
        let lens_offset = Vec2::new(0.0, 0.0);
        let wavelength = random_wavelength(rng);
        let time = rng.gen();
        let ray: Ray = scene
            .camera
            .get_ray(pixel_location, lens_offset, time, wavelength);
        normal_map(&ray, scene, rng)
    };
    let color: Srgb = color.into_color_unclamped();
    let color: Srgb<u8> = color.into_format();
    color
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
