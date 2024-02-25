//! Lambertian material. This is the default material with a smooth, matte surface.

use super::{MaterialTrait, MaterialType, ScatterRecord};
use crate::{
    hitable::HitRecord,
    pdf::{CosinePDF, PDF},
    ray::Ray,
    textures::{Texture, TextureInit, TextureTrait},
    Float, PI,
};
use rand::prelude::SmallRng;

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// Initialization structure for the [Lambertian] material.
pub struct LambertianInit {
    #[cfg_attr(feature = "serde-derive", serde(default))]
    /// Texture initializer for the material.
    pub albedo: TextureInit,
}

impl From<LambertianInit> for Lambertian {
    fn from(value: LambertianInit) -> Self {
        Lambertian::new(value.albedo)
    }
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde-derive", serde(from = "LambertianInit"))]
/// Lambertian material. This is the default material with a smooth, matte surface.
pub struct Lambertian {
    #[cfg_attr(feature = "serde-derive", serde(default))]
    albedo: Texture,
}

impl MaterialTrait for Lambertian {
    /// Returns None, if ray is absorbed. Otherwise, returns a ray, albedo of what was hit, and (?) a value used for probability density function based sampling
    #[must_use]
    fn scatter(
        &self,
        _ray: &Ray,
        hit_record: &HitRecord,
        _rng: &mut SmallRng,
    ) -> Option<ScatterRecord> {
        Some(ScatterRecord {
            material_type: MaterialType::Diffuse,
            specular_ray: None,
            attenuation: self
                .albedo
                .color(hit_record.u, hit_record.v, hit_record.position),
            pdf_ptr: PDF::CosinePDF(CosinePDF::new(hit_record.normal)),
        })
    }

    /// Returns the scattering probability density function for the [Lambertian] material. TODO: explain the math
    #[must_use]
    fn scattering_pdf(
        &self,
        hit_record: &HitRecord,
        scattered: &Ray,
        _rng: &mut SmallRng,
    ) -> Option<Float> {
        // TODO: explain the math
        let cosine = hit_record.normal.dot(&scattered.direction.normalize());
        if cosine < 0.0 {
            None
        } else {
            Some(cosine / PI)
        }
    }
}

impl Lambertian {
    /// Creates a new instance of the [Lambertian] material with an albedo of the given [Texture].
    #[must_use]
    pub fn new(albedo: impl Into<Texture>) -> Self {
        Lambertian {
            albedo: albedo.into(),
        }
    }
}
