//! Materials enable different behaviors of light on objects.

#[cfg(not(target_arch = "spirv"))]
use crate::{hitrecord::HitRecord, pdf::PDF};

#[cfg(target_arch = "spirv")]
use crate::FloatTrait;

use crate::{
    color::Color, hitrecord::GPUHitRecord, ray::Ray, textures::GPUTexture, CloversRng, Float, Vec3,
};

pub mod dielectric;
pub mod diffuse_light;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

pub use dielectric::*;
pub use diffuse_light::*;
pub use isotropic::*;
pub use lambertian::*;
pub use metal::*;

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
#[cfg(not(target_arch = "spirv"))]
/// A material enum. TODO: for ideal clean abstraction, this should be a trait. However, that comes with some additional considerations, including e.g. performance.
pub enum Material {
    /// Dielectric material
    Dielectric(Dielectric),
    /// Lambertian material
    Lambertian(Lambertian),
    /// DiffuseLight material
    DiffuseLight(DiffuseLight),
    /// Metal material
    Metal(Metal),
    /// Isotropic material
    Isotropic(Isotropic),
}

#[cfg(not(target_arch = "spirv"))]
impl Default for Material {
    fn default() -> Self {
        Self::Lambertian(Lambertian::default())
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Material {
    /// Scatters a ray from the material
    pub fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut CloversRng,
    ) -> Option<ScatterRecord> {
        match *self {
            Material::Dielectric(d) => Dielectric::scatter(d, ray, hit_record, rng),
            Material::DiffuseLight(d) => DiffuseLight::scatter(d, ray, hit_record, rng),
            Material::Isotropic(i) => Isotropic::scatter(i, ray, hit_record, rng),
            Material::Lambertian(l) => Lambertian::scatter(l, ray, hit_record, rng),
            Material::Metal(m) => Metal::scatter(m, ray, hit_record, rng),
        }
    }

    /// Returns a probability? TODO: understand
    pub fn scattering_pdf(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        scattered: &Ray,
        rng: &mut CloversRng,
    ) -> Float {
        match *self {
            Material::Dielectric(m) => m.scattering_pdf(ray, hit_record, scattered, rng),
            Material::DiffuseLight(m) => m.scattering_pdf(ray, hit_record, scattered, rng),
            Material::Isotropic(m) => m.scattering_pdf(ray, hit_record, scattered, rng),
            Material::Lambertian(m) => m.scattering_pdf(ray, hit_record, scattered, rng),
            Material::Metal(m) => m.scattering_pdf(ray, hit_record, scattered, rng),
        }
    }

    /// Returns the amount of light the material emits. By default, materials do not emit light, returning black.
    pub fn emit(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        u: Float,
        v: Float,
        position: Vec3,
    ) -> Color {
        match *self {
            Material::DiffuseLight(d) => d.emit(ray, hit_record, u, v, position),
            _ => Color::new(0.0, 0.0, 0.0),
        }
    }
}

#[derive(Clone, Copy)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
/// Enum for the types of materials: Diffuse and Specular (i.e., matte and shiny)
pub enum MaterialType {
    /// A matte material that does not reflect rays
    Diffuse,
    /// A shiny material that reflects some rays
    Specular,
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[cfg(not(target_arch = "spirv"))]
/// A record of an scattering event of a [Ray] on a [Material].
pub struct ScatterRecord<'a> {
    /// The material type that was scattered on
    pub material_type: MaterialType,
    /// Direction of a generated specular ray
    pub specular_ray: Option<Ray>,
    /// Current color to take into account when following the scattered ray for futher iterations
    pub attenuation: Color,
    /// Probability density function to use with the [ScatterRecord].
    // TODO: understand & explain
    pub pdf_ptr: PDF<'a>,
}

/// GPU compatible ScatterRecord. Vastly simplified and work in progress
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GPUScatterRecord {
    /// The material type that was scattered on
    pub material_type: MaterialType,
    /// Direction of a generated specular ray
    pub specular_ray: Ray,
    /// Current color to take into account when following the scattered ray for futher iterations
    pub attenuation: Color,
}

fn reflect(vector: Vec3, normal: Vec3) -> Vec3 {
    vector - 2.0 * vector.dot(normal) * normal
}

fn refract(uv: Vec3, normal: Vec3, etai_over_etat: Float) -> Vec3 {
    let cos_theta: Float = -uv.dot(normal);
    let r_out_parallel: Vec3 = etai_over_etat * (uv + cos_theta * normal);
    let r_out_perp: Vec3 = -(1.0 - r_out_parallel.length_squared()).sqrt() * normal;

    r_out_parallel + r_out_perp
}

fn schlick(cosine: Float, refractive_index: Float) -> Float {
    let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * ((1.0 - cosine).powf(5.0))
}

/// A GPU compatible material enum.
#[derive(Copy, Clone)]
#[repr(C)]
pub enum GPUMaterialKind {
    /// Dielectric material
    Dielectric,
    /// Lambertian material
    Lambertian,
    /// DiffuseLight material
    DiffuseLight,
    /// Metal material
    Metal,
    /// Isotropic material
    Isotropic,
}

/// A GPU compatible material struct
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GPUMaterial {
    /// The type of the material
    pub kind: GPUMaterialKind,
    /// The emissive texture of the material, used for lights
    pub emit: GPUTexture,
    /// Refractive index of the material, used for glass
    pub refractive_index: Float,
    /// Color of the material, used for glass
    pub color: Color,
    /// Albedo of the material
    pub albedo: GPUTexture,
    /// Fuzziness of the material, used for metal
    pub fuzz: Float,
}

impl GPUMaterial {
    /// Returns the amount of light the material emits. By default, materials do not emit light, returning black.
    pub fn emit(
        &self,
        ray: &Ray,
        hit_record: &GPUHitRecord,
        u: Float,
        v: Float,
        position: Vec3,
    ) -> Color {
        match self.kind {
            GPUMaterialKind::DiffuseLight => GPUDiffuseLight::emit(
                GPUDiffuseLight { emit: self.emit },
                ray,
                hit_record,
                u,
                v,
                position,
            ),
            _ => Color::new(0.0, 0.0, 0.0),
        }
    }

