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
use hitable::{BVHNode, HitRecord, Hitable, HitableList};
use scenes::{metal_spheres, random_scene};

const SHADOW_SMOOTHING: Float = 0.001;
const GAMMA: Float = 2.0;
const WIDTH: u32 = 1000;
const HEIGHT: u32 = 600;
const ANTIALIAS_SAMPLES: u32 = 100;
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
fn colorize(ray: &Ray, world: &dyn Hitable, depth: u32, rng: ThreadRng) -> Color {
    let color: Color;

    match world.hit(&ray, SHADOW_SMOOTHING, Float::MAX) {
        Some(hit_record) => {
            // Hit an object, scatter and colorize the new ray
            if depth < MAX_DEPTH {
                if let Some((scattered, attenuation)) =
                    hit_record.material.scatter(&ray, &hit_record, rng)
                {
                    color = attenuation.component_mul(&colorize(&scattered, world, depth + 1, rng));
                    return color;
                }
            }
            // Ray absorbed, no color
            Color::new(0.0, 0.0, 0.0)
        }
        None => {
            // Background, blue-white gradient. Magic from tutorial.
            let unit_direction: Vec3 = ray.direction.normalize();
            let t: Float = 0.5 * (unit_direction.y + 1.0);
            color = ((1.0 - t) as Float) * Color::new(1.0, 1.0, 1.0)
                + (t as Float) * Color::new(0.5, 0.7, 1.0);
            color
        }
    }
}

/// The main drawing function, returns an `ImageResult`.
fn draw() -> ImageResult<()> {
    let mut img: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

    let rng = rand::thread_rng();
    // let world: HitableList = random_scene::scene(rng);
    let world: BVHNode = random_scene::bvh_scene(rng);
    let camera: Camera = random_scene::camera();
    // let world: HitableList = metal_spheres::scene(rng);
    // let camera: Camera = metal_spheres::camera();

    img.enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(x, y, pixel)| {
            let mut rng = rand::thread_rng();
            let mut color: Color = Color::new(0.0, 0.0, 0.0);
            let mut u: Float;
            let mut v: Float;
            let mut ray: Ray;

            // Multisampling for antialiasing
            for _sample in 0..ANTIALIAS_SAMPLES {
                u = (x as Float + rng.gen::<Float>()) / WIDTH as Float;
                v = (y as Float + rng.gen::<Float>()) / HEIGHT as Float;
                ray = camera.get_ray(u, v, rng);
                color += colorize(&ray, &world, 0, rng);
            }
            color /= ANTIALIAS_SAMPLES as Float;

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
