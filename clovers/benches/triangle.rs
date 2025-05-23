use clovers::hitable::HitableTrait;
use clovers::materials::Material;
use clovers::objects::Triangle;
use clovers::random::random_unit_vector;
use clovers::ray::Ray;
use clovers::wavelength::random_wavelength;
use clovers::Float;
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
            let rng = SmallRng::from_os_rng();
            let (triangle, ray) = random_triangle_and_ray();
            (triangle, ray, rng)
        })
        .counter(1u32)
        .bench_values(|(triangle, ray, mut rng)| {
            black_box(
                triangle
                    .hit(&ray, Float::NEG_INFINITY, Float::INFINITY, &mut rng)
                    .is_some(),
            )
        })
}

fn random_vec(rng: &mut SmallRng) -> Vec3 {
    Vec3::new(rng.random(), rng.random(), rng.random())
}

fn random_3_coords(rng: &mut SmallRng) -> (Vec3, Vec3, Vec3) {
    let a = random_vec(rng);
    let b = random_vec(rng);
    let c = random_vec(rng);
    (a, b, c)
}

fn random_triangle_ingredients() -> (Vec3, Vec3, Vec3, &'static Material) {
    let mut rng = SmallRng::from_os_rng();
    let material: &'static Material = Box::leak(Box::default());
    let (a, b, c) = random_3_coords(&mut rng);
    (a, b, c, material)
}

fn random_triangle() -> Triangle<'static> {
    let (a, b, c, material) = random_triangle_ingredients();
    Triangle::new(a, b, c, material)
}

fn random_ray() -> Ray {
    let mut rng = SmallRng::from_os_rng();
    black_box(Ray {
        origin: Vec3::new(0.0, 0.0, 0.0),
        direction: random_unit_vector(&mut rng),
        time: rng.random(),
        wavelength: random_wavelength(&mut rng),
    })
}

fn random_triangle_and_ray() -> (Triangle<'static>, Ray) {
    (random_triangle(), random_ray())
}
