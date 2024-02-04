use clovers::interval::*;
use divan::black_box;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

fn main() {
    divan::main();
}

#[divan::bench]
fn new(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            let mut rng = SmallRng::from_entropy();
            (rng.gen(), rng.gen())
        })
        .bench_values(|(a, b)| black_box(Interval::new(a, b)))
}

#[divan::bench]
fn new_from_intervals(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            let mut rng = SmallRng::from_entropy();
            let ab = random_interval(&mut rng);
            let cd = random_interval(&mut rng);
            (ab, cd)
        })
        .bench_values(|(ab, cd)| black_box(Interval::new_from_intervals(ab, cd)))
}

#[divan::bench]
fn expand(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            let mut rng = SmallRng::from_entropy();
            let ab = random_interval(&mut rng);
            let delta = rng.gen();
            (ab, delta)
        })
        .bench_values(|(ab, delta)| black_box(ab.expand(delta)))
}

#[divan::bench]
fn size(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            let mut rng = SmallRng::from_entropy();
            random_interval(&mut rng)
        })
        .bench_values(|ab| black_box(ab.size()))
}

// Helper functions

fn random_interval(rng: &mut SmallRng) -> Interval {
    let (a, b) = (rng.gen(), rng.gen());
    Interval::new(a, b)
}
