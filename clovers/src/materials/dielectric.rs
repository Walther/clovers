//! A dielectric material. This resembles glass and other transparent and reflective materials.

use super::{reflect, refract, schlick, MaterialType, ScatterRecord};
use crate::{
    color::Color,
    hitable::HitRecord,
    pdf::{ZeroPDF, PDF},
    ray::Ray,
    Float, Vec3,
};
use rand::rngs::SmallRng;
use rand::Rng;

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// A dielectric material. This resembles glass and other transparent and reflective materials.
pub struct Dielectric {
    /// Refractive index of the material. Used for calculating the new direction of a ray when entering the material at an angle. Follows Snell's law of refraction. Default value: 1.5, based on typical window glass.
    #[cfg_attr(feature = "serde-derive", serde(default = "default_index"))]
    pub refractive_index: Float,
    /// Color of the material. Used for colorizing the rays. Default value: [`Color::new(1.0, 1.0, 1.0)`](crate::color::Color), producing a fully transparent, clear glass.
    #[cfg_attr(feature = "serde-derive", serde(default = "default_color"))]
    pub color: Color,
}

fn default_index() -> Float {
    1.5
}

fn default_color() -> Color {
    Color::new(1.0, 1.0, 1.0)
}

impl<'a> Dielectric {
    /// Scatter method for the Dielectric material. Given a `ray` and a `hit_record`, evaluate a [`ScatterRecord`] based on possible reflection or refraction.
    pub fn scatter(
        self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut SmallRng,
    ) -> Option<ScatterRecord<'a>> {
        let albedo = self.color;
        let specular_ray: Ray;

        let etai_over_etat: Float = match hit_record.front_face {
            true => 1.0 / self.refractive_index,
            false => self.refractive_index,
        };

        let unit_direction: Vec3 = ray.direction.normalize();
        let cos_theta: Float = (-unit_direction.dot(&hit_record.normal)).min(1.0);
        let sin_theta: Float = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            let reflected: Vec3 = reflect(unit_direction, hit_record.normal);
            specular_ray = Ray::new(hit_record.position, reflected, ray.time);
        } else {
            let reflect_probability: Float = schlick(cos_theta, etai_over_etat);
            if rng.gen::<Float>() < reflect_probability {
                let reflected: Vec3 = reflect(unit_direction, hit_record.normal);
                specular_ray = Ray::new(hit_record.position, reflected, ray.time);
            } else {
                let refracted: Vec3 = refract(unit_direction, hit_record.normal, etai_over_etat);
                specular_ray = Ray::new(hit_record.position, refracted, ray.time);
            }
        }
        Some(ScatterRecord {
            material_type: MaterialType::Specular,
            specular_ray: Some(specular_ray),
            attenuation: albedo,
            pdf_ptr: PDF::ZeroPDF(ZeroPDF::new()), //TODO: ugly hack due to nullptr in original tutorial
        })
    }

    /// Scattering probability density function for Dielectric material. NOTE: not implemented!
    #[allow(clippy::unused_self)] // TODO
    pub fn scattering_pdf(
        self,
        _ray: &Ray,
        _hit_record: &HitRecord,
        _scattered: &Ray,
        _rng: &mut SmallRng,
    ) -> Float {
        todo!()
    }

    /// Creates a new [Dielectric] material with the given refractive index and color.
    #[must_use]
    pub fn new(refractive_index: Float, color: Color) -> Self {
        Dielectric {
            refractive_index,
            color,
        }
    }
}

impl Default for Dielectric {
    fn default() -> Self {
        Dielectric::new(default_index(), default_color())
    }
}
