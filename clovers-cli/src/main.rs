//! Command Line Interface for the raytracing renderer.
//!
//! CPU-based rendering is fully functional. GPU-based rendering is at early experimentation stage only.

#![deny(clippy::all)]

// External imports
use clap::Parser;
use human_format::Formatter;
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
mod draw_cpu;
mod json_scene;

// Configure CLI parameters
#[derive(Parser)]
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
    #[clap(short = 'd', long, default_value = "100")]
    max_depth: u32,
    /// Gamma correction value
    #[clap(short, long, default_value = "2.2")]
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
    /// Render a normal map only. Experimental feature.
    #[clap(long)]
    normalmap: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts: Opts = Opts::parse();

    if opts.debug {
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
    if !opts.quiet {
        println!("clovers 🍀    ray tracing in rust 🦀");
        println!("width:          {}", opts.width);
        println!("height:         {}", opts.height);
        println!("samples:        {}", opts.samples);
        println!("max depth:      {}", opts.max_depth);
        let rays: u64 =
            opts.width as u64 * opts.height as u64 * opts.samples as u64 * opts.max_depth as u64;
        println!("max total rays: {}", Formatter::new().format(rays as f64));
        println!(); // Empty line before progress bar
    }

    let renderopts: RenderOpts = RenderOpts {
        width: opts.width,
        height: opts.height,
        samples: opts.samples,
        max_depth: opts.max_depth,
        gamma: opts.gamma,
        quiet: opts.quiet,
        normalmap: opts.normalmap,
    };
    let threads = std::thread::available_parallelism()?;

    info!("Reading the scene file");
    let path = Path::new(&opts.input);
    let scene = match path.extension() {
        Some(ext) => match &ext.to_str() {
            Some("json") => json_scene::initialize(path, &opts),
            _ => panic!("Unknown file type"),
        },
        None => panic!("Unknown file type"),
    }?;

    info!("Calling draw()");
    let start = Instant::now();
    let pixelbuffer = match opts.gpu {
        // Note: live progress bar printed within draw_cpu::draw
        false => draw_cpu::draw(renderopts, &scene),
        true => unimplemented!("GPU accelerated rendering is currently unimplemented"),
    };
    info!("Drawing a pixelbuffer finished");

    info!("Converting pixelbuffer to an image");
    let width = opts.width;
    let height = opts.height;
    let mut img: RgbImage = ImageBuffer::new(width, height);
    img.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let index = y * width + x;
        let rgb = pixelbuffer[index as usize];
        let r = (rgb.red * 255.0) as u8;
        let g = (rgb.green * 255.0) as u8;
        let b = (rgb.blue * 255.0) as u8;
        *pixel = Rgb([r, g, b]);
    });

    // Graphics assume origin at bottom left corner of the screen
    // Our buffer writes pixels from top left corner. Simple fix, just flip it!
    image::imageops::flip_vertical_in_place(&mut img);
    // TODO: fix the coordinate system

    let duration = Instant::now() - start;
    let formatted_duration = format_duration(duration);
    info!("Finished render in {}", formatted_duration);

    if !opts.quiet {
        println!(); // Empty line after progress bar
        println!("Finished render in {}", formatted_duration);
    }

    info!("Writing an image file");

    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut bytes), ImageOutputFormat::Png)?;
    let mut png = Png::from_bytes(bytes.into())?;

    let comment = if opts.normalmap {
        format!("Comment\0{} rendered with the clovers raytracing engine at {}x{} in normalmap mode. finished render in {}, using {} threads", opts.input, opts.width, opts.height, formatted_duration, threads)
    } else {
        format!("Comment\0{} rendered with the clovers raytracing engine at {}x{}, {} samples per pixel, {} max ray bounce depth. finished render in {}, using {} threads", opts.input, opts.width, opts.height, opts.samples, opts.max_depth, formatted_duration, threads)
    };
    let software = "Software\0https://github.com/walther/clovers".to_string();

    for metadata in [comment, software] {
        let bytes = metadata.as_bytes().to_owned();
        let chunk = PngChunk::new([b't', b'E', b'X', b't'], bytes.into());
        png.chunks_mut().push(chunk);
    }

    let target = match opts.output {
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
