//! Isotropic material.

use super::{MaterialTrait, MaterialType, ScatterRecord};
use crate::{
    pdf::{SpherePDF, PDF},
    ray::Ray,
    textures::{Texture, TextureTrait},
    wavelength::Wavelength,
    Float, HitRecord, PI,
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
    fn scatter(
        &self,
        _ray: &Ray,
        _hit_record: &HitRecord,
        _rng: &mut SmallRng,
    ) -> Option<ScatterRecord<'_>> {
        Some(ScatterRecord {
            material_type: MaterialType::Diffuse,
            specular_ray: None,
            pdf_ptr: PDF::SpherePDF(SpherePDF::new()),
        })
    }

    /// Returns the scattering probability density function for the [Isotropic] material
    fn scattering_pdf(&self, _hit_record: &HitRecord, _scattered: &Ray) -> Option<Float> {
        Some(1.0 / (4.0 * PI))
    }

    fn color(&self, ray: &Ray, wavelength: Wavelength, hit_record: &HitRecord) -> Float {
        self.albedo.color(ray, wavelength, hit_record)
    }
}

impl Isotropic {
    /// Creates a new [Isotropic] material with an albedo of the given [Texture].
    #[must_use]
    pub fn new(emission: impl Into<Texture>) -> Self {
        Isotropic {
            albedo: emission.into(),
        }
    }
}
