use clovers::scenes::{initialize, Scene, SceneFile};
use clovers::RenderOpts;
use clovers_runtime::draw_cpu::draw;
use clovers_runtime::sampler::Sampler;
use divan::{black_box, AllocProfiler};

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;
const OPTS: RenderOpts = RenderOpts {
    width: WIDTH,
    height: HEIGHT,
    samples: 1,
    max_depth: 100,
    quiet: true,
    normalmap: false,
};

#[divan::bench]
fn draw_cornell(bencher: divan::Bencher) {
    bencher
        .with_inputs(get_cornell)
        .counter(1u32)
        .bench_values(|scene| black_box(draw(OPTS, &scene, Sampler::Random)))
}

fn get_cornell<'scene>() -> Scene<'scene> {
    const INPUT: &str = include_str!("../../scenes/cornell.json");
    let scene_file: SceneFile = serde_json::from_str(INPUT).unwrap();
    let scene: Scene = initialize(scene_file, WIDTH, HEIGHT);
    scene
}
