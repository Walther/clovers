use clovers::bvh::BvhAlgorithm;
use clovers::scenes::Scene;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use tracing::info;

use crate::scenefile::SceneFile;

pub fn initialize<'scene>(
    path: &Path,
    bvh_algorithm: BvhAlgorithm,
    width: u32,
    height: u32,
) -> Result<Scene<'scene>, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    info!("Parsing the scene file");
    let scene_file: SceneFile = serde_json::from_str(&contents)?;
    info!("Initializing the scene");
    let scene: Scene = SceneFile::initialize(scene_file, bvh_algorithm, width, height);
    info!("Count of nodes in the BVH tree: {}", scene.bvh_root.count());
    Ok(scene)
}
