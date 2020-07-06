#![deny(clippy::all)]
// A lot of loader functions etc, suppresses some warning noise
#![allow(dead_code)]

use rand::prelude::*;

use nalgebra::Vector3;

use chrono::Utc;
use humantime::format_duration;

use std::{error::Error, time::Instant};

use clap::Clap;

mod hitable;
mod objects;
mod ray;
use ray::Ray;
mod camera;
mod color;
mod colorize;
mod draw;
mod draw_gui;
mod materials;
mod scenes;
use draw::draw;
mod onb;
mod pdf;
mod perlin;
mod random;
mod textures;

#[cfg(feature = "gui")]
use draw_gui::draw_gui;

// Handy aliases for internal use
type Float = f32;
pub const PI: Float = std::f32::consts::PI as Float;
type Vec3 = Vector3<Float>;
const SHADOW_EPSILON: Float = 0.001;
const RECT_EPSILON: Float = 0.0001;
const CONSTANT_MEDIUM_EPSILON: Float = 0.0001;

// Configure CLI parameters
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Walther")]
struct Opts {
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
    let opts: Opts = Opts::parse();

    println!("clovers 🍀    ray tracing in rust 🦀");
    println!("width:        {}", opts.width);
    println!("height:       {}", opts.height);
    println!("samples:      {}", opts.samples);
    println!("max depth:    {}", opts.max_depth);
    let rays: u64 =
        opts.width as u64 * opts.height as u64 * opts.samples as u64 * opts.max_depth as u64;
    println!("approx. rays: {}", rays);
    println!(""); // Empty line before progress bar

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

    println!(""); // Empty line after progress bar
    println!("finished render in {}", format_duration(duration));

    // Timestamp & write
    let timestamp = Utc::now().timestamp();
    println!("output saved: renders/{}.png", timestamp);
    img.save(format!("renders/{}.png", timestamp))?;
    Ok(())
}
