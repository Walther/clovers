use super::Opts;
use clovers::scenes::{self, Scene, SceneFile};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use tracing::info;

pub(crate) fn initialize<'scene>(
    path: &Path,
    opts: &Opts,
) -> Result<Box<Scene<'scene>>, std::boxed::Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    info!("Parsing the scene file");
    let scene_file: SceneFile = serde_json::from_str(&contents)?;
    info!("Initializing the scene");
    let scene: Scene = scenes::initialize(scene_file, opts.width, opts.height);
    info!("Count of nodes in the BVH tree: {}", scene.objects.count());
    Ok(Box::new(scene))
}
