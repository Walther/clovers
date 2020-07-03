#![deny(clippy::all)]
// A lot of loader functions etc, suppresses some warning noise
#![allow(dead_code)]

use rand::prelude::*;

use image::ImageResult;

use nalgebra::Vector3;

use chrono::Utc;
use humantime::format_duration;
use std::time::{Duration, Instant};

mod hitable;
mod objects;
mod ray;
use ray::Ray;
mod camera;
mod color;
mod colorize;
mod draw;
mod materials;
mod scenes;
use draw::draw;
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
const WIDTH: u32 = 2048;
const HEIGHT: u32 = 2048;
const SAMPLES: u32 = 100;
const MAX_DEPTH: u32 = 100;

fn main() -> ImageResult<()> {
    println!("clovers 🍀    ray tracing in rust 🦀");
    println!("width:        {}", WIDTH);
    println!("height:       {}", HEIGHT);
    println!("samples:      {}", SAMPLES);
    println!("max depth:    {}", MAX_DEPTH);
    let rays: u64 = WIDTH as u64 * HEIGHT as u64 * SAMPLES as u64 * MAX_DEPTH as u64;
    println!("aprrox. rays: {}", rays);
    println!(""); // Empty line before progress bar

    let start = Instant::now();
    let img = draw()?; // Note: live progress bar printed within draw
    let duration = Instant::now() - start;

    println!(""); // Empty line after progress bar
    println!("finished render in {}", format_duration(duration));

    // Timestamp & write
    let timestamp = Utc::now().timestamp();
    println!("output saved: renders/{}.png", timestamp);
    img.save(format!("renders/{}.png", timestamp))?;
    Ok(())
}
