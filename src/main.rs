#![deny(clippy::all)]
// A lot of loader functions etc, suppresses some warning noise
#![allow(dead_code)]

use rayon::prelude::*;

use rand::prelude::*;

use image::{ImageBuffer, ImageResult, RgbImage};

use nalgebra::Vector3;

use std::time::Instant;

use chrono::Utc;

mod hitable;
mod objects;
mod ray;
use ray::Ray;
mod camera;
use camera::Camera;
mod color;
mod materials;
mod scenes;
use color::Color;
use hitable::{BVHNode, HitRecord, Hitable};
mod perlin;
mod textures;

// Handy aliases for internal use
type Float = f64;
pub const PI: Float = std::f64::consts::PI as Float;
type Vec3 = Vector3<Float>;
const SHADOW_EPSILON: Float = 0.001;
const RECT_EPSILON: Float = 0.0001;
const CONSTANT_MEDIUM_EPSILON: Float = 0.0001;
const GAMMA: Float = 2.0;
const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;
const SAMPLES: u32 = 100;
const MAX_DEPTH: u32 = 50;
const GUESS_RAYS_PER_MS: u32 = 400_000; // totally unscientific, reasonable approximation on my home machine

fn main() -> ImageResult<()> {
    println!("clovers - ray tracing in rust <3");
    let rays: u64 = WIDTH as u64 * HEIGHT as u64 * SAMPLES as u64 * MAX_DEPTH as u64;
    println!("tracing approximately {} rays", rays);
    println!(
        "guessing render time: {} ms",
        rays / GUESS_RAYS_PER_MS as u64
    );
    let start = Instant::now();
    let img = draw()?;
    let duration = Instant::now() - start;
    println!("finished render in {} ms", duration.as_millis());
    // Timestamp & write
    let timestamp = Utc::now().timestamp();
    println!("output saved: renders/{}.png", timestamp);
    img.save(format!("renders/{}.png", timestamp))?;
    Ok(())
}

/// The main coloring function
fn colorize(
    ray: &Ray,
    background_color: Color,
    world: &dyn Hitable,
    depth: u32,
    rng: ThreadRng,
) -> Color {
    let color: Color;

    if depth > MAX_DEPTH {
        // Ray bounce limit reached, return background_color
        return background_color;
    }

    // Here, smoothing is used to avoid "shadow acne"
    match world.hit(&ray, SHADOW_EPSILON, Float::MAX, rng) {
        // Hit an object
        Some(hit_record) => {
            let emitted: Color =
                hit_record
                    .material
                    .emitted(hit_record.u, hit_record.v, hit_record.position);
            // Try to scatter and colorize the new ray
            match hit_record.material.scatter(&ray, &hit_record, rng) {
                // Got a scatter and attenuation
                Some((scattered, attenuation)) => {
                    color = emitted
                        + attenuation.component_mul(
                            // Recurse
                            &colorize(&scattered, background_color, world, depth + 1, rng),
                        );

                    // TODO: consider whether you want to clamp here. Pros: avoids overflow. Cons: seems to affect a bunch of things, including lightness, saturation etc...
                    // let color = Color::new(color.r.min(1.0), color.g.min(1.0), color.b.min(1.0));
                    // let color = Color::new(color.r.max(0.0), color.g.max(0.0), color.b.max(0.0));

                    return color;
                }
                // No scatter, emit only
                None => {
                    return emitted;
                }
            }
        }
        // Did not hit anything, return the background_color
        None => {
            // DEBUG
            // return Color::new(0.3, 0.0, 0.0);
            return background_color;
        }
    }
}

/// The main drawing function, returns an `ImageResult`.
fn draw() -> ImageResult<ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>> {
    let mut img: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

    let rng = rand::thread_rng();
    let scene = scenes::cornell_with_sphere::load(rng);
    let world: BVHNode = scene.world;
    let camera: Camera = scene.camera;
    let background_color: Color = scene.background;

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
        });

    // Graphics assume origin at bottom left corner of the screen
    // Our buffer writes pixels from top left corner. Simple fix, just flip it!
    image::imageops::flip_vertical_in_place(&mut img);
    Ok(img)
}
