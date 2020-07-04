use super::{random_in_unit_sphere, reflect, Material};
use crate::{color::Color, hitable::HitRecord, ray::Ray, textures::Texture, Float, Vec3};
use rand::prelude::ThreadRng;

pub struct Metal {
    albedo: Texture,
    fuzz: Float,
}

impl Metal {
    pub fn scatter(
        albedo: &Texture,
        fuzz: &Float,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: ThreadRng,
    ) -> Option<(Ray, Color)> {
        let reflected: Vec3 = reflect(ray.direction.normalize(), hit_record.normal);
        let scattered: Ray = Ray::new(
            hit_record.position,
            reflected + *fuzz * random_in_unit_sphere(rng),
            ray.time,
        );
        let attenuation: Color = albedo.color(hit_record.u, hit_record.v, hit_record.position);
        if scattered.direction.dot(&hit_record.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }

    pub fn new(albedo: Texture, fuzz: Float) -> Self {
        Metal {
            albedo: albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}
