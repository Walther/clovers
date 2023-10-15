//! Isotropic material.

use super::{MaterialTrait, MaterialType, ScatterRecord};
use crate::{
    hitable::HitRecord,
    pdf::{SpherePDF, PDF},
    ray::Ray,
    textures::{Texture, TextureTrait},
    Float, PI,
};
use rand::prelude::SmallRng;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// Isotropic material. Used in [`ConstantMedium`](crate::objects::constant_medium). TODO: understand this!
pub struct Isotropic {
    #[cfg_attr(feature = "serde-derive", serde(default))]
    albedo: Texture,
}

impl MaterialTrait for Isotropic {
    /// Returns a [`ScatterRecord`] based on the [`HitRecord`] coordinates and the given [Texture], or [None] if the ray did not hit the material.
    #[must_use]
    fn scatter(
        &self,
        _ray: &Ray,
        hit_record: &HitRecord,
        _rng: &mut SmallRng,
    ) -> Option<ScatterRecord> {
        let albedo = self
            .albedo
            .color(hit_record.u, hit_record.v, hit_record.position);

        Some(ScatterRecord {
            material_type: MaterialType::Diffuse,
            specular_ray: None,
            attenuation: albedo,
            pdf_ptr: PDF::SpherePDF(SpherePDF::new()),
        })
    }

    /// Returns the scattering probability density function for the [Isotropic] material
    #[allow(clippy::unused_self)]
    #[must_use]
    fn scattering_pdf(
        &self,
        _hit_record: &HitRecord,
        _scattered: &Ray,
        _rng: &mut SmallRng,
    ) -> Option<Float> {
        Some(1.0 / (4.0 * PI))
    }
}

impl Isotropic {
    /// Creates a new [Isotropic] material with an albedo of the given [Texture].
    #[must_use]
    pub fn new(emission: Texture) -> Self {
        Isotropic { albedo: emission }
    }
}
