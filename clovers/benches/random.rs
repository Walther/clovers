use clovers::{Vec3, random::*};
use divan::{AllocProfiler, black_box};
use rand::{SeedableRng, rngs::SmallRng};

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

#[divan::bench]
fn unit_vector(bencher: divan::Bencher) {
    bencher
        .with_inputs(SmallRng::from_os_rng)
        .counter(1u32)
        .bench_values(|mut rng| random_unit_vector(black_box(&mut rng)))
}

#[divan::bench]
fn unit_disk(bencher: divan::Bencher) {
    bencher
        .with_inputs(SmallRng::from_os_rng)
        .counter(1u32)
        .bench_values(|mut rng| random_in_unit_disk(black_box(&mut rng)))
}

#[divan::bench]
fn cosine_direction(bencher: divan::Bencher) {
    bencher
        .with_inputs(SmallRng::from_os_rng)
        .counter(1u32)
        .bench_values(|mut rng| random_cosine_direction(black_box(&mut rng)))
}

#[divan::bench]
fn hemisphere(bencher: divan::Bencher) {
    bencher
        .with_inputs(SmallRng::from_os_rng)
        .counter(1u32)
        .bench_values(|mut rng| {
            let normal = Vec3::new(1.0, 0.0, 0.0);
            random_on_hemisphere(normal, black_box(&mut rng))
        })
}
