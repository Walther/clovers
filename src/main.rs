#![deny(clippy::all)]

use rayon::prelude::*;

use rand::prelude::*;

use image::{ImageBuffer, ImageResult, Rgb, RgbImage};

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
mod random_scene;
use hitable::{HitRecord, Hitable, HitableList};
use random_scene::random_scene;

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
type Vec3 = Vector3<Float>;

/// The main coloring function
fn colorize(ray: &Ray, world: &dyn Hitable, depth: u32, rng: ThreadRng) -> Vec3 {
    let color: Vec3;

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
            color = Vec3::new(0.0, 0.0, 0.0);
            color
        }
        None => {
            // Background, blue-white gradient. Magic from tutorial.
            let unit_direction: Vec3 = ray.direction.normalize();
            let t = 0.5 * (unit_direction.y + 1.0);
            color = (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);
            color
        }
    }
}

fn color_to_rgb(color: Vec3) -> Rgb<u8> {
    // Integer-i-fy
    let r = (255.99 * color.x).floor() as u8;
    let g = (255.99 * color.y).floor() as u8;
    let b = (255.99 * color.z).floor() as u8;
    Rgb([r, g, b])
}

/// The main drawing function, returns an `ImageResult`.
fn draw() -> ImageResult<()> {
    let mut img: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);
    let camera_position: Vec3 = Vec3::new(13.0, 2.0, 3.0);
    let camera_target: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let camera_up: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let fov: Float = 25.0;
    let aspect_ratio: Float = WIDTH as Float / HEIGHT as Float;
    let aperture: Float = 0.0;
    let focus_distance: Float = 10.0;
    let camera = Camera::new(
        camera_position,
        camera_target,
        camera_up,
        fov,
        aspect_ratio,
        aperture,
        focus_distance,
        0.0,
        1.0,
    );

    let rng = rand::thread_rng();
    let world: HitableList = random_scene(rng);

    img.enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(x, y, pixel)| {
            let mut rng = rand::thread_rng();
            let mut color: Vec3 = Vec3::new(0.0, 0.0, 0.0);
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

            // Gamma correction
            let gamma_correction = 1.0 / GAMMA;
            color.x = color.x.powf(gamma_correction);
            color.y = color.y.powf(gamma_correction);
            color.z = color.z.powf(gamma_correction);

            *pixel = color_to_rgb(color);
        });

    // Graphics assume origin at bottom left corner of the screen
    // Our buffer writes pixels from top left corner. Simple fix, just flip it!
    image::imageops::flip_vertical_in_place(&mut img);
    // Timestamp & write
    let timestamp = Utc::now().timestamp();
    println!("{}", timestamp);
    img.save(format!("renders/{}.png", timestamp))
}
