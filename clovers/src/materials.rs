//! Materials enable different behaviors of light on objects.

use crate::{color::Color, hitable::HitRecord, pdf::PDF, ray::Ray, Float, Vec3};
pub mod dielectric;
pub mod diffuse_light;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

pub use dielectric::*;
pub use diffuse_light::*;
use enum_dispatch::enum_dispatch;
pub use isotropic::*;
pub use lambertian::*;
pub use metal::*;
use rand::prelude::SmallRng;

#[enum_dispatch]
pub(crate) trait MaterialTrait {
    fn scatter(
        self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut SmallRng,
    ) -> Option<ScatterRecord>;

    fn scattering_pdf(
        self,
        ray: &Ray,
        hit_record: &HitRecord,
        scattered: &Ray,
        rng: &mut SmallRng,
    ) -> Option<Float>;

    fn emit(
        &self,
        _ray: &Ray,
        _hit_record: &HitRecord,
        _u: Float,
        _v: Float,
        _position: Vec3,
    ) -> Color {
        // Most materials don't emit, override when needed
        Color::new(0.0, 0.0, 0.0)
    }
}

#[enum_dispatch(MaterialTrait)]
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// A material enum. TODO: for ideal clean abstraction, this should be a trait. However, that comes with some additional considerations, including e.g. performance.
#[cfg_attr(feature = "serde-derive", serde(tag = "kind"))]
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

impl Default for Material {
    fn default() -> Self {
        Self::Lambertian(Lambertian::default())
    }
}

#[derive(Debug)]
/// Enum for the types of materials: Diffuse and Specular (i.e., matte and shiny)
pub enum MaterialType {
    /// A matte material that does not reflect rays
    Diffuse,
    /// A shiny material that reflects some rays
    Specular,
}

#[derive(Debug)]
/// A record of an scattering event of a [Ray] on a [Material].
pub struct ScatterRecord {
    /// The material type that was scattered on
    pub material_type: MaterialType,
    /// Direction of a generated specular ray
    pub specular_ray: Option<Ray>,
    /// Current color to take into account when following the scattered ray for futher iterations
    pub attenuation: Color,
    /// Probability density function to use with the [ScatterRecord].
    // TODO: understand & explain
    pub pdf_ptr: PDF,
}

// TODO: are these up to date / correct?

#[must_use]
fn reflect(vector: Vec3, normal: Vec3) -> Vec3 {
    vector - 2.0 * vector.dot(&normal) * normal
}

#[must_use]
fn refract(uv: Vec3, normal: Vec3, etai_over_etat: Float) -> Vec3 {
    let cos_theta: Float = -uv.dot(&normal);
    let r_out_parallel: Vec3 = etai_over_etat * (uv + cos_theta * normal);
    let r_out_perp: Vec3 = -(1.0 - r_out_parallel.norm_squared()).sqrt() * normal;
    r_out_parallel + r_out_perp
}

#[must_use]
fn schlick(cosine: Float, refractive_index: Float) -> Float {
    let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * ((1.0 - cosine).powf(5.0))
}
