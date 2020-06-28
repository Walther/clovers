use super::{random_in_unit_sphere, Material};
use crate::{color::Color, hitable::HitRecord, ray::Ray, textures::Texture};
use rand::prelude::ThreadRng;
use std::sync::Arc;
pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(emission: Arc<dyn Texture>) -> Self {
        Isotropic { albedo: emission }
    }
}

impl Material for Isotropic {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, rng: ThreadRng) -> Option<(Ray, Color)> {
        let scattered: Ray = Ray::new(hit_record.position, random_in_unit_sphere(rng), ray.time);
        let attenuation: Color = self
            .albedo
            .color(hit_record.u, hit_record.v, hit_record.position);

        Some((scattered, attenuation))
    }
}
