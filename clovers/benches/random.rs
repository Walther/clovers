use clovers::{random::*, Vec3};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::rngs::SmallRng;
#[allow(unused_imports)]
use rand::SeedableRng;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = SmallRng::from_entropy();

    c.bench_function("random in unit sphere", |b| {
        b.iter(|| random_in_unit_sphere(black_box(&mut rng)))
    });

    c.bench_function("random in unit disk", |b| {
        b.iter(|| random_in_unit_disk(black_box(&mut rng)))
    });

    c.bench_function("random unit vector", |b| {
        b.iter(|| random_unit_vector(black_box(&mut rng)))
    });

    c.bench_function("random cosine direction", |b| {
        b.iter(|| random_cosine_direction(black_box(&mut rng)))
    });

    let normal = Vec3::new(1.0, 0.0, 0.0);
    c.bench_function("random in hemisphere", |b| {
        b.iter(|| random_in_hemisphere(normal, black_box(&mut rng)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
