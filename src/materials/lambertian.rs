use super::{Material, MaterialType, ScatterRecord};
use crate::{hitable::HitRecord, pdf::CosinePDF, ray::Ray, textures::Texture, Float, PI};
use rand::prelude::ThreadRng;

#[derive(Copy, Clone)]
pub struct Lambertian {
    albedo: Texture,
}

impl Lambertian {
    /// Returns None, if ray is absorbed. Otherwise, returns a ray, albedo of what was hit, and (?) a value used for probability density function based sampling
    pub fn scatter(
        self,
        _ray: &Ray,
        hit_record: &HitRecord,
        _rng: ThreadRng,
    ) -> Option<ScatterRecord> {
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

    pub fn new(albedo: Texture) -> Material {
        Material::Lambertian(Lambertian { albedo })
    }
}
