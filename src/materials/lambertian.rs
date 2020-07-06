use super::{Material, MaterialType, ScatterRecord};
use crate::{
    color::Color,
    hitable::HitRecord,
    onb::ONB,
    pdf::CosinePDF,
    random::{random_cosine_direction, random_unit_vector},
    ray::Ray,
    textures::Texture,
    Float, Vec3, PI,
};
use rand::prelude::ThreadRng;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct Lambertian {
    albedo: Texture,
}

impl Lambertian {
    /// Returns None, if ray is absorbed. Otherwise, returns a ray, albedo of what was hit, and (?) a value used for probability density function based sampling
    pub fn scatter(
        self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: ThreadRng,
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
        ray: &Ray,
        hit_record: &HitRecord,
        scattered: &Ray,
        rng: ThreadRng,
    ) -> Float {
        let cosine = hit_record.normal.dot(&scattered.direction.normalize());
        if cosine < 0.0 {
            return 0.0;
        } else {
            return cosine / PI;
        }
    }

    pub fn new(albedo: Texture) -> Material {
        Material::Lambertian(Lambertian { albedo })
    }
}
