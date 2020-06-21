use rayon::prelude::*;

use rand::prelude::*;

use image::{ImageBuffer, ImageResult, Rgb, RgbImage};

use nalgebra::Vector3;

use std::time::Instant;

mod sphere;
use sphere::Sphere;
mod hitable;
use hitable::{HitRecord, Hitable, HitableList};
mod ray;
use ray::Ray;
mod camera;
use camera::Camera;

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
fn colorize(ray: &Ray, world: &dyn Hitable) -> Vec3 {
    let color: Vec3;

    if let Some(hit_record) = world.hit(&ray, 0.0, Float::MAX) {
        // Hit an object, colorize based on surface normals
        color = 0.5
            * Vec3::new(
                hit_record.normal.x + 1.0,
                hit_record.normal.y + 1.0,
                hit_record.normal.z + 1.0,
            );
    } else {
        // Background, blue-white gradient. Magic from tutorial.
        let unit_direction: Vec3 = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        color = (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);
    }

    color
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
    // Let's start dirty & hardcoded
    let width = 1200;
    let height = 600;
    let antialias_samples = 100;
    let mut img: RgbImage = ImageBuffer::new(width, height);
    let camera = Camera::default();
    let sphere1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let sphere2 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
    let world: HitableList = HitableList {
        hitables: vec![Box::new(sphere1), Box::new(sphere2)],
    };

    img.enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(x, y, pixel)| {
            let mut rng = rand::thread_rng();
            let mut color: Vec3 = Vec3::new(0.0, 0.0, 0.0);
            let mut u: Float;
            let mut v: Float;
            let mut ray: Ray;
            for _sample in 0..antialias_samples {
                u = (x as Float + rng.gen::<Float>()) / width as Float;
                v = (y as Float + rng.gen::<Float>()) / height as Float;
                ray = camera.get_ray(u, v);
                color += colorize(&ray, &world);
            }
            color /= antialias_samples as Float;

            *pixel = color_to_rgb(color);
        });

    img.save("renders/image.png")
}
