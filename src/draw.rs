use crate::{color::Color, colorize::colorize, ray::Ray, scenes, Float};
use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;

use scenes::Scene;

// Attempt using a more manual thread approach
use std::sync::{Arc, Mutex};
use std::thread;

/// The main drawing function, returns a Vec<Color> as a pixelbuffer.
pub fn draw(
    width: u32,
    height: u32,
    samples: u32,
    max_depth: u32,
    gamma: Float,
    scene: Scene,
) -> Vec<Color> {
    // Progress bar
    let pixels = (width * height) as u64;
    let bar = ProgressBar::new(pixels);
    bar.set_draw_delta(pixels / 1000);
    bar.set_style(ProgressStyle::default_bar().template(
        "Elapsed: {elapsed_precise}\nPixels:  {bar} {pos}/{len}\nETA:     {eta_precise}",
    ));

    let black = Color::new(0.0, 0.0, 0.0);
    let pixelbuffer = vec![black; pixels as usize];

    let safe_pixelbuffer = Arc::new(Mutex::new(pixelbuffer));
    let safe_scene = Arc::new(Mutex::new(scene));
    let safe_bar = Arc::new(Mutex::new(bar));

    // TODO: better work division?
    let cpus: u64 = num_cpus::get() as u64;
    let pixels_per_cpu = pixels / cpus;
    println!("thread count: {}", cpus);
    println!("pixels_per_cpu: {}", pixels_per_cpu);

    let handles = (0..cpus)
        .map(|cpu| {
            let scene = safe_scene.clone();
            let pixelbuffer = safe_pixelbuffer.clone();
            let bar = safe_bar.clone();
            thread::spawn(move || {
                // reusables
                let mut rng = rand::thread_rng();
                let mut color;
                let mut u: Float;
                let mut v: Float;
                let mut x: u64;
                let mut y: u64;
                let mut ray: Ray;

                let range = (cpu * pixels_per_cpu)..(cpu * pixels_per_cpu + pixels_per_cpu);
                println!("range: {} {}", range.start, range.end);
                for index in range {
                    x = index % width as u64;
                    y = index / width as u64;
                    color = Color::new(0.0, 0.0, 0.0);

                    // Multisampling for antialiasing
                    for _sample in 0..samples {
                        // unlock mutex
                        let scene = scene.lock().unwrap();

                        u = (x as Float + rng.gen::<Float>()) / width as Float;
                        v = (y as Float + rng.gen::<Float>()) / height as Float;
                        ray = scene.camera.get_ray(u, v, rng);
                        let new_color = colorize(&ray, &scene, 0, max_depth, rng);
                        // skip NaN and Infinity
                        if new_color.r.is_finite()
                            && new_color.g.is_finite()
                            && new_color.b.is_finite()
                        {
                            color += new_color;
                        }
                    }
                    color /= samples as Float;

                    color = color.gamma_correction(gamma);

                    let mut pixelbuffer = pixelbuffer.lock().unwrap();
                    pixelbuffer[index as usize] = color;

                    let bar = bar.lock().unwrap();
                    bar.inc(1);
                }
            })
        })
        .collect::<Vec<thread::JoinHandle<_>>>();

    for thread in handles {
        thread.join().unwrap();
    }

    let result = safe_pixelbuffer.lock().unwrap().clone();
    result
}
