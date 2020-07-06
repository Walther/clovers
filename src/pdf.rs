use crate::{hitable::Hitable, onb::ONB, random::random_cosine_direction, Float, Vec3, PI};
use rand::prelude::*;
use std::sync::Arc;

pub enum PDF {
    CosinePDF(CosinePDF),
    HitablePDF(HitablePDF),
}

impl PDF {
    pub fn value(&self, direction: Vec3, time: Float, rng: ThreadRng) -> Float {
        match self {
            PDF::CosinePDF(p) => p.value(direction),
            PDF::HitablePDF(p) => p.value(direction, time, rng),
        }
    }
    pub fn generate(&self, rng: ThreadRng) -> Vec3 {
        match self {
            PDF::CosinePDF(p) => p.generate(rng),
            PDF::HitablePDF(p) => p.generate(rng),
        }
    }
}

pub struct CosinePDF {
    uvw: ONB,
}

impl CosinePDF {
    pub fn new(w: Vec3) -> PDF {
        PDF::CosinePDF(CosinePDF {
            uvw: ONB::build_from_w(w),
        })
    }

    pub fn value(&self, direction: Vec3) -> Float {
        let cosine = direction.normalize().dot(&self.uvw.w);
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }

    pub fn generate(&self, rng: ThreadRng) -> Vec3 {
        self.uvw.local(random_cosine_direction(rng))
    }
}

pub struct HitablePDF {
    origin: Vec3,
    hitable: Arc<Hitable>,
}

impl HitablePDF {
    pub fn new(hitable: Hitable, origin: Vec3) -> PDF {
        PDF::HitablePDF(HitablePDF {
            origin,
            hitable: Arc::new(hitable),
        })
    }

    pub fn value(&self, direction: Vec3, time: Float, mut rng: ThreadRng) -> Float {
        self.hitable.pdf_value(self.origin, direction, time, rng)
    }

    pub fn generate(&self, rng: ThreadRng) -> Vec3 {
        self.hitable.random(self.origin, rng)
    }
}
