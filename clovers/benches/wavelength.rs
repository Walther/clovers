use clovers::wavelength::*;
use divan::black_box;
use rand::rngs::SmallRng;
use rand::SeedableRng;

fn main() {
    divan::main();
}

#[divan::bench]
fn random(bencher: divan::Bencher) {
    bencher
        .with_inputs(SmallRng::from_entropy)
        .bench_values(|mut rng| black_box(random_wavelength(&mut rng)))
}

#[divan::bench]
fn rotate(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            let mut rng = SmallRng::from_entropy();
            random_wavelength(&mut rng)
        })
        .bench_values(|wave| black_box(rotate_wavelength(wave)))
}

#[divan::bench]
fn into_xyz(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            let mut rng = SmallRng::from_entropy();
            random_wavelength(&mut rng)
        })
        .bench_values(|wave| black_box(wavelength_into_xyz(wave)))
}
