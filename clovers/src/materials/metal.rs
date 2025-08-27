//! A metal material.

use super::{reflect, MaterialTrait, MaterialType, ScatterRecord};
use crate::{
    pdf::{ZeroPDF, PDF},
    random::random_unit_vector,
    ray::Ray,
    textures::{Texture, TextureTrait},
    wavelength::Wavelength,
    Direction, Float, HitRecord,
};
use nalgebra::Unit;
use rand::prelude::SmallRng;

#[derive(Debug, Clone)]
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
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut SmallRng,
    ) -> Option<ScatterRecord<'_>> {
        let reflected: Direction = reflect(ray.direction, hit_record.normal);
        let direction = *reflected + self.fuzz * *random_unit_vector(rng);
        let direction = Unit::new_normalize(direction);
        Some(ScatterRecord {
            specular_ray: Some(Ray {
                origin: hit_record.position,
                direction,
                time: ray.time,
                wavelength: ray.wavelength,
            }),
            material_type: MaterialType::Specular,
            pdf_ptr: PDF::ZeroPDF(ZeroPDF::new()),
        })
    }

    // TODO: should this material provide a `scattering_pdf` function?
    fn color(&self, ray: &Ray, wavelength: Wavelength, hit_record: &HitRecord) -> Float {
        self.albedo.color(ray, wavelength, hit_record)
    }
}

impl Metal {
    /// Creates a new [Metal] material with the albedo of the given [Texture] and a smoothness-roughness factor specified by `fuzz` parameter.
    #[must_use]
    pub fn new(albedo: impl Into<Texture>, fuzz: Float) -> Self {
        Metal {
            albedo: albedo.into(),
            fuzz: fuzz.min(1.0),
        }
    }
}
