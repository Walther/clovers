use clovers::spectrum::*;
use clovers::wavelength::*;
use divan::black_box;
use palette::white_point::E;
use palette::Xyz;
use rand::rngs::SmallRng;
use rand::Rng;
use rand::SeedableRng;

fn main() {
    divan::main();
}

#[divan::bench]
fn xyz_to_p(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            let mut rng = SmallRng::from_entropy();
            let wave = random_wavelength(&mut rng);
            let xyz: Xyz<E> = Xyz::new(rng.gen(), rng.gen(), rng.gen());
            (wave, xyz)
        })
        .bench_values(|(wave, xyz)| black_box(spectrum_xyz_to_p(wave, xyz)))
}
