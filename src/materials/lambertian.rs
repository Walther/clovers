use super::{Material, MaterialType, ScatterRecord};
use crate::{
    color::Color,
    hitable::HitRecord,
    pdf::CosinePDF,
    ray::Ray,
    textures::{SolidColor, Texture},
    Float, PI,
};
use rand::prelude::ThreadRng;
use serde::{Deserialize, Serialize};

impl Default for Lambertian {
    fn default() -> Self {
        Lambertian {
            // TODO: why does this have to be so manual? Compare to:
            // SolidColor::default()
            albedo: SolidColor::new(Color::default()),
        }
    }
}

#[derive(Copy, Clone, Deserialize, Serialize, Debug)]
pub struct Lambertian {
    albedo: Texture,
}

impl<'a> Lambertian {
    /// Returns None, if ray is absorbed. Otherwise, returns a ray, albedo of what was hit, and (?) a value used for probability density function based sampling
    pub fn scatter(
        self,
        _ray: &Ray,
        hit_record: &HitRecord,
        _rng: ThreadRng,
    ) -> Option<ScatterRecord<'a>> {
        Some(ScatterRecord {
            material_type: MaterialType::Diffuse,
            specular_ray: None,
            attenuation: self
                .albedo
                .color(hit_record.u, hit_record.v, hit_record.position),
            pdf_ptr: CosinePDF::new(hit_record.normal),
        })
    }

    pub fn scattering_pdf(
        self,
        _ray: &Ray,
        hit_record: &HitRecord,
        scattered: &Ray,
        _rng: ThreadRng,
    ) -> Float {
        let cosine = hit_record.normal.dot(&scattered.direction.normalize());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }

    pub fn new(albedo: Texture) -> dyn Material {
        Material::Lambertian(Lambertian { albedo })
    }
}
