use super::{reflect, refract, schlick, Material};
use crate::{color::Color, hitable::HitRecord, ray::Ray, Float, Vec3};
use rand::prelude::*;

#[derive(Clone)]
pub struct Dielectric {
    refractive_index: Float,
}

impl Dielectric {
    pub fn scatter(
        refractive_index: &Float,
        ray: &Ray,
        hit_record: &HitRecord,
        mut rng: ThreadRng,
    ) -> Option<(Ray, Color)> {
        let attenuation: Color = Color::new(1.0, 1.0, 1.0); // Glass does not attenuate
        let scattered: Ray;
        let etai_over_etat: Float = match hit_record.front_face {
            true => 1.0 / refractive_index,
            false => *refractive_index,
        };

        let unit_direction: Vec3 = ray.direction.normalize();
        let cos_theta: Float = (-unit_direction.dot(&hit_record.normal)).min(1.0);
        let sin_theta: Float = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            let reflected: Vec3 = reflect(unit_direction, hit_record.normal);
            scattered = Ray::new(hit_record.position, reflected, ray.time)
        } else {
            let reflect_probability: Float = schlick(cos_theta, etai_over_etat);
            if rng.gen::<Float>() < reflect_probability {
                let reflected: Vec3 = reflect(unit_direction, hit_record.normal);
                scattered = Ray::new(hit_record.position, reflected, ray.time);
            } else {
                let refracted: Vec3 = refract(unit_direction, hit_record.normal, etai_over_etat);
                scattered = Ray::new(hit_record.position, refracted, ray.time);
            }
        }

        Some((scattered, attenuation))
    }

    pub fn new(refractive_index: Float) -> Self {
        Dielectric { refractive_index }
    }
}
