//! Probability density functions

#![allow(missing_docs)] // TODO: Lots of undocumented things for now

use crate::random::random_in_unit_sphere;
use crate::{hitable::Hitable, onb::ONB, random::random_cosine_direction, Box, Float, Vec3, PI};
use rand::rngs::SmallRng;
use rand::Rng;

#[derive(Debug)]
pub enum PDF {
    CosinePDF(CosinePDF),
    SpherePDF(SpherePDF),
    HitablePDF(HitablePDF),
    MixturePDF(MixturePDF),
    ZeroPDF(ZeroPDF),
}

impl PDF {
    #[must_use]
    pub fn value(&self, direction: Vec3, time: Float, rng: &mut SmallRng) -> Float {
        match self {
            PDF::CosinePDF(p) => p.value(direction, time, rng),
            PDF::SpherePDF(p) => p.value(direction, time, rng),
            PDF::HitablePDF(p) => p.value(direction, time, rng),
            PDF::MixturePDF(p) => p.value(direction, time, rng),
            PDF::ZeroPDF(p) => p.value(direction, time, rng),
        }
    }
    #[must_use]
    pub fn generate(&self, rng: &mut SmallRng) -> Vec3 {
        match self {
            PDF::CosinePDF(p) => p.generate(rng),
            PDF::SpherePDF(p) => p.generate(rng),
            PDF::HitablePDF(p) => p.generate(rng),
            PDF::MixturePDF(p) => p.generate(rng),
            PDF::ZeroPDF(p) => p.generate(rng),
        }
    }
}

#[derive(Debug)]
pub struct CosinePDF {
    uvw: ONB,
}

impl CosinePDF {
    #[must_use]
    pub fn new(w: Vec3) -> Self {
        CosinePDF {
            uvw: ONB::build_from_w(w),
        }
    }

    #[must_use]
    pub fn value(&self, direction: Vec3, _time: Float, _rng: &mut SmallRng) -> Float {
        let cosine = direction.normalize().dot(&self.uvw.w);
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }

    #[must_use]
    pub fn generate(&self, rng: &mut SmallRng) -> Vec3 {
        self.uvw.local(random_cosine_direction(rng))
    }
}

#[derive(Debug)]
pub struct HitablePDF {
    origin: Vec3,
    hitable: Hitable,
}

impl HitablePDF {
    #[must_use]
    pub fn new(hitable: Hitable, origin: Vec3) -> Self {
        HitablePDF { origin, hitable }
    }

    #[must_use]
    pub fn value(&self, direction: Vec3, time: Float, rng: &mut SmallRng) -> Float {
        self.hitable.pdf_value(self.origin, direction, time, rng)
    }

    #[must_use]
    pub fn generate(&self, rng: &mut SmallRng) -> Vec3 {
        self.hitable.random(self.origin, rng)
    }
}

#[derive(Debug)]
pub struct MixturePDF {
    // Box to prevent infinite size
    pdf1: Box<PDF>,
    pdf2: Box<PDF>,
}

impl MixturePDF {
    #[must_use]
    pub fn new(pdf1: PDF, pdf2: PDF) -> Self {
        MixturePDF {
            pdf1: Box::new(pdf1),
            pdf2: Box::new(pdf2),
        }
    }

    #[must_use]
    pub fn value(&self, direction: Vec3, time: Float, rng: &mut SmallRng) -> Float {
        0.5 * self.pdf1.value(direction, time, rng) + 0.5 * self.pdf2.value(direction, time, rng)
    }

    #[must_use]
    pub fn generate(&self, rng: &mut SmallRng) -> Vec3 {
        if rng.gen::<bool>() {
            self.pdf1.generate(rng)
        } else {
            self.pdf2.generate(rng)
        }
    }
}

#[derive(Debug, Default)]
pub struct SpherePDF {}

impl SpherePDF {
    #[must_use]
    pub fn new() -> Self {
        SpherePDF {}
    }

    #[allow(clippy::unused_self)]
    #[must_use]
    pub fn value(&self, _direction: Vec3, _time: Float, _rng: &mut SmallRng) -> Float {
        1.0 / (4.0 * PI)
    }

    #[allow(clippy::unused_self)]
    #[must_use]
    pub fn generate(&self, rng: &mut SmallRng) -> Vec3 {
        random_in_unit_sphere(rng)
    }
}

// TODO: this is an ugly hack due to tutorial saying `srec.pdf_ptr = 0;` in 12.2 Handling Specular for Metal
#[derive(Debug)]
pub struct ZeroPDF {}

impl ZeroPDF {
    #[must_use]
    pub fn new() -> Self {
        ZeroPDF {}
    }

    #[allow(clippy::unused_self)]
    #[must_use]
    pub fn value(&self, _direction: Vec3, _time: Float, _rng: &mut SmallRng) -> Float {
        0.0
    }

    #[allow(clippy::unused_self)]
    #[must_use]
    pub fn generate(&self, _rng: &mut SmallRng) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}

impl Default for ZeroPDF {
    fn default() -> Self {
        Self::new()
    }
}
