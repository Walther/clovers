//! Lambertian material. This is the default material with a smooth, matte surface.

use super::{MaterialTrait, MaterialType, ScatterRecord};
use crate::{
    Float, HitRecord, PI,
    pdf::{CosinePDF, PDF},
    ray::Ray,
    textures::{Texture, TextureTrait},
    wavelength::Wavelength,
};
use rand::prelude::SmallRng;

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// Lambertian material. This is the default material with a smooth, matte surface.
pub struct Lambertian {
    #[cfg_attr(feature = "serde-derive", serde(default))]
    albedo: Texture,
}

impl MaterialTrait for Lambertian {
    /// Returns None, if ray is absorbed. Otherwise, returns a ray, albedo of what was hit, and (?) a value used for probability density function based sampling
    fn scatter(
        &self,
        _ray: &Ray,
        hit_record: &HitRecord,
        _rng: &mut SmallRng,
    ) -> Option<ScatterRecord> {
        Some(ScatterRecord {
            material_type: MaterialType::Diffuse,
            specular_ray: None,
            pdf_ptr: PDF::CosinePDF(CosinePDF::new(hit_record.normal)),
        })
    }

    /// Returns the scattering probability density function for the [Lambertian] material.
    ///
    /// Given the `HitRecord` normal and a scattered `Ray`, computes the dot product and normalizes by `1/pi`. If the dot product is negative, returns `None`.
    fn scattering_pdf(&self, hit_record: &HitRecord, scattered: &Ray) -> Option<Float> {
        let cosine = hit_record.normal.dot(&scattered.direction.normalize());
        if cosine < 0.0 {
            None
        } else {
            Some(cosine / PI)
        }
    }

    fn color(&self, ray: &Ray, wavelength: Wavelength, hit_record: &HitRecord) -> Float {
        self.albedo.color(ray, wavelength, hit_record)
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
