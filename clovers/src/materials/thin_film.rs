//! An iridescence feature based on thin-film interference.

use core::f32::consts::PI;

use crate::{ray::Ray, Float, HitRecord};

#[derive(Clone, Debug)]
/// An iridescence feature based on thin-film interference.
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde-derive", serde(default))]
pub struct ThinFilm {
    /// Refractive index of the material.
    pub refractive_index: Float,
    /// Thickness of the film in nanometers.
    pub thickness: Float,
}

impl ThinFilm {
    /// Creates a new instance of [`ThinFilm`] with the specified `refractive_index` and `thickness` in nanometers.
    #[must_use]
    pub fn new(refractive_index: Float, thickness: Float) -> Self {
        Self {
            refractive_index,
            thickness,
        }
    }

    /// Calculates the strength of the interference. This should be used as a multiplier to the material's albedo. Range: `0..2` inclusive, with area 1, preserving energy conversation across spectrum.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn interference(&self, ray: &Ray, hit_record: &HitRecord) -> Float {
        // Assume ray coming from air
        // TODO: other material interfaces?
        let n1 = 1.0;
        let n2 = self.refractive_index;
        let n_ratio = n1 / n2;

        // https://en.wikipedia.org/wiki/Snell%27s_law#Vector_form
        let cos_theta_1: Float = -ray.direction.dot(&hit_record.normal);
        let sin_theta_1: Float = (1.0 - cos_theta_1 * cos_theta_1).sqrt();
        let sin_theta_2: Float = n_ratio * sin_theta_1;
        let cos_theta_2: Float = (1.0 - (sin_theta_2 * sin_theta_2)).sqrt();

        // https://en.wikipedia.org/wiki/Thin-film_interference
        let optical_path_difference = 2.0 * self.refractive_index * self.thickness * cos_theta_2;
        let m = optical_path_difference / (ray.wavelength as Float);
        // range 0 to 2, area 1
        1.0 + (m * 2.0 * PI).cos()
    }
}

impl Default for ThinFilm {
    fn default() -> Self {
        Self {
            thickness: 500.0,
            refractive_index: 1.5,
        }
    }
}
