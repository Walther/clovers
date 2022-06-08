//! A diffuse light material.

use super::ScatterRecord;
use crate::{
    color::Color,
    hitable::HitRecord,
    ray::Ray,
    textures::{SolidColor, Texture},
    Float, Vec3,
};
use rand::prelude::SmallRng;

/// A diffuse light material. On this material, rays never scatter - the material always emits a color based on its texture.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
pub struct DiffuseLight {
    emit: Texture,
}

impl Default for DiffuseLight {
    /// Creates a new [`DiffuseLight`] with white light at intensity `100.0`
    fn default() -> Self {
        DiffuseLight {
            emit: Texture::SolidColor(SolidColor::new(Color::new(100.0, 100.0, 100.0))),
        }
    }
}

impl<'a> DiffuseLight {
    /// Scatter method for the [`DiffuseLight`] material. Always returns `None`, as diffuse light does not scatter.
    #[allow(clippy::unused_self)]
    pub fn scatter(
        self,
        _ray: &Ray,
        _hit_record: &HitRecord,
        _rng: &mut SmallRng,
    ) -> Option<ScatterRecord<'a>> {
        None
    }

    /// Scattering probability density function for the [`DiffuseLight`] material. Always returns 0, as diffuse light does not scatter.
    #[allow(clippy::unused_self)] // TODO:
    pub fn scattering_pdf(
        self,
        _ray: &Ray,
        _hit_record: &HitRecord,
        _scattered: &Ray,
        _rng: &mut SmallRng,
    ) -> Option<Float> {
        None
    }

    /// Emission function for [`DiffuseLight`]. If the given [`HitRecord`] has been hit on the `front_face`, emit a color based on the texture and surface coordinates. Otherwise, emit pure black.
    #[must_use]
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

    /// Creates a new [`DiffuseLight`] material with the given [Texture].
    #[must_use]
    pub fn new(emission: Texture) -> Self {
        DiffuseLight { emit: emission }
    }
}
