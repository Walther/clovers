//! Wrapper for GLTF materials.

use gltf::Material;
use rand::rngs::SmallRng;

use crate::{hitable::HitRecord, ray::Ray, Float};

use super::{MaterialTrait, ScatterRecord};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// GLTF Material wrapper type
pub struct GLTFMaterial {
    // #[cfg_attr(feature = "serde-derive", serde(skip))]
    // material: Material,
}

impl Default for GLTFMaterial {
    fn default() -> Self {
        todo!()
    }
}

impl GLTFMaterial {
    /// Initialize a new GLTF material wrapper
    #[must_use]
    pub fn new(_material: &Material) -> Self {
        Self {}
    }
}

impl MaterialTrait for GLTFMaterial {
    fn scatter(
        &self,
        _ray: &Ray,
        _hit_record: &HitRecord,
        _rng: &mut SmallRng,
    ) -> Option<ScatterRecord> {
        todo!()
    }

    fn scattering_pdf(
        &self,
        _ray: &Ray,
        _hit_record: &HitRecord,
        _scattered: &Ray,
        _rng: &mut SmallRng,
    ) -> Option<Float> {
        todo!()
    }
}
