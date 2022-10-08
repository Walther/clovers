//! A metal material.

use super::{reflect, MaterialTrait, MaterialType, ScatterRecord};
use crate::{
    hitable::HitRecord,
    pdf::{ZeroPDF, PDF},
    random::random_in_unit_sphere,
    ray::Ray,
    textures::Texture,
    Float, Vec3,
};
use rand::prelude::SmallRng;

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// A metal material. The amount of reflection can be adjusted with the `fuzz` parameter.
pub struct Metal {
    #[cfg_attr(feature = "serde-derive", serde(default))]
    albedo: Texture,
    #[cfg_attr(feature = "serde-derive", serde(default))]
    fuzz: Float,
}

impl MaterialTrait for Metal {
    /// Scatter function for the [Metal] material. Metal always reflects, and a specular ray is calculated with some randomness adjusted by the `fuzz` factor. This means the metal can be made more shiny or more matte. The returned [`ScatterRecord`] will have a probability density function of [`ZeroPDF`] and material type of [`MaterialType::Specular`]
    #[must_use]
    fn scatter(
        self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut SmallRng,
    ) -> Option<ScatterRecord> {
        let reflected: Vec3 = reflect(ray.direction.normalize(), hit_record.normal);
        Some(ScatterRecord {
            specular_ray: Some(Ray::new(
                hit_record.position,
                reflected + self.fuzz * random_in_unit_sphere(rng),
                ray.time,
            )),
            attenuation: self
                .albedo
                .color(hit_record.u, hit_record.v, hit_record.position),
            material_type: MaterialType::Specular,
            pdf_ptr: PDF::ZeroPDF(ZeroPDF::new()),
        })
    }

    /// Scattering probability density function for [Metal]. Always returns zero. TODO: why?
    #[allow(clippy::unused_self)] // TODO:
    #[must_use]
    fn scattering_pdf(
        self,
        _ray: &Ray,
        _hit_record: &HitRecord,
        _scattered: &Ray,
        _rng: &mut SmallRng,
    ) -> Option<Float> {
        None // TODO: why does metal never scatter? should it scatter if fuzzy?
    }
}

impl Metal {
    /// Creates a new [Metal] material with the albedo of the given [Texture] and a smoothness-roughness factor specified by `fuzz` parameter.
    #[must_use]
    pub fn new(albedo: Texture, fuzz: Float) -> Self {
        Metal {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}
