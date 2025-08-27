//! A dielectric material. This resembles glass and other transparent and reflective materials.

use super::{reflect, refract, schlick, MaterialTrait, MaterialType, ScatterRecord};
use crate::{
    pdf::{ZeroPDF, PDF},
    ray::Ray,
    spectrum::spectral_power,
    wavelength::Wavelength,
    Direction, Float, HitRecord,
};
use palette::{white_point::E, Xyz};
use rand::rngs::SmallRng;
use rand::Rng;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// A dielectric material. This resembles glass and other transparent and reflective materials.
pub struct Dielectric {
    /// Refractive index of the material. Used for calculating the new direction of a ray when entering the material at an angle. Follows Snell's law of refraction. Default value: 1.5, based on typical window glass.
    #[cfg_attr(feature = "serde-derive", serde(default = "default_index"))]
    pub refractive_index: Float,
    /// Color of the material. Used for colorizing the rays. Default value: [`(1.0, 1.0, 1.0)`], producing a fully transparent, clear glass.
    #[cfg_attr(feature = "serde-derive", serde(default = "default_color"))]
    pub color: Xyz<E>,
}

fn default_index() -> Float {
    1.5
}

fn default_color() -> Xyz<E> {
    Xyz::new(1.0, 1.0, 1.0)
}

impl MaterialTrait for Dielectric {
    /// Scatter method for the Dielectric material. Given a `ray` and a `hit_record`, evaluate a [`ScatterRecord`] based on possible reflection or refraction.
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut SmallRng,
    ) -> Option<ScatterRecord<'_>> {
        let refraction_ratio: Float = if hit_record.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let direction: Direction = ray.direction;
        let cos_theta: Float = (-direction.dot(&hit_record.normal)).min(1.0);
        let sin_theta: Float = (1.0 - cos_theta * cos_theta).sqrt();
        let specular_direction: Direction = if refraction_ratio * sin_theta > 1.0 {
            reflect(direction, hit_record.normal)
        } else {
            let reflect_probability: Float = schlick(cos_theta, refraction_ratio);
            if rng.random::<Float>() < reflect_probability {
                reflect(direction, hit_record.normal)
            } else {
                // Refracted
                refract(direction, hit_record.normal, refraction_ratio)
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
            pdf_ptr: PDF::ZeroPDF(ZeroPDF::new()), //TODO: ugly hack due to nullptr in original tutorial
        })
    }

    // TODO: should this material provide a `scattering_pdf` function?

    fn color(&self, _ray: &Ray, wavelength: Wavelength, _hit_record: &HitRecord) -> Float {
        spectral_power(self.color, wavelength)
    }
}

impl Default for Dielectric {
    fn default() -> Self {
        Dielectric {
            refractive_index: default_index(),
            color: default_color(),
        }
    }
}
