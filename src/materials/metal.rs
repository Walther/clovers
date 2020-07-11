use super::{reflect, Material, MaterialType, ScatterRecord};
use crate::{
    hitable::HitRecord, pdf::ZeroPDF, random::random_in_unit_sphere, ray::Ray, textures::Texture,
    Float, Vec3,
};
use rand::prelude::ThreadRng;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub struct Metal {
    albedo: Texture,
    fuzz: Float,
}

impl Metal {
    pub fn scatter(
        self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: ThreadRng,
    ) -> Option<ScatterRecord> {
        let reflected: Vec3 = reflect(ray.direction.normalize(), hit_record.normal);
        Some(ScatterRecord {
            specular_ray: Some(Ray::new(
                hit_record.position,
                reflected + self.fuzz * random_in_unit_sphere(rng),
                ray.time,
            )),
            attenuation: self.albedo.color(0.0, 0.0, hit_record.position), // NOTE: random coords...
            material_type: MaterialType::Specular,
            pdf_ptr: ZeroPDF::new(),
        })
    }

    pub fn scattering_pdf(
        self,
        _ray: &Ray,
        _hit_record: &HitRecord,
        _scattered: &Ray,
        _rng: ThreadRng,
    ) -> Float {
        0.0 // TODO: why does metal scatter 0? No mention in tutorial afaiu
    }

    pub fn new(albedo: Texture, fuzz: Float) -> Material {
        Material::Metal(Metal {
            albedo,
            fuzz: fuzz.min(1.0),
        })
    }
}
