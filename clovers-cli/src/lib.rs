//! Runtime functions of the `clovers` renderer.

use clap::Args;

pub mod debug_visualizations;
pub mod draw_cpu;
pub mod json_scene;
pub mod normals;
pub mod render;
pub mod sampler;
pub mod scenefile;
pub mod trace;
pub mod write;

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
