use super::{random_unit_vector, Material};
use crate::{color::Color, hitable::HitRecord, ray::Ray, textures::Texture, Vec3};
use rand::prelude::ThreadRng;

#[derive(Copy, Clone)]
pub struct Lambertian {
    albedo: Texture,
}

impl Lambertian {
    pub fn scatter(
        self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: ThreadRng,
    ) -> Option<(Ray, Color)> {
        let scatter_direction: Vec3 = hit_record.normal + random_unit_vector(rng);
        let scattered = Ray::new(hit_record.position, scatter_direction, ray.time);
        let attenuation = self
            .albedo
            .color(hit_record.u, hit_record.v, hit_record.position);
        Some((scattered, attenuation))
    }

    pub fn new(albedo: Texture) -> Material {
        Material::Lambertian(Lambertian { albedo })
    }
}
