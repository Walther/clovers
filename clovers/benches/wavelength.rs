use clovers::wavelength::*;
use divan::{black_box, AllocProfiler};
use rand::rngs::SmallRng;
use rand::SeedableRng;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

#[divan::bench]
fn random(bencher: divan::Bencher) {
    bencher
        .with_inputs(SmallRng::from_entropy)
        .counter(1u32)
        .bench_values(|mut rng| black_box(random_wavelength(&mut rng)))
}

#[divan::bench]
fn rotate(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            let mut rng = SmallRng::from_entropy();
            random_wavelength(&mut rng)
        })
        .counter(1u32)
        .bench_values(|wave| black_box(rotate_wavelength(wave)))
}

#[divan::bench]
fn into_xyz(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            let mut rng = SmallRng::from_entropy();
            random_wavelength(&mut rng)
        })
        .counter(1u32)
        .bench_values(|wave| black_box(wavelength_into_xyz(wave)))
}
