use super::Material;
use crate::{
    color::Color, hitable::HitRecord, random::random_in_unit_sphere, ray::Ray, textures::Texture,
    Float,
};
use rand::prelude::ThreadRng;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct Isotropic {
    albedo: Texture,
}

impl Isotropic {
    pub fn new(emission: Texture) -> Material {
        Material::Isotropic(Isotropic { albedo: emission })
    }

    pub fn scatter(
        self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: ThreadRng,
    ) -> Option<(Ray, Color, Float)> {
        let scattered: Ray = Ray::new(hit_record.position, random_in_unit_sphere(rng), ray.time);
        let albedo: Color = self
            .albedo
            .color(hit_record.u, hit_record.v, hit_record.position);

        let pdf = 1.0; // TODO:

        Some((scattered, albedo, pdf))
    }

    pub fn scattering_pdf(
        self,
        ray: &Ray,
        hit_record: &HitRecord,
        scattered: &Ray,
        rng: ThreadRng,
    ) -> Float {
        todo!()
    }
}
