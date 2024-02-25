//! Dispersive material.
//! Based on [Cauchy's equation](https://en.wikipedia.org/wiki/Cauchy%27s_equation)

/*
Material 	                A 	        B (μm2)
Fused silica          	    1.4580 	    0.00354
Borosilicate glass BK7 	    1.5046 	    0.00420
Hard crown glass K5 	    1.5220 	    0.00459
Barium crown glass BaK4 	1.5690 	    0.00531
Barium flint glass BaF10 	1.6700 	    0.00743
Dense flint glass SF10 	    1.7280 	    0.01342
*/

// TODO: consider other options, e.g. Sellmeier https://en.wikipedia.org/wiki/Sellmeier_equation

use palette::Xyz;
use rand::{rngs::SmallRng, Rng};

use crate::{
    hitable::HitRecord,
    pdf::{ZeroPDF, PDF},
    ray::Ray,
    wavelength::Wavelength,
    Float, Vec3,
};

use super::{reflect, refract, schlick, MaterialTrait, MaterialType, ScatterRecord};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// A dispersive glass material.
pub struct Dispersive {
    /// Cauchy coefficient A of the material
    #[cfg_attr(feature = "serde-derive", serde(default = "default_a"))]
    pub cauchy_a: Float,
    /// Cauchy coefficient B of the material
    #[cfg_attr(feature = "serde-derive", serde(default = "default_b"))]
    pub cauchy_b: Float,
}

fn default_a() -> Float {
    1.5046
}

fn default_b() -> Float {
    0.00420
}

// TODO: less precision loss?
#[allow(clippy::cast_precision_loss)]
impl Dispersive {
    /// Creates a new [Dispersive] material with the given Cauchy equation constants.
    #[must_use]
    pub fn new(cauchy_a: Float, cauchy_b: Float) -> Self {
        Dispersive { cauchy_a, cauchy_b }
    }

    /// Calculates the refractive index of the material for the given wavelength
    #[must_use]
    pub fn refractive_index(&self, wavelength: Wavelength) -> Float {
        let wave_micros = wavelength as Float / 1000.0;
        self.cauchy_a + (self.cauchy_b / (wave_micros * wave_micros))
    }
}

impl Default for Dispersive {
    fn default() -> Self {
        Dispersive::new(default_a(), default_b())
    }
}

impl MaterialTrait for Dispersive {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut SmallRng,
    ) -> Option<ScatterRecord> {
        // Calculate refractive index based on the wavelength of the incoming material
        // TODO: colored dispersive glass?
        let attenuation = Xyz::new(1.0, 1.0, 1.0);
        let refractive_index = self.refractive_index(ray.wavelength);
        let refraction_ratio: Float = if hit_record.front_face {
            1.0 / refractive_index
        } else {
            refractive_index
        };

        // Copied from Dielectric, is this correct?
        let unit_direction: Vec3 = ray.direction.normalize();
        let cos_theta: Float = (-unit_direction.dot(&hit_record.normal)).min(1.0);
        let sin_theta: Float = (1.0 - cos_theta * cos_theta).sqrt();
        let specular_direction: Vec3 = if refraction_ratio * sin_theta > 1.0 {
            reflect(unit_direction, hit_record.normal)
        } else {
            let reflect_probability: Float = schlick(cos_theta, refraction_ratio);
            if rng.gen::<Float>() < reflect_probability {
                reflect(unit_direction, hit_record.normal)
            } else {
                // Refracted
                refract(unit_direction, hit_record.normal, refraction_ratio)
            }
        };
        let specular_ray = Ray {
            origin: hit_record.position,
            direction: specular_direction,
            time: ray.time,
            wavelength: ray.wavelength,
        };

        Some(ScatterRecord {
            material_type: MaterialType::Specular,
            specular_ray: Some(specular_ray),
            attenuation,
            pdf_ptr: PDF::ZeroPDF(ZeroPDF::new()), //TODO: ugly hack due to nullptr in original tutorial
        })
        // End copied
    }

    fn scattering_pdf(
        &self,
        _hit_record: &HitRecord,
        _scattered: &Ray,
        _rng: &mut SmallRng,
    ) -> Option<Float> {
        None // TODO: should a dispersive material scatter? how much?
    }
}
