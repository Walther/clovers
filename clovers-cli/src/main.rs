//! Command Line Interface for the `clovers` raytracing renderer.

#![deny(clippy::all)]

// External imports
use clap::{Args, Parser, Subcommand};
use std::error::Error;

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
mod render;
use render::{render, RenderParams};
#[doc(hidden)]
mod sampler;
#[doc(hidden)]
mod validate;
use validate::{validate, ValidateParams};

/// clovers 🍀 path tracing renderer
#[derive(Parser)]
pub struct Cli {
    #[command(flatten)]
    global_options: GlobalOptions,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Args, Debug)]
/// Global options
pub struct GlobalOptions {
    /// Enable some debug logging
    #[clap(long)]
    debug: bool,
    /// Suppress most of the text output
    #[clap(short, long)]
    quiet: bool,
}

#[derive(Subcommand, Debug)]
/// Subcommands for the CLI
pub enum Commands {
    #[command(arg_required_else_help = true)]
    /// Render a given scene file
    Render(RenderParams),
    #[command(arg_required_else_help = true)]
    /// Validate a given scene file
    Validate(ValidateParams),
}

#[doc(hidden)]
fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    match args.command {
        Commands::Render(params) => render(args.global_options, params),
        Commands::Validate(params) => validate(params),
    }
}
