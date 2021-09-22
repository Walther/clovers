//! Various internal helper functions for getting specific kinds of random values.

#[cfg(feature = "rand-crate")]
use crate::CloversRng;

use crate::{Float, Vec3, PI};
// TODO: fix trait import
#[cfg(feature = "rand-crate")]
#[cfg(not(target_arch = "spirv"))]
use rand::Rng;

#[cfg(target_arch = "spirv")]
use crate::FloatTrait;

/// Internal helper. Originally used for lambertian reflection with flaws
pub fn random_in_unit_sphere(rng: &mut CloversRng) -> Vec3 {
    let mut position: Vec3;
    // TODO: figure out a non-loop method
    // See https://github.com/RayTracing/raytracing.github.io/issues/765
    loop {
        position = 2.0 * Vec3::new(rng.gen::<Float>(), rng.gen::<Float>(), rng.gen::<Float>())
            - Vec3::new(1.0, 1.0, 1.0);
        // TODO: better ergonomics. nalgebra uses a reference, glam uses plain value
        #[cfg(target_arch = "spirv")]
        if position.length_squared() >= 1.0 {
            return position;
        }
        #[cfg(not(target_arch = "spirv"))]
        if position.magnitude_squared() >= 1.0 {
            return position;
        }
    }
}

/// Internal helper. Use this for the more correct "True Lambertian" reflection
pub fn random_unit_vector(rng: &mut CloversRng) -> Vec3 {
    let a: Float = rng.gen_range(0.0..2.0 * PI);
    let z: Float = rng.gen_range(-1.0..1.0);
    let r: Float = (1.0 - z * z).sqrt();
    Vec3::new(r * a.cos(), r * a.sin(), z)
}

/// Internal helper.
pub fn random_in_unit_disk(rng: &mut CloversRng) -> Vec3 {
    let mut position: Vec3;
    // TODO: figure out a non-loop method
    // See https://github.com/RayTracing/raytracing.github.io/issues/765
    loop {
        // TODO: understand this defocus disk thingy
        position =
            2.0 * Vec3::new(rng.gen::<Float>(), rng.gen::<Float>(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        // TODO: better ergonomics. nalgebra uses a reference, glam uses plain value
        #[cfg(target_arch = "spirv")]
        if position.dot(position) >= 1.0 {
            return position;
        }
        #[cfg(not(target_arch = "spirv"))]
        if position.dot(&position) >= 1.0 {
            return position;
        }
    }
}

/// Internal helper.
pub fn random_cosine_direction(rng: &mut CloversRng) -> Vec3 {
    let r1 = rng.gen::<Float>();
    let r2 = rng.gen::<Float>();
    let z = (1.0 - r2).sqrt();

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    Vec3::new(x, y, z)
}

/// Internal helper.
pub fn random_to_sphere(radius: Float, distance_squared: Float, rng: &mut CloversRng) -> Vec3 {
    let r1 = rng.gen::<Float>();
    let r2 = rng.gen::<Float>();
    let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * (1.0 - z * z).sqrt();
    let y = phi.sin() * (1.0 - z * z).sqrt();

    Vec3::new(x, y, z)
}

// For GPU random below
#[cfg(not(feature = "rand-crate"))]
use core::intrinsics::transmute;

/// Alternative random implementation, used on GPU only
#[cfg(not(feature = "rand-crate"))]
pub struct CloversRng {
    state: u32,
    a: u32,
    c: u32,
}
#[cfg(not(feature = "rand-crate"))]
impl CloversRng {
    /// Generate a random value
    pub fn gen<T>(&mut self) -> Float {
        // TODO: does this make any sense
        self.state = self.a * self.state + self.c;
        let f: Float;
        #[allow(unsafe_code)]
        unsafe {
            f = transmute::<u32, Float>(self.state);
        }

        // Return the fractional part of the float in order to keep the return value between 0..1
        f.fract()
    }

    /// Initialize the random number generator. This implementation is NOT actually based on entropy, but is named such due to api compatibility with rand::rngs::SmallRng
    pub fn from_entropy() -> Self {
        // https://en.wikipedia.org/wiki/Linear_congruential_generator#Parameters_in_common_use
        // "Numerical Recipes"
        let a = 1664525;
        let c = 1013904223;
        let state = a + c;
        CloversRng { state, a, c }
    }

    /// Initialize the random number generator.
    pub fn from_seed(seed: u32) -> Self {
        // https://en.wikipedia.org/wiki/Linear_congruential_generator#Parameters_in_common_use
        // "Numerical Recipes"
        let a = 1664525;
        let c = 1013904223;
        let state = seed + a + c;
        CloversRng { state, a, c }
    }

    /// Generate a random value in the given range
    pub fn gen_range<T>(&mut self, _range: core::ops::Range<T>) -> T
    where
        T: Into<Float>,
    {
        todo!()
    }
}
