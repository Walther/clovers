//! An opinionated method for drawing a scene using the CPU for rendering.

use clovers::wavelength::{random_wavelength, wavelength_into_xyz};
use clovers::Vec2;
use clovers::{ray::Ray, scenes::Scene, Float};
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use palette::chromatic_adaptation::AdaptInto;
use palette::white_point::E;
use palette::{LinSrgb, Xyz};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;

use crate::colorize::colorize;
use crate::debug_visualizations::{bvh_testcount, primitive_testcount};
use crate::normals::normal_map;
use crate::render::{RenderMode, RenderOptions};
use crate::sampler::blue::BlueSampler;
use crate::sampler::random::RandomSampler;
use crate::sampler::{Randomness, Sampler, SamplerTrait};
use crate::GlobalOptions;

/// The main drawing function, returns a `Vec<Srgb>` as a pixelbuffer.
pub fn draw(
    global_options: &GlobalOptions,
    render_options: &RenderOptions,
    scene: &Scene,
    _sampler: Sampler,
) -> Vec<Xyz<E>> {
    let GlobalOptions { debug: _, quiet } = *global_options;
    let RenderOptions {
        input: _,
        output: _,
        width,
        height,
        samples,
        max_depth: _,
        mode,
        sampler,
        bvh: _,
        formats: _,
    } = *render_options;
    let bar = progress_bar(height, quiet);

    let height = height as usize;
    let width = width as usize;

    // TODO: fix the coordinate system; this flips up<->down
    let rows: Vec<usize> = (0..height).rev().collect();

    let pixelbuffer: Vec<Xyz<E>> = rows
        .into_par_iter()
        .map(|row_index| {
            let mut sampler_rng = SmallRng::from_entropy();
            let mut sampler: Box<dyn SamplerTrait> = match sampler {
                Sampler::Blue => Box::new(BlueSampler::new(samples)),
                Sampler::Random => Box::new(RandomSampler::new(&mut sampler_rng)),
            };

            let mut rng = SmallRng::from_entropy();
            let mut row = Vec::with_capacity(width);

            for index in 0..width {
                let index = index + row_index * width;
                let pixel = match mode {
                    RenderMode::PathTracing => {
                        render_pixel(scene, render_options, index, &mut rng, &mut *sampler)
                    }
                    RenderMode::NormalMap => {
                        render_pixel_normalmap(scene, render_options, index, &mut rng)
                    }
                    RenderMode::BvhTestCount => render_pixel_bvhtestcount(
                        scene,
                        render_options,
                        index,
                        &mut rng,
                        &mut *sampler,
                    ),
                    RenderMode::PrimitiveTestCount => render_pixel_primitivetestcount(
                        scene,
                        render_options,
                        index,
                        &mut rng,
                        &mut *sampler,
                    ),
                };
                row.push(pixel);
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
    opts: &RenderOptions,
    index: usize,
    rng: &mut SmallRng,
    sampler: &mut dyn SamplerTrait,
) -> Xyz<E> {
    let (x, y, width, height) = index_to_params(opts, index);
    let pixel_location = Vec2::new(x, y);
    let canvas_size = Vec2::new(width, height);
    let max_depth = opts.max_depth;
    let mut pixel_color: Xyz<E> = Xyz::new(0.0, 0.0, 0.0);
    for sample in 0..opts.samples {
        let Randomness {
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
        let spectral_power: Float = colorize(&ray, scene, 0, max_depth, rng, sampler);
        // TODO: find and debug more sources of NaNs!
        if spectral_power.is_normal() && spectral_power.is_sign_positive() {
            let sample_color = wavelength_into_xyz(ray.wavelength);
            pixel_color += sample_color * spectral_power;
        }
    }
    pixel_color / opts.samples as Float
}

// Render a single pixel in normalmap mode
fn render_pixel_normalmap(
    scene: &Scene,
    opts: &RenderOptions,
    index: usize,
    rng: &mut SmallRng,
) -> Xyz<E> {
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
    color.adapt_into()
}

// Render a single pixel in bvh test count visualization mode
fn render_pixel_bvhtestcount(
    scene: &Scene,
    render_options: &RenderOptions,
    index: usize,
    rng: &mut SmallRng,
    _sampler: &mut dyn SamplerTrait,
) -> Xyz<E> {
    let (x, y, width, height) = index_to_params(render_options, index);
    let pixel_location = Vec2::new(x / width, y / height);
    let lens_offset = Vec2::new(0.0, 0.0);
    let wavelength = random_wavelength(rng);
    let time = rng.gen();
    let ray: Ray = scene
        .camera
        .get_ray(pixel_location, lens_offset, time, wavelength);

    bvh_testcount(&ray, scene, rng)
}

// Render a single pixel in primitive test count visualization mode
fn render_pixel_primitivetestcount(
    scene: &Scene,
    render_options: &RenderOptions,
    index: usize,
    rng: &mut SmallRng,
    _sampler: &mut dyn SamplerTrait,
) -> Xyz<E> {
    let (x, y, width, height) = index_to_params(render_options, index);
    let pixel_location = Vec2::new(x / width, y / height);
    let lens_offset = Vec2::new(0.0, 0.0);
    let wavelength = random_wavelength(rng);
    let time = rng.gen();
    let ray: Ray = scene
        .camera
        .get_ray(pixel_location, lens_offset, time, wavelength);

    primitive_testcount(&ray, scene, rng)
}

fn index_to_params(opts: &RenderOptions, index: usize) -> (Float, Float, Float, Float) {
    let x = (index % (opts.width as usize)) as Float;
    let y = (index / (opts.width as usize)) as Float;
    let width = opts.width as Float;
    let height = opts.height as Float;
    (x, y, width, height)
}

fn progress_bar(height: u32, quiet: bool) -> ProgressBar {
    let bar = ProgressBar::new(height as u64);
    if quiet {
        bar.set_draw_target(ProgressDrawTarget::hidden())
    } else {
        bar.set_style(ProgressStyle::default_bar().template(
            "Elapsed:   {elapsed_precise}\nRows:      {bar} {pos}/{len}\nRemaining: {eta_precise}",
        ).unwrap());
    }
    bar
}
