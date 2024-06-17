//! Runtime functions of the `clovers` renderer.

use clap::Args;

pub mod colorize;
pub mod draw_cpu;
pub mod json_scene;
pub mod normals;
pub mod render;
pub mod sampler;

// TODO: move this into a better place - but keep rustc happy with the imports
/// Global options
#[derive(Args, Debug)]
pub struct GlobalOptions {
    /// Enable some debug logging
    #[clap(long)]
    pub debug: bool,
    /// Suppress most of the text output
    #[clap(short, long)]
    pub quiet: bool,
}
