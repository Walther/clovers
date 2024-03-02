//! A cone light material.

use super::{MaterialTrait, ScatterRecord};
use crate::{
    hitable::HitRecord,
    ray::Ray,
    textures::{SolidColor, Texture, TextureTrait},
    Float, Position,
};
use palette::{white_point::E, Xyz};
use rand::prelude::SmallRng;

/// A cone light material. The material emits light if the incoming ray is within a certain amount of degrees from the surface normal.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
pub struct ConeLight {
    spread: Float,
    emit: Texture,
}

impl Default for ConeLight {
    /// Creates a new [`ConeLight`] with white light at intensity `100.0` and a spread of 10 degrees.
    fn default() -> Self {
        ConeLight {
            spread: 10.0,
            emit: Texture::SolidColor(SolidColor::new(Xyz::new(100.0, 100.0, 100.0))),
        }
    }
}

impl MaterialTrait for ConeLight {
    /// Scatter method for the [`ConeLight`] material. Always returns `None`, as diffuse light does not scatter.
    #[must_use]
    fn scatter(
        &self,
        _ray: &Ray,
        _hit_record: &HitRecord,
        _rng: &mut SmallRng,
    ) -> Option<ScatterRecord> {
        None
    }

    /// Scattering probability density function for the [`ConeLight`] material. Always returns 0, as diffuse light does not scatter.
    #[must_use]
    fn scattering_pdf(
        &self,
        _hit_record: &HitRecord,
        _scattered: &Ray,
        _rng: &mut SmallRng,
    ) -> Option<Float> {
        None
    }

    /// Emission function for [`ConeLight`]. If the given [`HitRecord`] has been hit on the `front_face`, emit a color based on the texture and surface coordinates. Otherwise, emit pure black.
    #[must_use]
    fn emit(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        u: Float,
        v: Float,
        position: Position,
    ) -> Xyz<E> {
        // If we don't hit the front face, return black
        if !hit_record.front_face {
            return Xyz::new(0.0, 0.0, 0.0);
        }

        // We have hit the front. Calculate the angle of incidence
        let spread_radians = self.spread.to_radians();
        let angle = (-ray.direction.dot(&hit_record.normal)
            / (ray.direction.magnitude() * hit_record.normal.magnitude()))
        .acos();

        let emit = self.emit.color(u, v, position);
        if angle <= spread_radians {
            emit
        } else {
            // Make sure that the front face of the lamp is tinted, even outside the main lighting angle
            let (r, g, b) = emit.into_components();
            let scaling_factor = r.max(g).max(b);
            if scaling_factor > 1.0 {
                emit / scaling_factor
            } else {
                emit
            }
        }
    }
}

impl ConeLight {
    /// Creates a new [`ConeLight`] material with the given [Texture].
    #[must_use]
    pub fn new(spread: Float, emit: impl Into<Texture>) -> Self {
        ConeLight {
            spread,
            emit: emit.into(),
        }
    }
}
