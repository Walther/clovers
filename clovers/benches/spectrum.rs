use clovers::{spectrum::*, wavelength::*};
use divan::{AllocProfiler, black_box};
use palette::{Xyz, white_point::E};
use rand::{SeedableRng, rngs::SmallRng};

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

#[divan::bench]
fn xyz_to_p(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            let mut rng = SmallRng::from_os_rng();
            let wave = random_wavelength(&mut rng);
            let color: Xyz<E> = Xyz::new(1.0, 1.0, 1.0);
            (wave, color)
        })
        .counter(1u32)
        .bench_values(|(wave, color)| black_box(spectral_power(color, wave)))
}
