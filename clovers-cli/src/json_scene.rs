use clovers::scenes::{self, Scene, SceneFile};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use tracing::info;

pub fn initialize<'scene>(
    path: &Path,
    width: u32,
    height: u32,
) -> Result<Scene<'scene>, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    info!("Parsing the scene file");
    let scene_file: SceneFile = serde_json::from_str(&contents)?;
    info!("Initializing the scene");
    let scene: Scene = scenes::initialize(scene_file, width, height);
    info!("Count of nodes in the BVH tree: {}", scene.hitables.count());
    Ok(scene)
}
