//! Materials enable different behaviors of light on objects.

use alloc::string::String;
use core::fmt::Debug;
use nalgebra::Unit;

use crate::{pdf::PDF, ray::Ray, wavelength::Wavelength, Direction, Float, HitRecord, Vec3};
pub mod cone_light;
pub mod dielectric;
pub mod diffuse_light;
pub mod dispersive;
#[cfg(feature = "gl_tf")]
pub mod gltf;
pub mod isotropic;
pub mod lambertian;
pub mod metal;
pub mod thin_film;

pub use cone_light::*;
pub use dielectric::*;
pub use diffuse_light::*;
pub use dispersive::*;
use enum_dispatch::enum_dispatch;
pub use isotropic::*;
pub use lambertian::*;
pub use metal::*;
use palette::{white_point::E, Xyz};
use rand::prelude::SmallRng;
pub use thin_film::*;

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
    #[cfg_attr(feature = "serde-derive", serde(flatten))]
    pub material: Material,
}

/// The main material struct for the renderer.
///
/// This is a wrapper type. It contains the common properties shared by all materials, and an `inner` field with properties and method implementations specific to each material [`Kind`].
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Default)]
pub struct Material {
    /// The inner material with properties and method implementations specific to each material [`Kind`].
    #[cfg_attr(feature = "serde-derive", serde(flatten))]
    pub kind: Kind,
    /// Optional thin film interference layer on top of the material
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub thin_film: Option<ThinFilm>,
}

impl MaterialTrait for Material {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut SmallRng,
    ) -> Option<ScatterRecord> {
        let mut scatter_record = self.kind.scatter(ray, hit_record, rng)?;
        if let Some(f) = &self.thin_film {
            scatter_record.attenuation *= f.interference(ray, hit_record);
        };
        Some(scatter_record)
    }

    fn scattering_pdf(&self, hit_record: &HitRecord, scattered: &Ray) -> Option<Float> {
        self.kind.scattering_pdf(hit_record, scattered)
    }

    fn emit(&self, ray: &Ray, wavelength: Wavelength, hit_record: &HitRecord) -> Float {
        self.kind.emit(ray, wavelength, hit_record)
    }

    fn is_wavelength_dependent(&self) -> bool {
        self.thin_film.is_some() || self.kind.is_wavelength_dependent()
    }
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
    fn scattering_pdf(&self, _hit_record: &HitRecord, _scattered: &Ray) -> Option<Float> {
        None
    }

    /// Returns the spectral power of the material sampled at the given wavelength. Defaults to zero, override for emissive materials.
    fn emit(&self, _ray: &Ray, _wavelength: Wavelength, _hit_record: &HitRecord) -> Float {
        0.0
    }

    /// Returns true if the material has wavelength-dependent scattering, like dispersion or iridescence.
    fn is_wavelength_dependent(&self) -> bool {
        false
    }
}

#[enum_dispatch(MaterialTrait)]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde-derive", serde(tag = "kind"))]
/// An enum for the material kind
pub enum Kind {
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

impl Default for Kind {
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
    pub attenuation: Xyz<E>,
    /// Probability density function to use with the [`ScatterRecord`].
    // TODO: understand & explain
    pub pdf_ptr: PDF<'ray>,
}

// TODO: are these up to date / correct?

#[must_use]
fn reflect(vector: Direction, normal: Direction) -> Direction {
    let v: Vec3 = *vector - 2.0 * vector.dot(&normal) * *normal;
    Unit::new_normalize(v)
}

#[must_use]
fn refract(vector: Direction, normal: Direction, refraction_ratio: Float) -> Direction {
    let cos_theta: Float = -vector.dot(&normal);
    let cos_theta = cos_theta.min(1.0); // Clamp
    let r_out_parallel: Vec3 = refraction_ratio * (*vector + cos_theta * *normal);
    let r_out_perp: Vec3 = -(1.0 - r_out_parallel.norm_squared()).sqrt() * *normal;
    Unit::new_normalize(r_out_parallel + r_out_perp)
}

#[must_use]
fn schlick(cosine: Float, refractive_index: Float) -> Float {
    let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * ((1.0 - cosine).powf(5.0))
}
