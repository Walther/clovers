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
fn new() -> AABB {
    let mut rng = SmallRng::from_entropy();
    black_box(random_aabb(&mut rng))
}

#[divan::bench]
fn hit() -> bool {
    let mut rng = SmallRng::from_entropy();
    let aabb = black_box(random_aabb(&mut rng));
    let ray = black_box(random_ray(&mut rng));
    black_box(aabb.hit(&ray, NEG_INFINITY, INFINITY))
}

#[divan::bench]
fn hit_old() -> bool {
    let mut rng = SmallRng::from_entropy();
    let aabb = black_box(random_aabb(&mut rng));
    let ray = black_box(random_ray(&mut rng));
    #[allow(deprecated)]
    black_box(aabb.hit_old(&ray, NEG_INFINITY, INFINITY))
}

#[divan::bench]
fn hit_new() -> bool {
    let mut rng = SmallRng::from_entropy();
    let aabb = black_box(random_aabb(&mut rng));
    let ray = black_box(random_ray(&mut rng));
    #[allow(deprecated)]
    black_box(aabb.hit_new(&ray, NEG_INFINITY, INFINITY))
}

// Helper functions

fn random_aabb(rng: &mut SmallRng) -> AABB {
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
    black_box(AABB::new(ab, cd, ef))
}

fn random_ray(rng: &mut SmallRng) -> Ray {
    black_box(Ray {
        origin: Vec3::new(rng.gen(), rng.gen(), rng.gen()),
        direction: Vec3::new(rng.gen(), rng.gen(), rng.gen()),
        time: rng.gen(),
        wavelength: random_wavelength(rng),
    })
}
