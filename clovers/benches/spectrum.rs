use clovers::spectrum::*;
use clovers::wavelength::*;
use divan::black_box;
use divan::AllocProfiler;
use palette::white_point::E;
use palette::Xyz;
use rand::rngs::SmallRng;
use rand::SeedableRng;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

#[divan::bench]
fn xyz_to_p(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            let mut rng = SmallRng::from_entropy();
            let wave = random_wavelength(&mut rng);
            let xyz: Xyz<E> = Xyz::new(1.0, 1.0, 1.0);
            (wave, xyz)
        })
        .counter(1u32)
        .bench_values(|(wave, xyz)| black_box(spectrum_xyz_to_p(wave, xyz)))
}
