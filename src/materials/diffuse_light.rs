use super::Material;
use crate::{color::Color, hitable::HitRecord, ray::Ray, textures::Texture, Float, Vec3};
use rand::prelude::ThreadRng;
use std::sync::Arc;
pub struct DiffuseLight {
    emit: Texture,
}

impl DiffuseLight {
    pub fn scatter(
        emit: &Texture,
        _ray: &Ray,
        _hit_record: &HitRecord,
        _rng: ThreadRng,
    ) -> Option<(Ray, Color)> {
        None
    }

    pub fn emit(emit: Texture, u: Float, v: Float, position: Vec3) -> Color {
        emit.color(u, v, position)
    }

    pub fn new(emission: Texture) -> Self {
        DiffuseLight { emit: emission }
    }
}
