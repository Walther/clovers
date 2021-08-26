//! Lambertian material. This is the default material with a smooth, matte surface.

use super::{MaterialType, ScatterRecord};
use crate::{
    hitable::HitRecord,
    pdf::{CosinePDF, PDF},
    ray::Ray,
    textures::Texture,
    Float, PI,
};
use rand::rngs::SmallRng;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Deserialize, Serialize, Debug, Default)]
/// Lambertian material. This is the default material with a smooth, matte surface.
pub struct Lambertian {
    #[serde(default)]
    albedo: Texture,
}

impl<'a> Lambertian {
    /// Returns None, if ray is absorbed. Otherwise, returns a ray, albedo of what was hit, and (?) a value used for probability density function based sampling
    pub fn scatter(
        self,
        _ray: &Ray,
        hit_record: &HitRecord,
        _rng: SmallRng,
    ) -> Option<ScatterRecord<'a>> {
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
    pub fn scattering_pdf(
        self,
        _ray: &Ray,
        hit_record: &HitRecord,
        scattered: &Ray,
        _rng: SmallRng,
    ) -> Float {
        // TODO: explain the math
        let cosine = hit_record.normal.dot(&scattered.direction.normalize());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }

    /// Creates a new instance of the [Lambertian] material with an albedo of the given [Texture].
    pub fn new(albedo: impl Into<Texture>) -> Self {
        Lambertian {
            albedo: albedo.into(),
        }
    }
}
