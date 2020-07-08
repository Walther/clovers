use super::{Material, ScatterRecord};
use crate::{color::Color, hitable::HitRecord, ray::Ray, textures::Texture, Float, Vec3};
use rand::prelude::ThreadRng;

#[derive(Copy, Clone)]
pub struct DiffuseLight {
    emit: Texture,
}

impl DiffuseLight {
    pub fn scatter(
        self,
        _ray: &Ray,
        _hit_record: &HitRecord,
        _rng: ThreadRng,
    ) -> Option<ScatterRecord> {
        None
    }

    pub fn scattering_pdf(
        self,
        _ray: &Ray,
        _hit_record: &HitRecord,
        _scattered: &Ray,
        _rng: ThreadRng,
    ) -> Float {
        0.0 // TODO: cleanup
    }

    pub fn emit(
        self,
        _ray: &Ray,
        hit_record: &HitRecord,
        u: Float,
        v: Float,
        position: Vec3,
    ) -> Color {
        if hit_record.front_face {
            self.emit.color(u, v, position)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    }

    pub fn new(emission: Texture) -> Material {
        Material::DiffuseLight(DiffuseLight { emit: emission })
    }
}
