#![deny(clippy::all)]

// TODO: this is a temporary hack: main.rs mostly copied from the CLI version
// Eventually, this GUI version should have, well, a GUI for adjusting these parameters.

// External imports
use clap::Clap;
use std::error::Error;
use std::fs::File;
use std::io::Read;

// Internal imports
use clovers::*;
mod draw_gui;
use draw_gui::draw_gui;
use scenes::*;

// Configure CLI parameters
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Walther")]
struct Opts {
    /// Input filename / location
    #[clap(short, long)]
    input: String,
    /// Output filename / location. [default: renders/timestamp.png]
    // #[clap(short, long)]
    // output: Option<String>,
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
    // #[clap(long)]
    // gpu: bool,
    /// Enable some debug logging
    // #[clap(long)]
    // debug: bool,
    /// Render a normal map only. Experimental feature.
    #[clap(long)]
    normalmap: bool,
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
    let mut file = File::open(opts.input)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    let scene_file: SceneFile = serde_json::from_str(&contents)?;
    let scene: Scene = scenes::initialize(scene_file, opts.width, opts.height);

    let renderopts: RenderOpts = RenderOpts {
        width: opts.width,
        height: opts.height,
        samples: opts.samples,
        max_depth: opts.max_depth,
        gamma: opts.gamma,
        quiet: opts.quiet,
        normalmap: opts.normalmap,
    };

    // TODO: write result to file
    // TODO: have an actual UI for things
    let _result = draw_gui(renderopts, scene);

    Ok(())
}
