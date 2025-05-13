use clovers::interval::Interval;
use clovers::random::random_unit_vector;
use clovers::ray::Ray;
use clovers::wavelength::random_wavelength;
use clovers::Float;
use clovers::{aabb::*, Vec3};
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
            random_intervals(&mut rng)
        })
        .counter(1u32)
        .bench_values(|(ab, cd, ef)| black_box(AABB::new(ab, cd, ef)))
}

#[divan::bench]
fn hit(bencher: divan::Bencher) {
    bencher
        .with_inputs(random_aabb_and_ray)
        .counter(1u32)
        .bench_values(|(aabb, ray)| black_box(aabb.hit(&ray, Float::NEG_INFINITY, Float::INFINITY)))
}

// Helper functions

fn random_intervals(rng: &mut SmallRng) -> (Interval, Interval, Interval) {
    let (a, b, c, d, e, f) = black_box((
        rng.random(),
        rng.random(),
        rng.random(),
        rng.random(),
        rng.random(),
        rng.random(),
    ));
    let ab = Interval::new(a, b);
    let cd = Interval::new(c, d);
    let ef = Interval::new(e, f);
    (ab, cd, ef)
}

fn random_aabb(rng: &mut SmallRng) -> AABB {
    let (ab, cd, ef) = random_intervals(rng);
    black_box(AABB::new(ab, cd, ef))
}

fn random_ray(rng: &mut SmallRng) -> Ray {
    black_box(Ray {
        origin: Vec3::new(0.0, 0.0, 0.0),
        direction: random_unit_vector(rng),
        time: rng.random(),
        wavelength: random_wavelength(rng),
    })
}

fn random_aabb_and_ray() -> (AABB, Ray) {
    let mut rng = SmallRng::from_os_rng();
    let aabb = black_box(random_aabb(&mut rng));
    let ray = black_box(random_ray(&mut rng));
    (aabb, ray)
}
