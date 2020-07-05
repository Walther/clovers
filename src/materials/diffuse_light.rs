use super::Material;
use crate::{color::Color, hitable::HitRecord, ray::Ray, textures::Texture, Float, Vec3};
use rand::prelude::ThreadRng;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct DiffuseLight {
    emit: Texture,
}

impl DiffuseLight {
    pub fn scatter(
        self,
        _ray: &Ray,
        _hit_record: &HitRecord,
        _rng: ThreadRng,
    ) -> Option<(Ray, Color)> {
        None
    }

    pub fn emit(self, u: Float, v: Float, position: Vec3) -> Color {
        self.emit.color(u, v, position)
    }

    pub fn new(emission: Texture) -> Material {
        Material::DiffuseLight(DiffuseLight { emit: emission })
    }
}
