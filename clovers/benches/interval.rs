use clovers::{interval::*, Float};
use divan::black_box;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

fn main() {
    divan::main();
}

#[divan::bench]
fn new() -> Interval {
    let mut rng = SmallRng::from_entropy();
    let (a, b) = black_box((rng.gen(), rng.gen()));
    Interval::new(a, b)
}

#[divan::bench]
fn new_from_intervals() -> Interval {
    let mut rng = SmallRng::from_entropy();
    let (a, b) = black_box((rng.gen(), rng.gen()));
    let (c, d) = black_box((rng.gen(), rng.gen()));
    let ab = Interval::new(a, b);
    let cd = Interval::new(c, d);
    Interval::new_from_intervals(ab, cd)
}

#[divan::bench]
fn expand() -> Interval {
    let mut rng = SmallRng::from_entropy();
    let (a, b) = black_box((rng.gen(), rng.gen()));
    let ab = Interval::new(a, b);
    ab.expand(rng.gen())
}

#[divan::bench]
fn size() -> Float {
    let mut rng = SmallRng::from_entropy();
    let (a, b) = black_box((rng.gen(), rng.gen()));
    let ab = Interval::new(a, b);
    ab.size()
}
