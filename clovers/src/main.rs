#![deny(clippy::all)]

// External imports
use chrono::Utc;
use clap::Clap;
use humantime::format_duration;
use image::{ImageBuffer, Rgb, RgbImage};
use std::{error::Error, fs, time::Instant};
use std::{fs::File, sync::Arc};

// Internal imports
use clovers::*;
pub mod draw;
use draw::draw;
use scenes::Scene;

// Configure CLI parameters
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Walther")]
pub struct Opts {
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
    println!(); // Empty line before progress bar

    // Read the given scene file
    let file = File::open(opts.input)?;
    let scene: Scene = scenes::initialize(file, opts.width, opts.height)?;
    let scene: Arc<Scene> = Arc::new(scene);

    // Note: live progress bar printed within draw
    let start = Instant::now();
    let pixelbuffer = draw(
        opts.width,
        opts.height,
        opts.samples,
        opts.max_depth,
        opts.gamma,
        scene,
    );

    // Translate our internal pixelbuffer into an Image buffer
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
    // Our coordinate system is weird in general, try flipping this way too.
    image::imageops::flip_horizontal_in_place(&mut img);
    // TODO: fix the coordinate system

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
