use super::{Material, MaterialType, ScatterRecord};
use crate::{
    color::Color, hitable::HitRecord, pdf::CosinePDF, ray::Ray, textures::Texture, Float, PI,
};
use rand::prelude::ThreadRng;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub struct Isotropic {
    albedo: Texture,
}

impl<'a> Isotropic {
    pub fn new(emission: Texture) -> dyn Material {
        Material::Isotropic(Isotropic { albedo: emission })
    }

    pub fn scatter(
        self,
        _ray: &Ray,
        hit_record: &HitRecord,
        _rng: ThreadRng,
    ) -> Option<ScatterRecord<'a>> {
        // TODO: fix / verify correctness!
        // this is just copied from lambertian as an experiment
        let albedo: Color = self
            .albedo
            .color(hit_record.u, hit_record.v, hit_record.position);

        Some(ScatterRecord {
            material_type: MaterialType::Diffuse,
            specular_ray: None,
            attenuation: albedo,
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
        // TODO: fix / verify correctness!
        // this is just copied from lambertian as an experiment
        let cosine = hit_record.normal.dot(&scattered.direction.normalize());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
}
