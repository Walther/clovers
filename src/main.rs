#![deny(clippy::all)]

use rayon::prelude::*;

use rand::prelude::*;

use image::{ImageBuffer, ImageResult, RgbImage};

use nalgebra::Vector3;

use std::time::Instant;

use chrono::Utc;

mod hitable;
mod moving_sphere;
mod ray;
mod sphere;
use ray::Ray;
mod camera;
use camera::Camera;
mod color;
mod material;
use material::Material;
mod scenes;
use color::Color;
use hitable::{face_normal, BVHNode, HitRecord, Hitable, HitableList};
use scenes::{metal_spheres, random_scene, simple_light, two_perlin_spheres, two_spheres};
mod perlin;
mod texture;
use perlin::Perlin;
mod xy_rect;

const SMOOTHING_EPSILON: Float = 0.00001;
const GAMMA: Float = 2.0;
const WIDTH: u32 = 1000;
const HEIGHT: u32 = 600;
const SAMPLES: u32 = 500;
const MAX_DEPTH: u32 = 50;

fn main() -> ImageResult<()> {
    println!("clovers - ray tracing in rust <3");
    let start = Instant::now();
    draw()?;
    let duration = Instant::now() - start;
    println!("rendered in {} ms", duration.as_millis());
    Ok(())
}

// Handy aliases for internal use
type Float = f32;
pub const PI: Float = std::f32::consts::PI as Float;
type Vec3 = Vector3<Float>;

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
    match world.hit(&ray, SMOOTHING_EPSILON, Float::MAX) {
        Some(hit_record) => {
            // Hit an object
            let emitted: Color =
                hit_record
                    .material
                    .emitted(hit_record.u, hit_record.v, hit_record.position);
            // scatter and colorize the new ray
            if let Some((scattered, attenuation)) =
                hit_record.material.scatter(&ray, &hit_record, rng)
            {
                color = attenuation.component_mul(&colorize(
                    &scattered,
                    background_color,
                    world,
                    depth + 1,
                    rng,
                ));
                return color + emitted;
            } else {
                // no scatter, emit only
                return emitted;
            }
        }
        None => {
            // Did not hit anything, return the background_color
            return background_color;
        }
    }
}

/// The main drawing function, returns an `ImageResult`.
fn draw() -> ImageResult<()> {
    let mut img: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

    let rng = rand::thread_rng();
    let world: HitableList = simple_light::scene(rng);
    let world: BVHNode = world.into_bvh(0.0, 1.0, rng);
    let camera: Camera = simple_light::camera();
    let background_color: Color = Color::new(0.0, 0.0, 0.0);

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
    // Timestamp & write
    let timestamp = Utc::now().timestamp();
    println!("{}", timestamp);
    img.save(format!("renders/{}.png", timestamp))
}
