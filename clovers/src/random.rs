//! Various internal helper functions for getting specific kinds of random values.

use crate::{Direction, Float, Vec2, Vec3, PI};
use nalgebra::Unit;
use rand::rngs::SmallRng;
use rand::Rng;
use rand_distr::{Distribution, UnitDisc, UnitSphere};

/// Internal helper.
#[must_use]
pub fn random_unit_vector(rng: &mut SmallRng) -> Direction {
    let v = UnitSphere.sample(rng).into();
    Unit::new_normalize(v)
}

/// Internal helper.
#[must_use]
pub fn random_in_unit_disk(rng: &mut SmallRng) -> Vec2 {
    let v: [Float; 2] = UnitDisc.sample(rng);
    Vec2::new(v[0], v[1])
}

/// Internal helper.
#[must_use]
pub fn random_cosine_direction(rng: &mut SmallRng) -> Direction {
    let r1: Float = rng.gen();
    let r2: Float = rng.gen();
    let z = (1.0 - r2).sqrt();

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    let v: Vec3 = Vec3::new(x, y, z);
    Unit::new_normalize(v)
}

/// Internal helper.
#[must_use]
pub fn random_on_hemisphere(normal: Vec3, rng: &mut SmallRng) -> Direction {
    let in_unit_sphere: Direction = random_unit_vector(rng);
    if in_unit_sphere.dot(&normal) > 0.0 {
        // In the same hemisphere as the normal
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}