    /// Scatters a ray from the material
    pub fn scatter(
        &self,
        ray: &Ray,
        hit_record: &GPUHitRecord,
        rng: &mut CloversRng,
    ) -> GPUScatterRecord {
        match self.kind {
            GPUMaterialKind::Dielectric => GPUDielectric::scatter(
                GPUDielectric {
                    refractive_index: self.refractive_index,
                    color: self.color,
                },
                ray,
                hit_record,
                rng,
            ),
            GPUMaterialKind::DiffuseLight => {
                GPUDiffuseLight::scatter(GPUDiffuseLight { emit: self.emit }, ray, hit_record, rng)
            }
            GPUMaterialKind::Isotropic => GPUIsotropic::scatter(
                GPUIsotropic {
                    albedo: self.albedo,
                },
                ray,
                hit_record,
                rng,
            ),
            GPUMaterialKind::Lambertian => GPULambertian::scatter(
                GPULambertian {
                    albedo: self.albedo,
                },
                ray,
                hit_record,
                rng,
            ),
            GPUMaterialKind::Metal => GPUMetal::scatter(
                GPUMetal {
                    albedo: self.albedo,
                    fuzz: self.fuzz,
                },
                ray,
                hit_record,
                rng,
            ),
        }
    }

    /// Returns a probability? TODO: understand
    pub fn scattering_pdf(
        &self,
        ray: &Ray,
        hit_record: &GPUHitRecord,
        scattered: &Ray,
        rng: &mut CloversRng,
    ) -> Float {
        match self.kind {
            GPUMaterialKind::Dielectric => GPUDielectric::scattering_pdf(
                GPUDielectric {
                    refractive_index: self.refractive_index,
                    color: self.color,
                },
                ray,
                hit_record,
                scattered,
                rng,
            ),
            GPUMaterialKind::DiffuseLight => GPUDiffuseLight::scattering_pdf(
                GPUDiffuseLight { emit: self.emit },
                ray,
                hit_record,
                scattered,
                rng,
            ),
            GPUMaterialKind::Isotropic => GPUIsotropic::scattering_pdf(
                GPUIsotropic {
                    albedo: self.albedo,
                },
                ray,
                hit_record,
                scattered,
                rng,
            ),
            GPUMaterialKind::Lambertian => GPULambertian::scattering_pdf(
                GPULambertian {
                    albedo: self.albedo,
                },
                ray,
                hit_record,
                scattered,
                rng,
            ),
            GPUMaterialKind::Metal => GPUMetal::scattering_pdf(
                GPUMetal {
                    albedo: self.albedo,
                    fuzz: self.fuzz,
                },
                ray,
                hit_record,
                scattered,
                rng,
            ),
        }
    }
}
