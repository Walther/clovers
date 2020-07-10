#![deny(clippy::all)]
// A lot of loader functions etc, suppresses some warning noise
#![allow(dead_code)]

// External imports
use chrono::Utc;
use clap::Clap;
use humantime::format_duration;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;
use std::{error::Error, fs, sync::Arc, time::Instant};

// Internal imports
use clovers::*;
mod draw;
use draw::draw;
#[cfg(feature = "gui")]
mod draw_gui;
use camera::{Camera, CameraInit};
use color::Color;
#[cfg(feature = "gui")]
use draw_gui::draw_gui;
use hitable::{Hitable, HitableList};
use materials::{Dielectric, DiffuseLight};
use objects::{FlipFace, Sphere, XYRect, XZRect, YZRect};
use scenes::Scene;
use textures::{SolidColor, Texture};

// Configure CLI parameters
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Walther")]
struct Opts {
    /// Output filename / location. [default: renders/timestamp.png]
    #[clap(short, long)]
    output: Option<String>,
    /// Width of the image in pixels
    #[clap(short, long, default_value = "1024")]
    width: u32,
    /// Height of the image in pixels
    #[clap(short, long, default_value = "1024")]
    height: u32,
    /// Number of samples to generate per each pixel
    #[clap(short, long, default_value = "100")]
    samples: u32,
    /// Maximum evaluated bounce depth for each ray
    #[clap(short, long, default_value = "100")]
    max_depth: u32,
    /// Gamma correction value
    #[clap(short, long, default_value = "2.0")]
    gamma: Float,
    /// Optional GUI with iterative rendering
    #[clap(long)]
    gui: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "cli")]
    let opts: Opts = Opts::parse();

    println!("clovers 🍀    ray tracing in rust 🦀");
    println!("width:        {}", opts.width);
    println!("height:       {}", opts.height);
    println!("samples:      {}", opts.samples);
    println!("max depth:    {}", opts.max_depth);
    let rays: u64 =
        opts.width as u64 * opts.height as u64 * opts.samples as u64 * opts.max_depth as u64;
    println!("approx. rays: {}", rays);
    println!(); // Empty line before progress bar

    #[derive(Deserialize, Serialize, Debug)]
    enum Object {
        XZRect(XZRect),
        XYRect(XYRect),
        YZRect(YZRect),
        Sphere(Sphere),
        FlipFace(XZRect),
    }

    impl From<Object> for Hitable {
        fn from(obj: Object) -> Hitable {
            match obj {
                Object::XZRect(x) => Hitable::XZRect(x),
                Object::XYRect(x) => Hitable::XYRect(x),
                Object::YZRect(x) => Hitable::YZRect(x),
                Object::Sphere(x) => Hitable::Sphere(x),
                Object::FlipFace(x) => FlipFace::new(Hitable::XZRect(x)),
            }
        }
    }

    // TODO: temporary priority lights
    let small_light = DiffuseLight::new(SolidColor::new(Color::new(15.0, 15.0, 15.0)));
    let small_light_obj = XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, small_light);
    let sphere = Sphere::new(Vec3::new(190.0, 90.0, 190.0), 90.0, Dielectric::new(1.5));
    let mut lights = HitableList::new();
    lights.add(small_light_obj);
    lights.add(sphere);
    let lights = lights.into_hitable(); // TODO: fixme, silly
    let lights = Arc::new(lights);

    // TODO: temporary let's try this serde thing out
    #[derive(Serialize, Deserialize, Debug)]
    struct SceneFile {
        time_0: Float,
        time_1: Float,
        background_color: Color,
        camera: CameraInit,
        objects: Vec<Object>,
    }
    let mut file = File::open("scenes/scene.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let scene: SceneFile = serde_json::from_str(&contents)?;
    let time_0 = scene.time_0;
    let time_1 = scene.time_1;
    let rng = rand::thread_rng();
    let background_color = scene.background_color;
    let camera = Camera::new(
        scene.camera.look_from,
        scene.camera.look_at,
        scene.camera.up,
        scene.camera.vertical_fov,
        opts.width as Float / opts.height as Float,
        scene.camera.aperture,
        scene.camera.focus_distance,
        time_0,
        time_1,
    );
    let mut hitables = HitableList::new();
    for obj in scene.objects {
        hitables.add(obj.into());
    }
    let scene = Scene::new(hitables, camera, time_0, time_1, background_color, rng);

    // gui version
    if opts.gui {
        if cfg!(feature = "gui") {
            #[cfg(feature = "gui")]
            let _result = draw_gui(
                opts.width,
                opts.height,
                opts.samples,
                opts.max_depth,
                opts.gamma,
                scene,
                lights,
            );
            return Ok(());
        } else {
            println!("clovers not built with feature 'gui' enabled");
            return Ok(());
        }
    }

    // cli version
    let start = Instant::now();
    let img = draw(
        opts.width,
        opts.height,
        opts.samples,
        opts.max_depth,
        opts.gamma,
        scene,
        lights,
    )?; // Note: live progress bar printed within draw
    let duration = Instant::now() - start;

    println!(); // Empty line after progress bar
    println!("finished render in {}", format_duration(duration));

    // Write
    let target: String;
    match opts.output {
        Some(filename) => {
            target = filename;
        }
        None => {
            // Default to using a timestamp & `renders/` directory
            let timestamp = Utc::now().timestamp();
            fs::create_dir_all("renders")?;
            target = format!("renders/{}.png", timestamp);
        }
    };
    img.save(format!("{}", target))?;
    println!("output saved: {}", target);

    Ok(())
}
