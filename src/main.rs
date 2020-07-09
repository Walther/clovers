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
use std::{error::Error, fs, time::Instant};

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
use hitable::Hitable;
use objects::XZRect;
use textures::Texture;

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

    if opts.gui {
        if cfg!(feature = "gui") {
            #[cfg(feature = "gui")]
            let _result = draw_gui(opts.width, opts.height, opts.samples);
            return Ok(());
        } else {
            println!("clovers not built with feature 'gui' enabled");
            return Ok(());
        }
    }

    // TODO: temporary let's try this serde thing out
    #[derive(Serialize, Deserialize, Debug)]
    struct SceneFile {
        background_color: Color,
        camera: CameraInit,
        objects: Vec<XZRect>,
    }
    let mut file = File::open("scenes/scene.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let scene: SceneFile = serde_json::from_str(&contents)?;
    let camera = Camera::new(
        scene.camera.look_from,
        scene.camera.look_at,
        scene.camera.up,
        scene.camera.vertical_fov,
        opts.width as Float / opts.height as Float,
        scene.camera.aperture,
        scene.camera.focus_distance,
        scene.camera.time_0,
        scene.camera.time_1,
    );
    dbg!(&scene.objects);
    return Ok(());
    // TODO: temp early return

    // png writing version
    let start = Instant::now();
    let img = draw(
        opts.width,
        opts.height,
        opts.samples,
        opts.max_depth,
        opts.gamma,
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
