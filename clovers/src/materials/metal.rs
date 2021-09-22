//! A metal material.

#[cfg(not(target_arch = "spirv"))]
use super::{reflect, MaterialType, ScatterRecord};

#[cfg(not(target_arch = "spirv"))]
use crate::{
    hitrecord::HitRecord,
    pdf::{ZeroPDF, PDF},
    random::random_in_unit_sphere,
    ray::Ray,
    textures::Texture,
    CloversRng, Float, Vec3,
};

use crate::textures::GPUTexture;

#[derive(Clone, Copy, Default)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
#[cfg(not(target_arch = "spirv"))]
/// A metal material. The amount of reflection can be adjusted with the `fuzz` parameter.
pub struct Metal {
    #[cfg_attr(feature = "serde-derive", serde(default))]
    albedo: Texture,
    #[cfg_attr(feature = "serde-derive", serde(default))]
    fuzz: Float,
}

#[cfg(not(target_arch = "spirv"))]
impl<'a> Metal {
    /// Scatter function for the [Metal] material. Metal always reflects, and a specular ray is calculated with some randomness adjusted by the `fuzz` factor. This means the metal can be made more shiny or more matte. The returned [ScatterRecord] will have a probability density function of [ZeroPDF] and material type of [MaterialType::Specular]
    pub fn scatter(
        self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut CloversRng,
    ) -> Option<ScatterRecord<'a>> {
        let reflected: Vec3 = reflect(ray.direction.normalize(), hit_record.normal);
        Some(ScatterRecord {
            specular_ray: Some(Ray::new(
                hit_record.position,
                reflected + self.fuzz * random_in_unit_sphere(rng),
                ray.time,
            )),
            // TODO: fix coordinates. Currently metal only works for [SolidColor]
            attenuation: self.albedo.color(0.0, 0.0, hit_record.position),
            material_type: MaterialType::Specular,
            pdf_ptr: PDF::ZeroPDF(ZeroPDF::new()),
        })
    }

    /// Scattering probability density function for [Metal]. Always returns zero. TODO: why?
    pub fn scattering_pdf(
        self,
        _ray: &Ray,
        _hit_record: &HitRecord,
        _scattered: &Ray,
        _rng: &mut CloversRng,
    ) -> Float {
        0.0 // TODO: why does metal scatter 0? No mention in tutorial afaiu
    }

    /// Creates a new [Metal] material with the albedo of the given [Texture] and a smoothness-roughness factor specified by `fuzz` parameter.
    pub fn new(albedo: Texture, fuzz: Float) -> Self {
        Metal {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

/// GPU accelerated Metal material
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GPUMetal {
    albedo: GPUTexture,
}

#[cfg(not(target_arch = "spirv"))]
impl From<Metal> for GPUMetal {
    fn from(d: Metal) -> Self {
        GPUMetal {
            albedo: d.albedo.into(),
        }
    }
}
