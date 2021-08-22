#![deny(clippy::all)]

// External imports
use chrono::Utc;
use clap::Clap;
use env_logger::Env;
use humantime::format_duration;
use image::{ImageBuffer, Rgb, RgbImage};
use log::{debug, info};
use std::fs::File;
use std::io::Read;
use std::{error::Error, fs, time::Instant};

// Internal imports
use clovers::*;
mod draw_cpu;
mod draw_gpu;
use scenes::*;

// Configure CLI parameters
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Walther", name = "clovers")]
struct Opts {
    /// Input filename / location
    #[clap(short, long)]
    input: String,
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
    /// Suppress most of the text output
    #[clap(short, long)]
    quiet: bool,
    /// Use the GPU draw process instead of CPU
    #[clap(long)]
    gpu: bool,
    /// Enable some debug logging
    #[clap(long)]
    debug: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts: Opts = Opts::parse();

    if opts.debug {
        env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
        debug!("Debug logging enabled");
    } else {
        env_logger::Builder::from_env(Env::default().default_filter_or("error")).init();
    }

    // Pretty printing output, unless in quiet mode
    if !opts.quiet {
        println!("clovers 🍀    ray tracing in rust 🦀");
        println!("width:        {}", opts.width);
        println!("height:       {}", opts.height);
        println!("samples:      {}", opts.samples);
        println!("max depth:    {}", opts.max_depth);
        let rays: u64 =
            opts.width as u64 * opts.height as u64 * opts.samples as u64 * opts.max_depth as u64;
        println!("approx. rays: {}", rays);
        println!(); // Empty line before progress bar
    }

    info!("Reading the scene file");
    let mut file = File::open(opts.input)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    info!("Parsing the scene file");
    let scene_file: SceneFile = serde_json::from_str(&contents)?;
    info!("Initializing the scene");
    let scene: Scene = scenes::initialize(scene_file, opts.width, opts.height);

    info!("Calling draw()");
    let start = Instant::now();
    let pixelbuffer = match opts.gpu {
        // Note: live progress bar printed within draw_cpu::draw
        false => draw_cpu::draw(
            opts.width,
            opts.height,
            opts.samples,
            opts.max_depth,
            opts.gamma,
            opts.quiet,
            scene,
        ),
        true => futures::executor::block_on(draw_gpu::draw(
            opts.width,
            opts.height,
            opts.samples,
            opts.max_depth,
            opts.gamma,
            opts.quiet,
            scene,
        )),
    };
    info!("Drawing a pixelbuffer finished");

    info!("Converting pixelbuffer to an image");
    let width = opts.width;
    let height = opts.height;
    let mut img: RgbImage = ImageBuffer::new(width, height);
    img.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let index = y * width + x;
        *pixel = Rgb(pixelbuffer[index as usize].to_rgb_u8());
    });

    // Graphics assume origin at bottom left corner of the screen
    // Our buffer writes pixels from top left corner. Simple fix, just flip it!
    image::imageops::flip_vertical_in_place(&mut img);
    // TODO: fix the coordinate system

    let duration = Instant::now() - start;

    if !opts.quiet {
        println!(); // Empty line after progress bar
        info!("finished render in {}", format_duration(duration));
    }

    info!("Writing an image file");
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
    img.save(target.to_string())?;
    info!("Image saved to {}", target);
    println!("Image saved to: {}", target);

    Ok(())
}
