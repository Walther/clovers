//! Probability density functions

#![allow(missing_docs)] // TODO: Lots of undocumented things for now

use crate::{
    hitable::{Hitable, HitableTrait},
    onb::ONB,
    random::{random_cosine_direction, random_unit_vector},
    wavelength::Wavelength,
    Box, Direction, Float, Position, PI,
};
use enum_dispatch::enum_dispatch;
use rand::rngs::SmallRng;
use rand::Rng;

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
    #[must_use]
    fn value(
        &self,
        direction: Direction,
        wavelength: Wavelength,
        time: Float,
        rng: &mut SmallRng,
    ) -> Float;

    // TODO: verify correctness & explain for all impls
    #[must_use]
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
    #[must_use]
    fn value(
        &self,
        direction: Direction,
        _wavelength: Wavelength,
        _time: Float,
        _rng: &mut SmallRng,
    ) -> Float {
        let cosine = direction.normalize().dot(&self.uvw.w);
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }

    #[must_use]
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

impl<'scene> PDFTrait for HitablePDF<'scene> {
    #[must_use]
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

    #[must_use]
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

impl<'scene> PDFTrait for MixturePDF<'scene> {
    #[must_use]
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

    #[must_use]
    fn generate(&self, rng: &mut SmallRng) -> Position {
        if rng.gen::<bool>() {
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
    #[must_use]
    fn value(
        &self,
        _direction: Direction,
        _wavelength: Wavelength,
        _time: Float,
        _rng: &mut SmallRng,
    ) -> Float {
        1.0 / (4.0 * PI)
    }

    #[must_use]
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
    #[must_use]
    fn value(
        &self,
        _direction: Direction,
        _wavelength: Wavelength,
        _time: Float,
        _rng: &mut SmallRng,
    ) -> Float {
        0.0
    }

    #[must_use]
    fn generate(&self, rng: &mut SmallRng) -> Position {
        *random_unit_vector(rng)
    }
}

impl Default for ZeroPDF {
    fn default() -> Self {
        Self::new()
    }
}
