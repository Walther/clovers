use clovers::interval::*;
use divan::{black_box, AllocProfiler};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

#[divan::bench]
fn new(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            let mut rng = SmallRng::from_os_rng();
            (rng.random(), rng.random())
        })
        .counter(1u32)
        .bench_values(|(a, b)| black_box(Interval::new(a, b)))
}

#[divan::bench]
fn combine(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            let mut rng = SmallRng::from_os_rng();
            let ab = random_interval(&mut rng);
            let cd = random_interval(&mut rng);
            (ab, cd)
        })
        .counter(1u32)
        .bench_values(|(ab, cd)| black_box(Interval::combine(&ab, &cd)))
}

#[divan::bench]
fn expand(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            let mut rng = SmallRng::from_os_rng();
            let ab = random_interval(&mut rng);
            let delta = rng.random();
            (ab, delta)
        })
        .counter(1u32)
        .bench_values(|(ab, delta)| black_box(ab.expand(delta)))
}

#[divan::bench]
fn size(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            let mut rng = SmallRng::from_os_rng();
            random_interval(&mut rng)
        })
        .counter(1u32)
        .bench_values(|ab| black_box(ab.size()))
}

// Helper functions

fn random_interval(rng: &mut SmallRng) -> Interval {
    let (a, b) = (rng.random(), rng.random());
    Interval::new(a, b)
}
