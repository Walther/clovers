use clovers::{random::*, Vec3};
use divan::black_box;
use rand::rngs::SmallRng;
use rand::SeedableRng;

fn main() {
    divan::main();
}

#[divan::bench]
fn unit_vector() -> Vec3 {
    let mut rng = SmallRng::from_entropy();
    random_unit_vector(black_box(&mut rng))
}

#[divan::bench]
fn unit_disk() -> Vec3 {
    let mut rng = SmallRng::from_entropy();
    random_in_unit_disk(black_box(&mut rng))
}

#[divan::bench]
fn cosine_direction() -> Vec3 {
    let mut rng = SmallRng::from_entropy();
    random_cosine_direction(black_box(&mut rng))
}

#[divan::bench]
fn hemisphere() -> Vec3 {
    let mut rng = SmallRng::from_entropy();
    let normal = Vec3::new(1.0, 0.0, 0.0);
    random_on_hemisphere(normal, black_box(&mut rng))
}
