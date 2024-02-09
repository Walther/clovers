use std::f32::{INFINITY, NEG_INFINITY};

use clovers::hitable::HitableTrait;
use clovers::materials::Material;
use clovers::objects::Triangle;
use clovers::ray::Ray;
use clovers::wavelength::random_wavelength;
use clovers::Vec3;
use divan::black_box;
use divan::AllocProfiler;
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
        .with_inputs(random_triangle_ingredients)
        .counter(1u32)
        .bench_values(|(a, b, c, material)| black_box(Triangle::new(a, b, c, material)))
}

#[divan::bench]
fn hit(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            let rng = SmallRng::from_entropy();
            let (triangle, ray) = random_triangle_and_ray();
            (triangle, ray, rng)
        })
        .counter(1u32)
        .bench_values(|(triangle, ray, mut rng)| {
            black_box(
                triangle
                    .hit(&ray, NEG_INFINITY, INFINITY, &mut rng)
                    .is_some(),
            )
        })
}

fn random_vec(rng: &mut SmallRng) -> Vec3 {
    Vec3::new(rng.gen(), rng.gen(), rng.gen())
}

fn random_3_coords(rng: &mut SmallRng) -> (Vec3, Vec3, Vec3) {
    let a = random_vec(rng);
    let b = random_vec(rng);
    let c = random_vec(rng);
    (a, b, c)
}

fn random_triangle_ingredients() -> (Vec3, Vec3, Vec3, &'static Material) {
    let mut rng = SmallRng::from_entropy();
    let material: &'static Material = Box::leak(Box::default());
    let (a, b, c) = random_3_coords(&mut rng);
    (a, b, c, material)
}

fn random_triangle() -> Triangle<'static> {
    let (a, b, c, material) = random_triangle_ingredients();
    Triangle::new(a, b, c, material)
}

fn random_ray() -> Ray {
    let mut rng = SmallRng::from_entropy();
    black_box(Ray {
        origin: Vec3::new(0.0, 0.0, 0.0),
        direction: Vec3::new(rng.gen(), rng.gen(), rng.gen()),
        time: rng.gen(),
        wavelength: random_wavelength(&mut rng),
    })
}

fn random_triangle_and_ray() -> (Triangle<'static>, Ray) {
    (random_triangle(), random_ray())
}
