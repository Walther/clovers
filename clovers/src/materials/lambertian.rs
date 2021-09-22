//! Lambertian material. This is the default material with a smooth, matte surface.

#[cfg(not(target_arch = "spirv"))]
use super::ScatterRecord;

use super::{GPUScatterRecord, MaterialType};

#[cfg(not(target_arch = "spirv"))]
use crate::{
    hitrecord::HitRecord,
    pdf::{CosinePDF, PDF},
    textures::Texture,
};

use crate::{hitrecord::GPUHitRecord, ray::Ray, textures::GPUTexture, CloversRng, Float, PI};

#[derive(Clone, Copy, Default)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
#[cfg(not(target_arch = "spirv"))]
/// Lambertian material. This is the default material with a smooth, matte surface.
pub struct Lambertian {
    #[cfg_attr(feature = "serde-derive", serde(default))]
    albedo: Texture,
}

#[cfg(not(target_arch = "spirv"))]
impl<'a> Lambertian {
    /// Returns None, if ray is absorbed. Otherwise, returns a ray, albedo of what was hit, and (?) a value used for probability density function based sampling
    pub fn scatter(
        self,
        _ray: &Ray,
        hit_record: &HitRecord,
        _rng: &mut CloversRng,
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
        _rng: &mut CloversRng,
    ) -> Float {
        // TODO: explain the math
        let cosine = hit_record.normal.dot(scattered.direction.normalize());
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

/// GPU accelerated Lambertian material
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GPULambertian {
    /// Albedo of the Lambertian texture
    pub albedo: GPUTexture,
}

#[cfg(not(target_arch = "spirv"))]
impl From<Lambertian> for GPULambertian {
    fn from(d: Lambertian) -> Self {
        GPULambertian {
            albedo: d.albedo.into(),
        }
    }
}

impl GPULambertian {
    /// Returns the scattering probability density function for the [GPULambertian] material. TODO: explain the math
    pub fn scattering_pdf(
        self,
        _ray: &Ray,
        hit_record: &GPUHitRecord,
        scattered: &Ray,
        _rng: &mut CloversRng,
    ) -> Float {
        // TODO: explain the math
        let cosine = hit_record.normal.dot(scattered.direction.normalize());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }

    /// GPU compatible scatter.
    // TODO: docs
    pub fn scatter(
        self,
        _ray: &Ray,
        hit_record: &GPUHitRecord,
        _rng: &mut CloversRng,
    ) -> GPUScatterRecord {
        GPUScatterRecord {
            material_type: MaterialType::Diffuse,
            specular_ray: Ray::default(), // Does not matter; specular is not used on diffuse materials
            attenuation: self
                .albedo
                .color(hit_record.u, hit_record.v, hit_record.position),
        }
    }
}
