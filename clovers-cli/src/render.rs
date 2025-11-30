use std::path::Path;
use std::{error::Error, fs, time::Instant};

use clap::{Args, ValueEnum};
use humantime::format_duration;
use time::OffsetDateTime;
use tracing::{debug, info, Level};
use tracing_subscriber::fmt::time::UtcTime;

use crate::draw_cpu;
use crate::json_scene::initialize;
use crate::sampler::Sampler;
use crate::write;
use crate::GlobalOptions;

#[derive(Args, Clone, Debug)]
pub struct RenderOptions {
    /// Input filename / location
    #[arg(short, long)]
    pub input: String,
    /// Output file path, without extension. Defaults to `./renders/unix_timestamp`.
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
    #[arg(long, default_value = "sah")]
    pub bvh: BvhAlgorithm,
    /// File format selection for the output.
    /// Multiple formats can be provided to save the same image in multiple formats.
    #[arg(short, long, default_value = "png", num_args = 1..)]
    pub formats: Vec<Format>,
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
    /// Split at the Longest Axis Midpoint of the current AABB
    Lam,
    /// Split based on the Surface Area Heuristic.
    Sah,
}

#[derive(Copy, Clone, Debug, PartialEq, ValueEnum)]
pub enum Format {
    /// Portable Network Graphics, lossless, standard dynamic range
    Png,
    /// OpenEXR, high dynamic range
    Exr,
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
        ref formats,
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
        BvhAlgorithm::Lam => clovers::bvh::BvhAlgorithm::Lam,
        BvhAlgorithm::Sah => clovers::bvh::BvhAlgorithm::Sah,
    };

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
    let duration = Instant::now() - start;
    let duration = format_duration(duration);
    info!("Finished render in {}", duration);
    if !quiet {
        println!("Finished render in {}", duration);
    }

    for format in formats {
        let extension = match format {
            Format::Png => "png",
            Format::Exr => "exr",
        };

        let target = match output {
            Some(filename) => format!("{filename}.{extension}"),
            None => {
                // Default to using a timestamp & `renders/` directory
                let timestamp = OffsetDateTime::now_utc().unix_timestamp();
                fs::create_dir_all("renders")?;
                format!("renders/{timestamp}.{extension}")
            }
        };

        match format {
            Format::Png => write::png(&pixelbuffer, &target, &duration, &render_options),
            Format::Exr => write::exr(&pixelbuffer, width, height, &target),
        }?;

        info!("Image saved to {}", target);
        println!("Image saved to: {}", target);
    }

    Ok(())
}
