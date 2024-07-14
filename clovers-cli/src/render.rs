use clap::{Args, ValueEnum};
use humantime::format_duration;
use image::{ImageBuffer, ImageFormat, Rgb, RgbImage};
use img_parts::png::{Png, PngChunk};
use std::fs::File;
use std::io::Cursor;
use std::path::Path;
use std::{error::Error, fs, time::Instant};
use time::OffsetDateTime;
use tracing::{debug, info, Level};
use tracing_subscriber::fmt::time::UtcTime;

use crate::draw_cpu;
use crate::json_scene::initialize;
use crate::sampler::Sampler;
use crate::GlobalOptions;

#[derive(Args, Debug)]
pub struct RenderOptions {
    /// Input filename / location
    #[arg()]
    pub input: String,
    /// Output filename / location. Defaults to ./renders/unix_timestamp.png
    #[arg(short, long)]
    pub output: Option<String>,
    /// Width of the image in pixels.
    #[arg(short, long, default_value = "1024")]
    pub width: u32,
    /// Height of the image in pixels.
    #[arg(short, long, default_value = "1024")]
    pub height: u32,
    /// Number of samples to generate per each pixel.
    #[arg(short, long, default_value = "64")]
    pub samples: u32,
    /// Maximum evaluated bounce depth for each ray.
    #[arg(short = 'd', long, default_value = "64")]
    pub max_depth: u32,
    /// Rendering mode.
    #[arg(short = 'm', long, default_value = "path-tracing")]
    pub mode: RenderMode,
    /// Sampler to use for rendering.
    #[arg(long, default_value = "random")]
    pub sampler: Sampler,
    /// BVH construction algorithm.
    #[arg(long, default_value = "longest-axis")]
    pub bvh: BvhAlgorithm,
}

#[derive(Copy, Clone, Debug, PartialEq, ValueEnum)]
pub enum RenderMode {
    /// Full path tracing, the default
    PathTracing,
    /// Surface normals of the first hit
    NormalMap,
    /// Debug view for BVH ray hit count
    BvhTestCount,
    /// Debug view for primitive object ray hit count
    PrimitiveTestCount,
}

#[derive(Copy, Clone, Debug, PartialEq, ValueEnum)]
pub enum BvhAlgorithm {
    /// Split based on the longest axis of the current AABB
    LongestAxis,
    /// Split based on the Surface Area Heuristic.
    Sah,
}

// CLI usage somehow not detected
#[allow(dead_code)]
pub(crate) fn render(
    global_options: GlobalOptions,
    render_options: RenderOptions,
) -> Result<(), Box<dyn Error>> {
    let GlobalOptions { quiet, debug } = global_options;
    let RenderOptions {
        ref input,
        ref output,
        width,
        height,
        samples,
        max_depth,
        mode,
        sampler,
        bvh,
    } = render_options;

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
        if mode == RenderMode::NormalMap {
            println!("rendering a normalmap");
        } else {
            println!("{samples} samples per pixel");
            println!("using the {sampler} sampler");
            println!("{max_depth} max bounce depth");
        }
        println!(); // Empty line before progress bar
    }

    if sampler == Sampler::Blue && !([1, 2, 4, 8, 16, 32, 64, 128, 256].contains(&samples)) {
        panic!("the blue sampler only supports the following sample-per-pixel counts: [1, 2, 4, 8, 16, 32, 64, 128, 256]");
    }

    // TODO: improve ergonomics?
    let bvh_algorithm: clovers::bvh::BvhAlgorithm = match bvh {
        BvhAlgorithm::LongestAxis => clovers::bvh::BvhAlgorithm::LongestAxis,
        BvhAlgorithm::Sah => clovers::bvh::BvhAlgorithm::Sah,
    };

    let threads = std::thread::available_parallelism()?;

    info!("Reading the scene file");
    let path = Path::new(&input);
    let scene = match path.extension() {
        Some(ext) => match &ext.to_str() {
            Some("json") => initialize(path, bvh_algorithm, width, height),
            _ => panic!("Unknown file type"),
        },
        None => panic!("Unknown file type"),
    }?;

    info!("Calling draw()");
    let start = Instant::now();
    let pixelbuffer = draw_cpu::draw(&global_options, &render_options, &scene, sampler);
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
    img.write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)?;
    let mut png = Png::from_bytes(bytes.into())?;

    let common = format!(
        "Comment\0Rendered with the clovers path tracing engine. Scene file {input} rendered using the {mode:?} rendering mode at {width}x{height} resolution"
    );
    let details = match mode {
        RenderMode::PathTracing => {
            format!(", {samples} samples per pixel, {max_depth} max ray bounce depth.")
        }
        _ => ".".to_owned(),
    };
    let stats = format!("Rendering finished in {formatted_duration}, using {threads} threads.");
    let comment = format!("{common}{details} {stats}");

    let software = "Software\0https://github.com/walther/clovers".to_string();

    for metadata in [comment, software] {
        let bytes = metadata.as_bytes().to_owned();
        let chunk = PngChunk::new([b't', b'E', b'X', b't'], bytes.into());
        png.chunks_mut().push(chunk);
    }

    let target = match output {
        Some(filename) => filename.to_owned(),
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
