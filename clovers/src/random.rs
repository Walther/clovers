//! Various internal helper functions for getting specific kinds of random values.

use crate::{Float, Vec3, PI};
use rand::distributions::Uniform;
use rand::rngs::SmallRng;
use rand::Rng;

/// Internal helper. Originally used for lambertian reflection with flaws
#[must_use]
#[inline]
pub fn random_in_unit_sphere(rng: &mut SmallRng) -> Vec3 {
    let mut position: Vec3;
    let distribution = Uniform::new(-1.0, 1.0);

    loop {
        position = Vec3::new(
            rng.sample(distribution),
            rng.sample(distribution),
            rng.sample(distribution),
        );
        if position.magnitude_squared() <= 1.0 {
            return position;
        }
    }
}

/// Internal helper. Use this for the more correct "True Lambertian" reflection
#[must_use]
pub fn random_unit_vector(rng: &mut SmallRng) -> Vec3 {
    random_in_unit_sphere(rng).normalize()
}

/// Internal helper.
#[must_use]
pub fn random_in_unit_disk(rng: &mut SmallRng) -> Vec3 {
    let mut position: Vec3;
    let distribution = Uniform::new(-1.0, 1.0);

    loop {
        position = Vec3::new(
            rng.sample(distribution),
            rng.sample(distribution),
            0.0, // z component zero
        );
        if position.magnitude_squared() <= 1.0 {
            return position;
        }
    }
}

/// Internal helper.
#[must_use]
pub fn random_cosine_direction(rng: &mut SmallRng) -> Vec3 {
    let r1: Float = rng.gen();
    let r2: Float = rng.gen();
    let z = (1.0 - r2).sqrt();

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    // TODO: should this be normalized?
    Vec3::new(x, y, z)
}

/// Internal helper.
#[must_use]
pub fn random_in_hemisphere(normal: Vec3, rng: &mut SmallRng) -> Vec3 {
    let in_unit_sphere: Vec3 = random_in_unit_sphere(rng);
    if in_unit_sphere.dot(&normal) > 0.0 {
        // In the same hemisphere as the normal
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}
