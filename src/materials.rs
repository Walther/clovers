use crate::{color::Color, hitable::HitRecord, textures::Texture, Float, Ray, ThreadRng, Vec3, PI};
use rand::prelude::*;
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
use std::sync::Arc;

#[derive(Copy, Clone)]
pub enum Material {
    Dielectric { refractive_index: Float },
    Lambertian { albedo: Texture },
    DiffuseLight { emit: Texture },
    Metal { albedo: Texture, fuzz: Float },
    Isotropic { albedo: Texture },
}

impl Material {
    pub fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: ThreadRng,
    ) -> Option<(Ray, Color)> {
        match self {
            Material::Dielectric { refractive_index } => {
                Dielectric::scatter(refractive_index, ray, hit_record, rng)
            }
            Material::Lambertian { albedo } => Lambertian::scatter(albedo, ray, hit_record, rng),
            Material::DiffuseLight { emit } => DiffuseLight::scatter(emit, ray, hit_record, rng),
            Material::Metal { albedo, fuzz } => Metal::scatter(albedo, fuzz, ray, hit_record, rng),
            Material::Isotropic { albedo } => Isotropic::scatter(albedo, ray, hit_record, rng),
        }
    }
    /// Returns the amount of light the material emits. By default, materials do not emit light, returning black.
    pub fn emit(&self, u: Float, v: Float, position: Vec3) -> Color {
        match self {
            Material::DiffuseLight { emit } => DiffuseLight::emit(*emit, u, v, position),
            _ => Color::new(0.0, 0.0, 0.0),
        }
    }
}

// Internal helper. Originally used for lambertian reflection with flaws
fn random_in_unit_sphere(mut rng: ThreadRng) -> Vec3 {
    let mut position: Vec3;
    loop {
        position = 2.0 * Vec3::new(rng.gen(), rng.gen(), rng.gen()) - Vec3::new(1.0, 1.0, 1.0);
        if position.magnitude_squared() >= 1.0 {
            return position;
        }
    }
}

// Internal helper. Use this for the more correct "True Lambertian" reflection
fn random_unit_vector(mut rng: ThreadRng) -> Vec3 {
    let a: Float = rng.gen_range(0.0, 2.0 * PI);
    let z: Float = rng.gen_range(-1.0, 1.0);
    let r: Float = (1.0 - z * z).sqrt();
    return Vec3::new(r * a.cos(), r * a.sin(), z);
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
