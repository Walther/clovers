use super::Material;
use crate::{color::Color, hitable::HitRecord, ray::Ray, textures::Texture, Float, Vec3};
use rand::prelude::ThreadRng;
use std::sync::Arc;
pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _ray: &Ray,
        _hit_record: &HitRecord,
        _rng: ThreadRng,
    ) -> Option<(Ray, Color)> {
        None
    }
    fn emitted(&self, u: Float, v: Float, position: Vec3) -> Color {
        self.emit.color(u, v, position)
    }
}

impl DiffuseLight {
    pub fn new(emission: Arc<dyn Texture>) -> Self {
        DiffuseLight { emit: emission }
    }
}
