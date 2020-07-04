use super::{random_in_unit_sphere, Material};
use crate::{color::Color, hitable::HitRecord, ray::Ray, textures::Texture};
use rand::prelude::ThreadRng;
pub struct Isotropic {
    albedo: Texture,
}

impl Isotropic {
    pub fn new(emission: Texture) -> Material {
        Material::Isotropic { albedo: emission }
    }

    pub fn scatter(
        albedo: &Texture,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: ThreadRng,
    ) -> Option<(Ray, Color)> {
        let scattered: Ray = Ray::new(hit_record.position, random_in_unit_sphere(rng), ray.time);
        let attenuation: Color = albedo.color(hit_record.u, hit_record.v, hit_record.position);

        Some((scattered, attenuation))
    }
}
