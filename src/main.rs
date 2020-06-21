use rayon::prelude::*;

use rand::prelude::*;

use image::{ImageBuffer, ImageResult, Rgb, RgbImage};

use nalgebra::Vector3;

use std::{sync::Arc, time::Instant};

mod sphere;
use sphere::Sphere;
mod hitable;
use hitable::{HitRecord, Hitable, HitableList};
mod ray;
use ray::Ray;
mod camera;
use camera::Camera;
mod material;
use material::{Dielectric, Lambertian, Material, Metal};

const SHADOW_SMOOTHING: Float = 0.001;
const GAMMA: Float = 2.0;
const WIDTH: u32 = 1200;
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

// Internal helper
pub fn random_in_unit_sphere(mut rng: ThreadRng) -> Vec3 {
    let mut position: Vec3;
    loop {
        position = 2.0 * Vec3::new(rng.gen(), rng.gen(), rng.gen()) - Vec3::new(1.0, 1.0, 1.0);
        if position.magnitude_squared() >= 1.0 {
            return position;
        }
    }
}

/// The main coloring function
fn colorize(ray: &Ray, world: &dyn Hitable, depth: u32, rng: ThreadRng) -> Vec3 {
    let color: Vec3;

    if let Some(hit_record) = world.hit(&ray, SHADOW_SMOOTHING, Float::MAX) {
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
        return color;
    } else {
        // Background, blue-white gradient. Magic from tutorial.
        let unit_direction: Vec3 = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        color = (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);
        return color;
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
    let camera_position: Vec3 = Vec3::new(-2.0, 2.0, 1.0);
    let camera_targt: Vec3 = Vec3::new(0.0, 0.0, -1.0);
    let camera_up: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let fov: Float = 90.0;
    let aspect_ratio: Float = WIDTH as Float / HEIGHT as Float;
    let camera = Camera::new(camera_position, camera_targt, camera_up, fov, aspect_ratio);

    let sphere1 = Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Arc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5))),
    );
    let sphere2 = Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
    );
    let sphere3 = Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2))),
    );
    let sphere4 = Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Arc::new(Dielectric::new(1.5)),
    );
    let sphere5 = Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.45,
        Arc::new(Dielectric::new(1.5)),
    );
    let world: HitableList = HitableList {
        hitables: vec![
            Box::new(sphere1),
            Box::new(sphere2),
            Box::new(sphere3),
            Box::new(sphere4),
            Box::new(sphere5),
        ],
    };

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
                ray = camera.get_ray(u, v);
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
    img.save("renders/image.png")
}
