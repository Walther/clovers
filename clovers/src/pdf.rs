//! Probability density functions

#![allow(missing_docs)] // TODO: Lots of undocumented things for now

use crate::{
    Box, Direction, Float, PI, Position,
    hitable::{Hitable, HitableTrait},
    onb::ONB,
    random::{random_cosine_direction, random_unit_vector},
    wavelength::Wavelength,
};
use enum_dispatch::enum_dispatch;
use rand::{Rng, rngs::SmallRng};

#[enum_dispatch(PDFTrait)]
#[derive(Debug, Clone)]
pub enum PDF<'scene> {
    CosinePDF(CosinePDF),
    SpherePDF(SpherePDF),
    HitablePDF(HitablePDF<'scene>),
    MixturePDF(MixturePDF<'scene>),
    ZeroPDF(ZeroPDF),
}

#[enum_dispatch]
pub trait PDFTrait {
    fn value(
        &self,
        direction: Direction,
        wavelength: Wavelength,
        time: Float,
        rng: &mut SmallRng,
    ) -> Float;

    // TODO: verify correctness & explain for all impls
    fn generate(&self, rng: &mut SmallRng) -> Position;
}

#[derive(Debug, Clone)]
pub struct CosinePDF {
    uvw: ONB,
}

impl CosinePDF {
    #[must_use]
    pub fn new(w: Direction) -> Self {
        CosinePDF {
            uvw: ONB::build_from_w(w),
        }
    }
}

impl PDFTrait for CosinePDF {
    fn value(
        &self,
        direction: Direction,
        _wavelength: Wavelength,
        _time: Float,
        _rng: &mut SmallRng,
    ) -> Float {
        let cosine = direction.normalize().dot(&self.uvw.w);
        if cosine <= 0.0 { 0.0 } else { cosine / PI }
    }

    fn generate(&self, rng: &mut SmallRng) -> Position {
        *self.uvw.local(random_cosine_direction(rng))
    }
}

#[derive(Debug, Clone)]
pub struct HitablePDF<'scene> {
    origin: Position,
    hitable: &'scene Hitable<'scene>,
}

impl<'scene> HitablePDF<'scene> {
    #[must_use]
    pub fn new(hitable: &'scene Hitable, origin: Position) -> Self {
        HitablePDF { origin, hitable }
    }
}

impl PDFTrait for HitablePDF<'_> {
    fn value(
        &self,
        direction: Direction,
        wavelength: Wavelength,
        time: Float,
        rng: &mut SmallRng,
    ) -> Float {
        self.hitable
            .pdf_value(self.origin, direction, wavelength, time, rng)
    }

    fn generate(&self, rng: &mut SmallRng) -> Position {
        self.hitable.random(self.origin, rng)
    }
}

#[derive(Debug, Clone)]
pub struct MixturePDF<'scene> {
    // Box to prevent infinite size
    pdf1: Box<PDF<'scene>>,
    pdf2: Box<PDF<'scene>>,
}

impl<'scene> MixturePDF<'scene> {
    #[must_use]
    pub fn new(pdf1: PDF<'scene>, pdf2: PDF<'scene>) -> Self {
        MixturePDF {
            pdf1: Box::new(pdf1),
            pdf2: Box::new(pdf2),
        }
    }
}

impl PDFTrait for MixturePDF<'_> {
    fn value(
        &self,
        direction: Direction,
        wavelength: Wavelength,
        time: Float,
        rng: &mut SmallRng,
    ) -> Float {
        0.5 * self.pdf1.value(direction, wavelength, time, rng)
            + 0.5 * self.pdf2.value(direction, wavelength, time, rng)
    }

    fn generate(&self, rng: &mut SmallRng) -> Position {
        if rng.random::<bool>() {
            self.pdf1.generate(rng)
        } else {
            self.pdf2.generate(rng)
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SpherePDF {}

impl SpherePDF {
    #[must_use]
    pub fn new() -> Self {
        SpherePDF {}
    }
}

impl PDFTrait for SpherePDF {
    fn value(
        &self,
        _direction: Direction,
        _wavelength: Wavelength,
        _time: Float,
        _rng: &mut SmallRng,
    ) -> Float {
        1.0 / (4.0 * PI)
    }

    fn generate(&self, rng: &mut SmallRng) -> Position {
        // TODO: verify correctness! radius?
        *random_unit_vector(rng)
    }
}

// TODO: this is an ugly hack due to tutorial saying `srec.pdf_ptr = 0;` in 12.2 Handling Specular for Metal
#[derive(Debug, Clone)]
pub struct ZeroPDF {}

impl ZeroPDF {
    #[must_use]
    pub fn new() -> Self {
        ZeroPDF {}
    }
}

impl PDFTrait for ZeroPDF {
    fn value(
        &self,
        _direction: Direction,
        _wavelength: Wavelength,
        _time: Float,
        _rng: &mut SmallRng,
    ) -> Float {
        0.0
    }

    fn generate(&self, rng: &mut SmallRng) -> Position {
        *random_unit_vector(rng)
    }
}

impl Default for ZeroPDF {
    fn default() -> Self {
        Self::new()
    }
}
