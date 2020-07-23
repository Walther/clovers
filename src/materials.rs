use crate::{color::Color, hitable::HitRecord, pdf::PDF, Float, Ray, Vec3};
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
use rand::prelude::ThreadRng;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub enum Material {
    Dielectric(Dielectric),
    Lambertian(Lambertian),
    DiffuseLight(DiffuseLight),
    Metal(Metal),
    Isotropic(Isotropic),
}

impl Material {
    /// Scatters a ray from the material
    pub fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: ThreadRng,
    ) -> Option<ScatterRecord> {
        match *self {
            Material::Lambertian(l) => Lambertian::scatter(l, ray, hit_record, rng),
            Material::DiffuseLight(d) => DiffuseLight::scatter(d, ray, hit_record, rng),
            Material::Metal(m) => Metal::scatter(m, ray, hit_record, rng),
            Material::Dielectric(d) => Dielectric::scatter(d, ray, hit_record, rng),
            _ => todo!(), // Material::Isotropic(i) => Isotropic::scatter(i, ray, hit_record, rng),
        }
    }

    /// Returns a probability? TODO: understand
    pub fn scattering_pdf(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        scattered: &Ray,
        rng: ThreadRng,
    ) -> Float {
        match *self {
            Material::Dielectric(m) => m.scattering_pdf(ray, hit_record, scattered, rng),
            Material::Lambertian(m) => m.scattering_pdf(ray, hit_record, scattered, rng),
            Material::DiffuseLight(m) => m.scattering_pdf(ray, hit_record, scattered, rng),
            Material::Metal(m) => m.scattering_pdf(ray, hit_record, scattered, rng),
            Material::Isotropic(m) => m.scattering_pdf(ray, hit_record, scattered, rng),
        }
    }

    /// Returns the amount of light the material emits. By default, materials do not emit light, returning black.
    pub fn emit(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        u: Float,
        v: Float,
        position: Vec3,
    ) -> Color {
        match *self {
            Material::DiffuseLight(d) => d.emit(ray, hit_record, u, v, position),
            _ => Color::new(0.0, 0.0, 0.0),
        }
    }
}

pub enum MaterialType {
    Diffuse,
    Specular,
}

pub struct ScatterRecord {
    pub material_type: MaterialType,
    pub specular_ray: Option<Ray>,
    pub attenuation: Color,
    pub pdf_ptr: PDF,
}

fn reflect(vector: Vec3, normal: Vec3) -> Vec3 {
    vector - 2.0 * vector.dot(normal) * normal
}

fn refract(uv: Vec3, normal: Vec3, etai_over_etat: Float) -> Vec3 {
    let cos_theta: Float = -uv.dot(normal);
    let r_out_parallel: Vec3 = etai_over_etat * (uv + cos_theta * normal);
    let r_out_perp: Vec3 = -(1.0 - r_out_parallel.mag_sq()).sqrt() * normal;
    r_out_parallel + r_out_perp
}

fn schlick(cosine: Float, refractive_index: Float) -> Float {
    let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * ((1.0 - cosine).powf(5.0))
}
