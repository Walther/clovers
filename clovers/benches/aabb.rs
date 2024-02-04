use std::f32::{INFINITY, NEG_INFINITY};

use clovers::interval::Interval;
use clovers::ray::Ray;
use clovers::wavelength::random_wavelength;
use clovers::{aabb::*, Vec3};
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
            random_intervals(&mut rng)
        })
        .bench_values(|(ab, cd, ef)| black_box(AABB::new(ab, cd, ef)))
}

#[divan::bench]
fn hit(bencher: divan::Bencher) {
    bencher
        .with_inputs(random_aabb_and_ray)
        .bench_values(|(aabb, ray)| black_box(aabb.hit(&ray, NEG_INFINITY, INFINITY)))
}

#[divan::bench]
fn hit_old(bencher: divan::Bencher) {
    bencher
        .with_inputs(random_aabb_and_ray)
        .bench_values(|(aabb, ray)| {
            #[allow(deprecated)]
            black_box(aabb.hit_old(&ray, NEG_INFINITY, INFINITY))
        })
}

#[divan::bench]
fn hit_new(bencher: divan::Bencher) {
    bencher
        .with_inputs(random_aabb_and_ray)
        .bench_values(|(aabb, ray)| {
            #[allow(deprecated)]
            black_box(aabb.hit_new(&ray, NEG_INFINITY, INFINITY))
        })
}

// Helper functions

fn random_intervals(rng: &mut SmallRng) -> (Interval, Interval, Interval) {
    let (a, b, c, d, e, f) = black_box((
        rng.gen(),
        rng.gen(),
        rng.gen(),
        rng.gen(),
        rng.gen(),
        rng.gen(),
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
        direction: Vec3::new(rng.gen(), rng.gen(), rng.gen()),
        time: rng.gen(),
        wavelength: random_wavelength(rng),
    })
}

fn random_aabb_and_ray() -> (AABB, Ray) {
    let mut rng = SmallRng::from_entropy();
    let aabb = black_box(random_aabb(&mut rng));
    let ray = black_box(random_ray(&mut rng));
    (aabb, ray)
}
