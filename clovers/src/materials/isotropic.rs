//! Isotropic material.

use super::{MaterialType, ScatterRecord};
use crate::{
    color::Color,
    hitable::HitRecord,
    pdf::{CosinePDF, PDF},
    ray::Ray,
    textures::Texture,
    Float, PI,
};
use rand::prelude::SmallRng;

#[derive(Debug, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Isotropic material. Used in [ConstantMedium](crate::objects::constant_medium). TODO: understand this!
pub struct Isotropic {
    #[cfg_attr(feature = "serde", serde(default))]
    albedo: Texture,
}

impl<'a> Isotropic {
    /// Creates a new [Isotropic] material with an albedo of the given [Texture].
    pub fn new(emission: Texture) -> Self {
        Isotropic { albedo: emission }
    }

    /// Returns a [ScatterRecord] based on the [HitRecord] coordinates and the given [Texture], or [None] if the ray did not hit the material. TODO: verify implementation, copied from [Lambertian](crate::materials::Lambertian)
    pub fn scatter(
        self,
        _ray: &Ray,
        hit_record: &HitRecord,
        _rng: &mut SmallRng,
    ) -> Option<ScatterRecord<'a>> {
        // TODO: fix / verify correctness!
        // this is just copied from lambertian as an experiment
        let albedo: Color = self
            .albedo
            .color(hit_record.u, hit_record.v, hit_record.position);

        Some(ScatterRecord {
            material_type: MaterialType::Diffuse,
            specular_ray: None,
            attenuation: albedo,
            pdf_ptr: PDF::CosinePDF(CosinePDF::new(hit_record.normal)),
        })
    }

    /// Returns the scattering probability density function for the [Isotropic] material. TODO: verify implementation, copied from [Lambertian](crate::materials::Lambertian)
    pub fn scattering_pdf(
        self,
        _ray: &Ray,
        hit_record: &HitRecord,
        scattered: &Ray,
        _rng: &mut SmallRng,
    ) -> Float {
        // TODO: fix / verify correctness!
        // this is just copied from lambertian as an experiment
        let cosine = hit_record.normal.dot(&scattered.direction.normalize());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
}
