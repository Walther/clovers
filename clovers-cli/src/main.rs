//! Command Line Interface for the `clovers` raytracing renderer.

#![deny(clippy::all)]

// External imports
use clap::Parser;
use humantime::format_duration;
use image::{ImageBuffer, ImageOutputFormat, Rgb, RgbImage};
use img_parts::png::{Png, PngChunk};
use std::fs::File;
use std::io::Cursor;
use std::path::Path;
use std::{error::Error, fs, time::Instant};
use time::OffsetDateTime;
use tracing::{debug, info, Level};
use tracing_subscriber::fmt::time::UtcTime;

// Internal imports
use clovers::*;
#[doc(hidden)]
mod colorize;
#[doc(hidden)]
mod draw_cpu;
#[doc(hidden)]
mod json_scene;
#[doc(hidden)]
pub mod normals;
#[doc(hidden)]
mod sampler;
use sampler::Sampler;

/// Command line parameters for the `clovers` raytracing renderer.
#[derive(Parser)]
#[clap(version = "0.1.0", author = "Walther", name = "clovers")]
pub struct Opts {
    /// Input filename / location
    #[clap(short, long)]
    input: String,
    /// Output filename / location. Default: renders/unix_timestamp.png
    #[clap(short, long)]
    output: Option<String>,
    /// Width of the image in pixels. Default: 1024
    #[clap(short, long, default_value = "1024")]
    width: u32,
    /// Height of the image in pixels. Default: 1024
    #[clap(short, long, default_value = "1024")]
    height: u32,
    /// Number of samples to generate per each pixel. Default: 64
    #[clap(short, long, default_value = "64")]
    samples: u32,
    /// Maximum evaluated bounce depth for each ray. Default: 64
    #[clap(short = 'd', long, default_value = "64")]
    max_depth: u32,
    /// Suppress most of the text output
    #[clap(short, long)]
    quiet: bool,
    /// Use the GPU draw process instead of CPU
    #[clap(long)]
    gpu: bool,
    /// Enable some debug logging
    #[clap(long)]
    debug: bool,
    /// Render a normal map only. Experimental feature.
    #[clap(long)]
    normalmap: bool,
    /// Sampler to use for rendering. Experimental feature.
    #[clap(long, default_value = "random")]
    sampler: Sampler,
}

#[doc(hidden)]
fn main() -> Result<(), Box<dyn Error>> {
    let Opts {
        input,
        output,
        width,
        height,
        samples,
        max_depth,
        quiet,
        gpu,
        debug,
        normalmap,
        sampler,
    } = Opts::parse();

    if debug {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .with_timer(UtcTime::rfc_3339())
            .init();
        debug!("Debug logging enabled");
    } else {
        tracing_subscriber::fmt()
            .with_max_level(Level::ERROR)
            .with_timer(UtcTime::rfc_3339())
            .init();
    }

    // Pretty printing output, unless in quiet mode
    if !quiet {
        println!("clovers 🍀 path tracing renderer");
        println!();
        println!("{width}x{height} resolution");
        println!("{samples} samples per pixel");
        println!("using the {sampler} sampler");
        println!("{max_depth} max bounce depth");
        println!(); // Empty line before progress bar
    }

    if sampler == Sampler::Blue && !([1, 2, 4, 8, 16, 32, 128, 256].contains(&samples)) {
        panic!("the blue sampler only supports the following sample-per-pixel counts: [1, 2, 4, 8, 16, 32, 128, 256]");
    }

    let renderopts: RenderOpts = RenderOpts {
        width,
        height,
        samples,
        max_depth,
        quiet,
        normalmap,
    };
    let threads = std::thread::available_parallelism()?;

    info!("Reading the scene file");
    let path = Path::new(&input);
    let scene = match path.extension() {
        Some(ext) => match &ext.to_str() {
            Some("json") => json_scene::initialize(path, width, height),
            _ => panic!("Unknown file type"),
        },
        None => panic!("Unknown file type"),
    }?;

    info!("Calling draw()");
    let start = Instant::now();
    let pixelbuffer = match gpu {
        // Note: live progress bar printed within draw_cpu::draw
        false => draw_cpu::draw(renderopts, &scene, sampler),
        true => unimplemented!("GPU accelerated rendering is currently unimplemented"),
    };
    info!("Drawing a pixelbuffer finished");

    info!("Converting pixelbuffer to an image");
    let mut img: RgbImage = ImageBuffer::new(width, height);
    img.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let index = y * width + x;
        *pixel = Rgb(pixelbuffer[index as usize].into());
    });

    // Graphics assume origin at bottom left corner of the screen
    // Our buffer writes pixels from top left corner. Simple fix, just flip it!
    image::imageops::flip_vertical_in_place(&mut img);
    // TODO: fix the coordinate system

    let duration = Instant::now() - start;
    let formatted_duration = format_duration(duration);
    info!("Finished render in {}", formatted_duration);

    if !quiet {
        println!("Finished render in {}", formatted_duration);
    }

    info!("Writing an image file");

    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut bytes), ImageOutputFormat::Png)?;
    let mut png = Png::from_bytes(bytes.into())?;

    let comment = if normalmap {
        format!("Comment\0{input} rendered with the clovers raytracing engine at {width}x{height} in normalmap mode. finished render in {formatted_duration}, using {threads} threads")
    } else {
        format!("Comment\0{input} rendered with the clovers raytracing engine at {width}x{height}, {samples} samples per pixel, {max_depth} max ray bounce depth. finished render in {formatted_duration}, using {threads} threads")
    };
    let software = "Software\0https://github.com/walther/clovers".to_string();

    for metadata in [comment, software] {
        let bytes = metadata.as_bytes().to_owned();
        let chunk = PngChunk::new([b't', b'E', b'X', b't'], bytes.into());
        png.chunks_mut().push(chunk);
    }

    let target = match output {
        Some(filename) => filename,
        None => {
            // Default to using a timestamp & `renders/` directory
            let timestamp = OffsetDateTime::now_utc().unix_timestamp();
            fs::create_dir_all("renders")?;
            format!("renders/{}.png", timestamp)
        }
    };

    let output = File::create(&target)?;
    png.encoder().write_to(output)?;

    info!("Image saved to {}", target);
    println!("Image saved to: {}", target);

    Ok(())
}
