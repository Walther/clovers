use crate::{
    camera::Camera, color::Color, colorize::colorize, hitable::BVHNode, ray::Ray, scenes, Float,
    GAMMA, HEIGHT, SAMPLES, WIDTH,
};
use image::{ImageBuffer, ImageResult, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;
use rayon::prelude::*;

/// The main drawing function, returns an `ImageResult`.
pub fn draw() -> ImageResult<ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>> {
    let mut img: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

    let rng = rand::thread_rng();
    let scene = scenes::cornell_with_sphere::load(rng);
    let world: BVHNode = scene.world;
    let camera: Camera = scene.camera;
    let background_color: Color = scene.background;

    // Progress bar
    let pixels = (WIDTH * HEIGHT) as u64;
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
            for _sample in 0..SAMPLES {
                u = (x as Float + rng.gen::<Float>()) / WIDTH as Float;
                v = (y as Float + rng.gen::<Float>()) / HEIGHT as Float;
                ray = camera.get_ray(u, v, rng);
                color += colorize(&ray, background_color, &world, 0, rng);
            }
            color /= SAMPLES as Float;

            color = color.gamma_correction(GAMMA);
            *pixel = color.to_rgb_u8();

            bar.inc(1);
        });

    // Graphics assume origin at bottom left corner of the screen
    // Our buffer writes pixels from top left corner. Simple fix, just flip it!
    image::imageops::flip_vertical_in_place(&mut img);
    Ok(img)
}
