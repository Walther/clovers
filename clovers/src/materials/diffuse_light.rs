//! A diffuse light material.

use super::{MaterialTrait, ScatterRecord};
use crate::{
    ray::Ray,
    textures::{SolidColor, Texture, TextureTrait},
    wavelength::Wavelength,
    Float, HitRecord,
};
use palette::Xyz;
use rand::prelude::SmallRng;

/// A diffuse light material. On this material, rays never scatter - the material always emits a color based on its texture.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
pub struct DiffuseLight {
    emit: Texture,
}

impl Default for DiffuseLight {
    /// Creates a new [`DiffuseLight`] with white light at intensity `100.0`
    fn default() -> Self {
        DiffuseLight {
            emit: Texture::SolidColor(SolidColor::new(Xyz::new(100.0, 100.0, 100.0))),
        }
    }
}

impl MaterialTrait for DiffuseLight {
    /// Scatter method for the [`DiffuseLight`] material. Always returns `None`, as diffuse light does not scatter.
    fn scatter(
        &self,
        _ray: &Ray,
        _hit_record: &HitRecord,
        _rng: &mut SmallRng,
    ) -> Option<ScatterRecord<'_>> {
        None
    }

    /// Emission function for [`DiffuseLight`]. If the given [`HitRecord`] has been hit on the `front_face`, emit a color based on the texture and surface coordinates. Otherwise, emit pure black.
    fn emit(&self, ray: &Ray, wavelength: Wavelength, hit_record: &HitRecord) -> Float {
        if !hit_record.front_face {
            return 0.0;
        }

        // HACK: current scene file format allows non-illuminants as light sources. These textures have no `emit()`, use `color()` instead.
        match self.emit {
            Texture::SolidColor(_) | Texture::SpatialChecker(_) | Texture::SurfaceChecker(_) => {
                self.emit.color(ray, wavelength, hit_record)
            }
            _ => self.emit.emit(ray, wavelength, hit_record),
        }
    }

    fn color(&self, ray: &Ray, wavelength: Wavelength, hit_record: &HitRecord) -> Float {
        self.emit.color(ray, wavelength, hit_record).clamp(0.0, 1.0)
    }
}

impl DiffuseLight {
    /// Creates a new [`DiffuseLight`] material with the given [Texture].
    #[must_use]
    pub fn new(emission: impl Into<Texture>) -> Self {
        DiffuseLight {
            emit: emission.into(),
        }
    }
}
