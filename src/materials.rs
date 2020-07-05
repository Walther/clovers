use serde::{Deserialize, Serialize};

use crate::{color::Color, hitable::HitRecord, Float, Ray, ThreadRng, Vec3};
pub mod dielectric;
pub mod diffuse_light;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

pub use dielectric::*;
pub use diffuse_light::*;
pub use isotropic::*;
pub use lambertian::*;
pub use metal::*;

#[derive(Copy, Clone, Deserialize, Serialize)]
pub enum Material {
    Dielectric(Dielectric),
    Lambertian(Lambertian),
    DiffuseLight(DiffuseLight),
    Metal(Metal),
    Isotropic(Isotropic),
}

impl Material {
    pub fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: ThreadRng,
    ) -> Option<(Ray, Color)> {
        match *self {
            Material::Dielectric(d) => Dielectric::scatter(d, ray, hit_record, rng),
            Material::Lambertian(l) => Lambertian::scatter(l, ray, hit_record, rng),
            Material::DiffuseLight(d) => DiffuseLight::scatter(d, ray, hit_record, rng),
            Material::Metal(m) => Metal::scatter(m, ray, hit_record, rng),
            Material::Isotropic(i) => Isotropic::scatter(i, ray, hit_record, rng),
        }
    }
    /// Returns the amount of light the material emits. By default, materials do not emit light, returning black.
    pub fn emit(&self, u: Float, v: Float, position: Vec3) -> Color {
        match *self {
            Material::DiffuseLight(d) => DiffuseLight::emit(d, u, v, position),
            _ => Color::new(0.0, 0.0, 0.0),
        }
    }
}

fn reflect(vector: Vec3, normal: Vec3) -> Vec3 {
    vector - 2.0 * vector.dot(&normal) * normal
}

fn refract(uv: Vec3, normal: Vec3, etai_over_etat: Float) -> Vec3 {
    let cos_theta: Float = -uv.dot(&normal);
    let r_out_parallel: Vec3 = etai_over_etat * (uv + cos_theta * normal);
    let r_out_perp: Vec3 = -(1.0 - r_out_parallel.norm_squared()).sqrt() * normal;
    return r_out_parallel + r_out_perp;
}

fn schlick(cosine: Float, refractive_index: Float) -> Float {
    let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * ((1.0 - cosine).powf(5.0))
}
