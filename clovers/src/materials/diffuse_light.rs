//! A diffuse light material.

#[cfg(not(target_arch = "spirv"))]
use super::ScatterRecord;
use super::{GPUScatterRecord, MaterialType};

#[cfg(not(target_arch = "spirv"))]
use crate::{
    hitrecord::HitRecord,
    textures::{SolidColor, Texture},
};

use crate::{
    color::Color, hitrecord::GPUHitRecord, ray::Ray, textures::GPUTexture, CloversRng, Float, Vec3,
};

/// A diffuse light material. On this material, rays never scatter - the material always emits a color based on its texture.
#[derive(Clone, Copy)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[cfg(not(target_arch = "spirv"))]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
pub struct DiffuseLight {
    emit: Texture,
}

#[cfg(not(target_arch = "spirv"))]
impl Default for DiffuseLight {
    /// Creates a new [DiffuseLight] with white light at intensity `100.0`
    fn default() -> Self {
        DiffuseLight {
            emit: Texture::SolidColor(SolidColor::new(Color::new(100.0, 100.0, 100.0))),
        }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl<'a> DiffuseLight {
    /// Scatter method for the [DiffuseLight] material. Always returns `None`, as diffuse light does not scatter.
    pub fn scatter(
        self,
        _ray: &Ray,
        _hit_record: &HitRecord,
        _rng: &mut CloversRng,
    ) -> Option<ScatterRecord<'a>> {
        None
    }

    /// Scattering probability density function for the [DiffuseLight] material. Always returns 0, as diffuse light does not scatter.
    pub fn scattering_pdf(
        self,
        _ray: &Ray,
        _hit_record: &HitRecord,
        _scattered: &Ray,
        _rng: &mut CloversRng,
    ) -> Float {
        0.0 // TODO: cleanup
    }

    /// Emission function for [DiffuseLight]. If the given [HitRecord] has been hit on the `front_face`, emit a color based on the texture and surface coordinates. Otherwise, emit pure black.
    pub fn emit(
        self,
        _ray: &Ray,
        hit_record: &HitRecord,
        u: Float,
        v: Float,
        position: Vec3,
    ) -> Color {
        if hit_record.front_face {
            self.emit.color(u, v, position)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    }

    /// Creates a new [DiffuseLight] material with the given [Texture].
    pub fn new(emission: Texture) -> Self {
        DiffuseLight { emit: emission }
    }
}

// TODO: clean up duplicated work due to GPU variants

/// GPU accelerated diffuse light material. On this material, rays never scatter - the material always emits a color based on its texture.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GPUDiffuseLight {
    /// The texture used for the emission
    pub emit: GPUTexture,
}

impl GPUDiffuseLight {
    /// Creates a new [GPUDiffuseLight] material with the given [GPUTexture].
    pub fn new(emission: GPUTexture) -> Self {
        GPUDiffuseLight { emit: emission }
    }

    /// Emission function for [GPUDiffuseLight]. If the given [HitRecord] has been hit on the `front_face`, emit a color based on the texture and surface coordinates. Otherwise, emit pure black.
    pub fn emit(
        self,
        _ray: &Ray,
        hit_record: &GPUHitRecord,
        u: Float,
        v: Float,
        position: Vec3,
    ) -> Color {
        if hit_record.front_face == 1 {
            self.emit.color(u, v, position)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    }

    /// Scattering probability density function for the [GPUDiffuseLight] material. Always returns 0, as diffuse light does not scatter.
    pub fn scattering_pdf(
        self,
        _ray: &Ray,
        _hit_record: &GPUHitRecord,
        _scattered: &Ray,
        _rng: &mut CloversRng,
    ) -> Float {
        0.0
    }

    /// Scatter method for the [DiffuseLight] material. Never scatters.
    pub fn scatter(
        self,
        _ray: &Ray,
        _hit_record: &GPUHitRecord,
        _rng: &mut CloversRng,
    ) -> GPUScatterRecord {
        GPUScatterRecord {
            material_type: MaterialType::Diffuse,
            specular_ray: Ray::default(), // TODO: should be ignored.
            attenuation: Color::new(1.0, 1.0, 1.0), // TODO: does this make sense here? Transparent color later?
        }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl From<DiffuseLight> for GPUDiffuseLight {
    fn from(d: DiffuseLight) -> Self {
        GPUDiffuseLight {
            emit: d.emit.into(),
        }
    }
}
