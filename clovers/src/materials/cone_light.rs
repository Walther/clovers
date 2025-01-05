//! A cone light material.

use super::{MaterialTrait, ScatterRecord};
use crate::{
    ray::Ray,
    textures::{SolidColor, Texture, TextureTrait},
    wavelength::Wavelength,
    Float, HitRecord,
};
use palette::Xyz;
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

    /// Emission function for [`ConeLight`]. If the given [`HitRecord`] has been hit on the `front_face`, emit a color based on the texture and surface coordinates. Otherwise, emit pure black.
    #[must_use]
    fn emit(&self, ray: &Ray, wavelength: Wavelength, hit_record: &HitRecord) -> Float {
        // If we don't hit the front face, return black
        if !hit_record.front_face {
            return 0.0;
        }

        // We have hit the front. Calculate the angle of incidence
        let spread_radians = self.spread.to_radians();
        let angle = (-ray.direction.dot(&hit_record.normal)
            / (ray.direction.magnitude() * hit_record.normal.magnitude()))
        .acos();

        if angle > spread_radians {
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

    #[must_use]
    fn color(&self, ray: &Ray, wavelength: Wavelength, hit_record: &HitRecord) -> Float {
        self.emit.color(ray, wavelength, hit_record).clamp(0.0, 1.0)
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
