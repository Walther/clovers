use clap::Args;
use clovers::bvh::BvhAlgorithm;
use std::{error::Error, path::Path};

use crate::json_scene;

#[derive(Args, Debug)]
pub struct ValidateParams {
    /// Input filename / location
    #[arg()]
    input: String,
}

pub(crate) fn validate(params: ValidateParams) -> Result<(), Box<dyn Error>> {
    // TODO: better validation, better error messages using `miette`
    // TODO: don't panic!

    let ValidateParams { input } = params;
    let path = Path::new(&input);
    let scene = match path.extension() {
        Some(ext) => match &ext.to_str() {
            Some("json") => json_scene::initialize(path, BvhAlgorithm::LongestAxis, 1, 1),
            _ => panic!("Unknown file type"),
        },
        None => panic!("Unknown file type"),
    }?;
    drop(scene);
    println!("✅ Scene file successfully validated!");
    Ok(())
}
