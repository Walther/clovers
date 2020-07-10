use crate::{
    color::Color,
    colorize::colorize,
    hitable::HitableList,
    materials::{Dielectric, DiffuseLight},
    objects::{Sphere, XZRect},
    ray::Ray,
    scenes,
    textures::SolidColor,
    Float, Vec3,
};
use clovers::hitable::Hitable;
use image::{ImageBuffer, ImageResult, Rgb, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;
use rayon::prelude::*;
use scenes::Scene;
use std::sync::Arc;

/// The main drawing function, returns an `ImageResult`.
pub fn draw(
    width: u32,
    height: u32,
    samples: u32,
    max_depth: u32,
    gamma: Float,
    scene: Scene,
    lights: Arc<Hitable>,
) -> ImageResult<ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>> {
    let mut img: RgbImage = ImageBuffer::new(width as u32, height as u32);

    let background_color: Color = scene.background;

    // Progress bar
    let pixels = (width * height) as u64;
    let bar = ProgressBar::new(pixels);
    bar.set_draw_delta(pixels / 1000);
    bar.set_style(ProgressStyle::default_bar().template(
        "Elapsed: {elapsed_precise}\nPixels:  {bar} {pos}/{len}\nETA:     {eta_precise}",
    ));

    img.enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(x, y, pixel)| {
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
                color += colorize(
                    &ray,
                    background_color,
                    &scene.world,
                    Arc::clone(&lights), // TODO: fixme, this is silly
                    0,
                    max_depth,
                    rng,
                );
            }
            color /= samples as Float;

            color = color.gamma_correction(gamma);
            *pixel = Rgb(color.to_rgb_u8());

            bar.inc(1);
        });

    // Graphics assume origin at bottom left corner of the screen
    // Our buffer writes pixels from top left corner. Simple fix, just flip it!
    image::imageops::flip_vertical_in_place(&mut img);
    Ok(img)
}
