//! A diffuse light material.

use super::{MaterialTrait, ScatterRecord};
use crate::{
    hitable::HitRecord,
    ray::Ray,
    textures::{SolidColor, Texture, TextureTrait},
    Float, Vec3,
};
use palette::{convert::IntoColorUnclamped, LinSrgb, Srgb};
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
            emit: Texture::SolidColor(SolidColor::new(Srgb::new(100.0, 100.0, 100.0))),
        }
    }
}

impl MaterialTrait for DiffuseLight {
    /// Scatter method for the [`DiffuseLight`] material. Always returns `None`, as diffuse light does not scatter.
    #[allow(clippy::unused_self)]
    #[must_use]
    fn scatter(
        &self,
        _ray: &Ray,
        _hit_record: &HitRecord,
        _rng: &mut SmallRng,
    ) -> Option<ScatterRecord> {
        None
    }

    /// Scattering probability density function for the [`DiffuseLight`] material. Always returns 0, as diffuse light does not scatter.
    #[allow(clippy::unused_self)] // TODO:
    #[must_use]
    fn scattering_pdf(
        &self,
        _hit_record: &HitRecord,
        _scattered: &Ray,
        _rng: &mut SmallRng,
    ) -> Option<Float> {
        None
    }

    /// Emission function for [`DiffuseLight`]. If the given [`HitRecord`] has been hit on the `front_face`, emit a color based on the texture and surface coordinates. Otherwise, emit pure black.
    #[must_use]
    fn emit(
        &self,
        _ray: &Ray,
        hit_record: &HitRecord,
        u: Float,
        v: Float,
        position: Vec3,
    ) -> LinSrgb {
        if hit_record.front_face {
            let emit = self.emit.color(u, v, position);
            let emit: LinSrgb = emit.into_color_unclamped();
            emit
        } else {
            LinSrgb::new(0.0, 0.0, 0.0)
        }
    }
}

impl DiffuseLight {
    /// Creates a new [`DiffuseLight`] material with the given [Texture].
    #[must_use]
    pub fn new(emission: Texture) -> Self {
        DiffuseLight { emit: emission }
    }
}
