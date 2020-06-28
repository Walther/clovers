use super::{random_in_unit_sphere, reflect, Material};
use crate::{color::Color, hitable::HitRecord, ray::Ray, textures::Texture, Float, Vec3};
use rand::prelude::ThreadRng;
use std::sync::Arc;
#[derive(Clone)]
pub struct Metal {
    albedo: Arc<dyn Texture>,
    fuzz: Float,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, rng: ThreadRng) -> Option<(Ray, Color)> {
        let reflected: Vec3 = reflect(ray.direction.normalize(), hit_record.normal);
        let scattered: Ray = Ray::new(
            hit_record.position,
            reflected + self.fuzz * random_in_unit_sphere(rng),
            ray.time,
        );
        let attenuation: Color = self
            .albedo
            .color(hit_record.u, hit_record.v, hit_record.position);
        if scattered.direction.dot(&hit_record.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}

impl Metal {
    pub fn new(albedo: Arc<dyn Texture>, fuzz: Float) -> Self {
        Metal {
            albedo: albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}
