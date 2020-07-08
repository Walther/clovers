use super::{reflect, refract, schlick, Material, MaterialType, ScatterRecord};
use crate::{color::Color, hitable::HitRecord, pdf::ZeroPDF, ray::Ray, Float, Vec3};
use rand::prelude::*;

#[derive(Copy, Clone)]
pub struct Dielectric {
    refractive_index: Float,
}

impl Dielectric {
    pub fn scatter(
        self,
        ray: &Ray,
        hit_record: &HitRecord,
        mut rng: ThreadRng,
    ) -> Option<ScatterRecord> {
        let albedo: Color = Color::new(1.0, 1.0, 1.0); // Glass does not attenuate
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

    pub fn new(refractive_index: Float) -> Material {
        Material::Dielectric(Dielectric { refractive_index })
    }
}
