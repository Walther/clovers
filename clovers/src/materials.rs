//! Materials enable different behaviors of light on objects.

use core::fmt::Debug;

use crate::{color::Color, hitable::HitRecord, pdf::PDF, ray::Ray, Float, Vec3};
pub mod cone_light;
pub mod dielectric;
pub mod diffuse_light;
pub mod dispersive;
#[cfg(feature = "gl_tf")]
pub mod gltf;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

pub use cone_light::*;
pub use dielectric::*;
pub use diffuse_light::*;
pub use dispersive::*;
use enum_dispatch::enum_dispatch;
pub use isotropic::*;
pub use lambertian::*;
pub use metal::*;
use palette::LinSrgb;
use rand::prelude::SmallRng;

/// Initialization structure for a `Material`. Either contains a `Material` by itself, or a String `name` to be found in a shared material list.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde-derive", serde(untagged))]
pub enum MaterialInit {
    /// Name of the shared material
    Shared(String),
    /// Owned material structure
    Owned(Material),
}

impl Default for MaterialInit {
    fn default() -> Self {
        Self::Shared(String::new())
    }
}

/// A `Material` that can be referred to by name for reuse across multiple `Object`s
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
pub struct SharedMaterial {
    /// Name of the shared material
    pub name: String,
    /// The shared material itself
    #[serde(flatten)]
    pub material: Material,
}

#[enum_dispatch]
/// Trait for materials. Requires three function implementations: `scatter`, `scattering_pdf`, and `emit`.
pub trait MaterialTrait: Debug {
    /// Given a ray and a hitrecord, return the possible `ScatterRecord`.
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut SmallRng,
    ) -> Option<ScatterRecord>;

    /// TODO: explain
    fn scattering_pdf(
        &self,
        hit_record: &HitRecord,
        scattered: &Ray,
        rng: &mut SmallRng,
    ) -> Option<Float>;

    /// Returns the emissivity of the material at the given position. Defaults to black as most materials don't emit - override when needed.
    fn emit(
        &self,
        _ray: &Ray,
        _hit_record: &HitRecord,
        _u: Float,
        _v: Float,
        _position: Vec3,
    ) -> LinSrgb {
        LinSrgb::new(0.0, 0.0, 0.0)
    }
}

#[enum_dispatch(MaterialTrait)]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// A material enum. TODO: for ideal clean abstraction, this should be a trait. However, that comes with some additional considerations, including e.g. performance.
#[cfg_attr(feature = "serde-derive", serde(tag = "kind"))]
pub enum Material {
    /// Dielectric material
    Dielectric(Dielectric),
    /// Dispersive material
    Dispersive(Dispersive),
    /// Lambertian material
    Lambertian(Lambertian),
    /// ConeLight material
    ConeLight(ConeLight),
    /// DiffuseLight material
    DiffuseLight(DiffuseLight),
    /// Metal material
    Metal(Metal),
    /// Isotropic material
    Isotropic(Isotropic),
}

impl Default for Material {
    fn default() -> Self {
        Self::Lambertian(Lambertian::default())
    }
}

#[derive(Debug, Clone)]
/// Enum for the types of materials: Diffuse and Specular (i.e., matte and shiny)
pub enum MaterialType {
    /// A matte material that does not reflect rays
    Diffuse,
    /// A shiny material that reflects some rays
    Specular,
}

#[derive(Debug, Clone)]
/// A record of an scattering event of a [Ray] on a [Material].
pub struct ScatterRecord<'ray> {
    /// The material type that was scattered on
    pub material_type: MaterialType,
    /// Direction of a generated specular ray
    pub specular_ray: Option<Ray>,
    /// Current color to take into account when following the scattered ray for futher iterations
    pub attenuation: Color,
    /// Probability density function to use with the [ScatterRecord].
    // TODO: understand & explain
    pub pdf_ptr: PDF<'ray>,
}

// TODO: are these up to date / correct?

#[must_use]
fn reflect(vector: Vec3, normal: Vec3) -> Vec3 {
    vector - 2.0 * vector.dot(&normal) * normal
}

#[must_use]
fn refract(uv: Vec3, normal: Vec3, refraction_ratio: Float) -> Vec3 {
    let cos_theta: Float = -uv.dot(&normal);
    let cos_theta = cos_theta.min(1.0); // Clamp
    let r_out_parallel: Vec3 = refraction_ratio * (uv + cos_theta * normal);
    let r_out_perp: Vec3 = -(1.0 - r_out_parallel.norm_squared()).sqrt() * normal;
    r_out_parallel + r_out_perp
}

#[must_use]
fn schlick(cosine: Float, refractive_index: Float) -> Float {
    let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * ((1.0 - cosine).powf(5.0))
}
