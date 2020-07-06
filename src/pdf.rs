use crate::{onb::ONB, random::random_cosine_direction, Float, Vec3, PI};
use rand::prelude::*;

pub enum PDF {
    CosinePDF(CosinePDF),
}

impl PDF {
    pub fn value(&self, direction: Vec3) -> Float {
        match self {
            PDF::CosinePDF(p) => p.value(direction),
        }
    }
    pub fn generate(&self, rng: ThreadRng) -> Vec3 {
        match self {
            PDF::CosinePDF(p) => p.generate(rng),
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

// class cosine_pdf : public pdf {
//   public:
//       cosine_pdf(const vec3& w) { uvw.build_from_w(w); }

//       virtual double value(const vec3& direction) const {
//           auto cosine = dot(unit_vector(direction), uvw.w());
//           return (cosine <= 0) ? 0 : cosine/pi;
//       }

//       virtual vec3 generate() const {
//           return uvw.local(random_cosine_direction());
//       }

//   public:
//       onb uvw;
// };
