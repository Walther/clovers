//! Probability density functions

#![allow(missing_docs)] // TODO: Lots of undocumented things for now

use crate::CloversRng;
use crate::{hitable::Hitable, onb::ONB, random::random_cosine_direction, Box, Float, Vec3, PI};
// TODO: fix trait import
#[cfg(feature = "rand-crate")]
#[cfg(not(target_arch = "spirv"))]
use rand::Rng;

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float as FloatTrait;

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
pub enum PDF<'a> {
    CosinePDF(CosinePDF),
    HitablePDF(HitablePDF<'a>),
    MixturePDF(MixturePDF<'a>),
    ZeroPDF(ZeroPDF),
}

impl<'a> PDF<'a> {
    pub fn value(&self, direction: Vec3, time: Float, rng: &mut CloversRng) -> Float {
        match self {
            PDF::CosinePDF(p) => p.value(direction, time, rng),
            PDF::HitablePDF(p) => p.value(direction, time, rng),
            PDF::MixturePDF(p) => p.value(direction, time, rng),
            PDF::ZeroPDF(p) => p.value(direction, time, rng),
        }
    }
    pub fn generate(&self, rng: &mut CloversRng) -> Vec3 {
        match self {
            PDF::CosinePDF(p) => p.generate(rng),
            PDF::HitablePDF(p) => p.generate(rng),
            PDF::MixturePDF(p) => p.generate(rng),
            PDF::ZeroPDF(p) => p.generate(rng),
        }
    }
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
pub struct CosinePDF {
    uvw: ONB,
}

impl CosinePDF {
    pub fn new(w: Vec3) -> Self {
        CosinePDF {
            uvw: ONB::build_from_w(w),
        }
    }

    pub fn value(&self, direction: Vec3, _time: Float, _rng: &mut CloversRng) -> Float {
        let cosine = direction.normalize().dot(&self.uvw.w);
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }

    pub fn generate(&self, rng: &mut CloversRng) -> Vec3 {
        self.uvw.local(random_cosine_direction(rng))
    }
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
pub struct HitablePDF<'a> {
    origin: Vec3,
    hitable: &'a Hitable,
}

impl<'a> HitablePDF<'a> {
    pub fn new(hitable: &'a Hitable, origin: Vec3) -> Self {
        HitablePDF { origin, hitable }
    }

    pub fn value(&self, direction: Vec3, time: Float, rng: &mut CloversRng) -> Float {
        self.hitable.pdf_value(self.origin, direction, time, rng)
    }

    pub fn generate(&self, rng: &mut CloversRng) -> Vec3 {
        self.hitable.random(self.origin, rng)
    }
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
pub struct MixturePDF<'a> {
    // Box to prevent infinite size
    pdf1: Box<PDF<'a>>,
    pdf2: Box<PDF<'a>>,
}

impl<'a> MixturePDF<'a> {
    pub fn new(pdf1: PDF<'a>, pdf2: PDF<'a>) -> Self {
        MixturePDF {
            pdf1: Box::new(pdf1),
            pdf2: Box::new(pdf2),
        }
    }

    pub fn value(&self, direction: Vec3, time: Float, rng: &mut CloversRng) -> Float {
        0.5 * self.pdf1.value(direction, time, rng) + 0.5 * self.pdf2.value(direction, time, rng)
    }

    pub fn generate(&self, rng: &mut CloversRng) -> Vec3 {
        if rng.gen::<bool>() {
            self.pdf1.generate(rng)
        } else {
            self.pdf2.generate(rng)
        }
    }
}

// TODO: this is an ugly hack due to tutorial saying `srec.pdf_ptr = 0;` in 12.2 Handling Specular for Metal
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
pub struct ZeroPDF {}

impl ZeroPDF {
    pub fn new() -> Self {
        ZeroPDF {}
    }

    pub fn value(&self, _direction: Vec3, _time: Float, _rng: &mut CloversRng) -> Float {
        0.0
    }

    pub fn generate(&self, _rng: &mut CloversRng) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}

impl Default for ZeroPDF {
    fn default() -> Self {
        Self::new()
    }
}
