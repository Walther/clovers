use crate::{Float, Vec3, PI};
use rand::prelude::*;

/// Internal helper. Originally used for lambertian reflection with flaws
pub fn random_in_unit_sphere(mut rng: ThreadRng) -> Vec3 {
    let mut position: Vec3;
    loop {
        position = 2.0 * Vec3::new(rng.gen(), rng.gen(), rng.gen()) - Vec3::new(1.0, 1.0, 1.0);
        if position.magnitude_squared() >= 1.0 {
            return position;
        }
    }
}

/// Internal helper. Use this for the more correct "True Lambertian" reflection
pub fn random_unit_vector(mut rng: ThreadRng) -> Vec3 {
    let a: Float = rng.gen_range(0.0, 2.0 * PI);
    let z: Float = rng.gen_range(-1.0, 1.0);
    let r: Float = (1.0 - z * z).sqrt();
    Vec3::new(r * a.cos(), r * a.sin(), z)
}

/// Internal helper.
pub fn random_in_unit_disk(rng: &mut ThreadRng) -> Vec3 {
    let mut position: Vec3;
    loop {
        // TODO: understand this defocus disk thingy
        position = 2.0 * Vec3::new(rng.gen(), rng.gen(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        if position.dot(&position) >= 1.0 {
            return position;
        }
    }
}

/// Internal helper.
pub fn random_cosine_direction(mut rng: ThreadRng) -> Vec3 {
    let r1 = rng.gen::<Float>();
    let r2 = rng.gen::<Float>();
    let z = (1.0 - r2).sqrt();

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    Vec3::new(x, y, z)
}

/// Internal helper.
pub fn random_to_sphere(radius: Float, distance_squared: Float, mut rng: ThreadRng) -> Vec3 {
    let r1 = rng.gen::<Float>();
    let r2 = rng.gen::<Float>();
    let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * (1.0 - z * z).sqrt();
    let y = phi.sin() * (1.0 - z * z).sqrt();

    Vec3::new(x, y, z)
}
