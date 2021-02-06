use super::{reflect, refract, schlick, Material, MaterialType, ScatterRecord};
use crate::{color::Color, hitable::HitRecord, pdf::ZeroPDF, ray::Ray, Float, Vec3};
use rand::prelude::*;

use serde::{Deserialize, Serialize};

/// A dielectric material. This resembless glass and other transparent and reflective materials.
#[derive(Copy, Clone, Deserialize, Serialize, Debug)]
pub struct Dielectric {
    /// Refractive index of the material. Used for calculating the new direction of a ray when entering the material at an angle. Follows Snell's law of refraction. Default value: 1.5, based on typical window glass.
    #[serde(default = "default_index")]
    pub refractive_index: Float,
    /// Color of the material. Used for colorizing the rays. Default value: [`Color::new(1.0, 1.0, 1.0)`](crate::color::Color), producing a fully transparent, clear glass.
    #[serde(default = "default_color")]
    pub color: Color,
}

fn default_index() -> Float {
    1.5
}

fn default_color() -> Color {
    Color::new(1.0, 1.0, 1.0)
}

impl<'a> Dielectric {
    pub fn scatter(
        self,
        ray: &Ray,
        hit_record: &HitRecord,
        mut rng: ThreadRng,
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
            Some(ScatterRecord {
                material_type: MaterialType::Specular,
                specular_ray: Some(specular_ray),
                attenuation: albedo,
                pdf_ptr: ZeroPDF::new(), //TODO: ugly hack due to nullptr in original tutorial
            })
        } else {
            let reflect_probability: Float = schlick(cos_theta, etai_over_etat);
            if rng.gen::<Float>() < reflect_probability {
                let reflected: Vec3 = reflect(unit_direction, hit_record.normal);
                specular_ray = Ray::new(hit_record.position, reflected, ray.time);
                Some(ScatterRecord {
                    material_type: MaterialType::Specular,
                    specular_ray: Some(specular_ray),
                    attenuation: albedo,
                    pdf_ptr: ZeroPDF::new(), //TODO: ugly hack due to nullptr in original tutorial
                })
            } else {
                let refracted: Vec3 = refract(unit_direction, hit_record.normal, etai_over_etat);
                specular_ray = Ray::new(hit_record.position, refracted, ray.time);
                Some(ScatterRecord {
                    material_type: MaterialType::Specular,
                    specular_ray: Some(specular_ray),
                    attenuation: albedo,
                    pdf_ptr: ZeroPDF::new(), //TODO: ugly hack due to nullptr in original tutorial
                })
            }
        }
    }

    pub fn scattering_pdf(
        self,
        _ray: &Ray,
        _hit_record: &HitRecord,
        _scattered: &Ray,
        _rng: ThreadRng,
    ) -> Float {
        todo!()
    }

    pub fn new(refractive_index: Float, color: Color) -> Material {
        Material::Dielectric(Dielectric {
            refractive_index,
            color,
        })
    }
}
