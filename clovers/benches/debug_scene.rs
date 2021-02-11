use std::{env, fs::File, sync::Arc};

use clovers::draw::draw;
use clovers::scenes::{initialize, Scene};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let width = 100;
    let height = 100;
    let samples = 10;
    let max_depth = 10;
    let gamma = 2.0;
    let mut path = env::current_dir().unwrap();
    path.push("scenes");
    path.push("debug_scene.json");
    dbg!(&path);
    let file = File::open(path).unwrap();
    let scene: Scene = initialize(file, width, height).unwrap();
    let scene: Arc<Scene> = Arc::new(scene);
    c.bench_function("debug scene", |b| {
        b.iter(|| {
            draw(
                width,
                height,
                samples,
                max_depth,
                gamma,
                black_box(scene.clone()),
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
