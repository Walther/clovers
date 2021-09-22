//! Isotropic material.

#[cfg(not(target_arch = "spirv"))]
use super::ScatterRecord;
use super::{GPUScatterRecord, MaterialType};

#[cfg(not(target_arch = "spirv"))]
use crate::{
    hitrecord::HitRecord,
    pdf::{CosinePDF, PDF},
    textures::Texture,
};

use crate::{
    color::Color, hitrecord::GPUHitRecord, ray::Ray, textures::GPUTexture, CloversRng, Float, PI,
};

#[derive(Clone, Copy, Default)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[cfg(not(target_arch = "spirv"))]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// Isotropic material. Used in [ConstantMedium](crate::objects::constant_medium). TODO: understand this!
pub struct Isotropic {
    #[cfg_attr(feature = "serde-derive", serde(default))]
    albedo: Texture,
}

#[cfg(not(target_arch = "spirv"))]
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
        _rng: &mut CloversRng,
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
        _rng: &mut CloversRng,
    ) -> Float {
        // TODO: fix / verify correctness!
        // this is just copied from lambertian as an experiment
        let cosine = hit_record.normal.dot(scattered.direction.normalize());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
}

/// GPU accelerated Isotropic material
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GPUIsotropic {
    /// Albedo of the Isotropic texture. TODO: could probably just be a color? Or does e.g. SpatialChecker make sense here?
    pub albedo: GPUTexture,
}

#[cfg(not(target_arch = "spirv"))]
impl From<Isotropic> for GPUIsotropic {
    fn from(d: Isotropic) -> Self {
        GPUIsotropic {
            albedo: d.albedo.into(),
        }
    }
}

impl GPUIsotropic {
    /// Returns the scattering probability density function for the [GPUIsotropic] material. TODO: verify implementation, copied from [Lambertian](crate::materials::Lambertian)
    pub fn scattering_pdf(
        self,
        _ray: &Ray,
        hit_record: &GPUHitRecord,
        scattered: &Ray,
        _rng: &mut CloversRng,
    ) -> Float {
        // TODO: fix / verify correctness!
        // this is just copied from lambertian as an experiment
        let cosine = hit_record.normal.dot(scattered.direction.normalize());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }

    /// Returns a [ScatterRecord] based on the [HitRecord] coordinates and the given [Texture], or [None] if the ray did not hit the material. TODO: verify implementation, copied from [Lambertian](crate::materials::Lambertian)
    pub fn scatter(
        self,
        _ray: &Ray,
        hit_record: &GPUHitRecord,
        _rng: &mut CloversRng,
    ) -> GPUScatterRecord {
        // TODO: fix / verify correctness!
        // this is just copied from lambertian as an experiment
        let albedo: Color = self
            .albedo
            .color(hit_record.u, hit_record.v, hit_record.position);

        GPUScatterRecord {
            material_type: MaterialType::Diffuse,
            specular_ray: Ray::default(), // should be ignored
            attenuation: albedo,
        }
    }
}
