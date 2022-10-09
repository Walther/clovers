//! Wrapper for GLTF materials.

use gltf::Material;
use rand::rngs::SmallRng;

use crate::{
    color::Color,
    hitable::HitRecord,
    pdf::{ZeroPDF, PDF},
    random::random_in_unit_sphere,
    ray::Ray,
    Float, Vec3,
};

use super::{reflect, MaterialTrait, MaterialType, ScatterRecord};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// GLTF Material wrapper type
pub struct GLTFMaterial {
    metallic_factor: Float,
    roughness_factor: Float,
    base_color_factor: Color,
    emissive_factor: Color,
}

impl Default for GLTFMaterial {
    fn default() -> Self {
        todo!()
    }
}

impl GLTFMaterial {
    /// Initialize a new GLTF material wrapper
    #[must_use]
    pub fn new(material: &Material) -> Self {
        let metallic_factor = material.pbr_metallic_roughness().metallic_factor();
        let roughness_factor = material.pbr_metallic_roughness().roughness_factor();
        let base_color_factor = material.pbr_metallic_roughness().base_color_factor().into();
        let emissive_factor = material.emissive_factor().into();
        Self {
            metallic_factor,
            roughness_factor,
            base_color_factor,
            emissive_factor,
        }
    }
}

impl MaterialTrait for GLTFMaterial {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut SmallRng,
    ) -> Option<ScatterRecord> {
        // TODO: borrowed from metal, should this be different?
        let reflected: Vec3 = reflect(ray.direction.normalize(), hit_record.normal);
        Some(ScatterRecord {
            specular_ray: Some(Ray::new(
                hit_record.position,
                reflected + self.roughness_factor * random_in_unit_sphere(rng),
                ray.time,
            )),
            // attenuation: self
            //     .albedo
            //     .color(hit_record.u, hit_record.v, hit_record.position),
            // TODO: fetch from texture
            attenuation: self.emissive_factor + self.base_color_factor,
            material_type: MaterialType::Specular,
            pdf_ptr: PDF::ZeroPDF(ZeroPDF::new()),
        })
    }

    fn scattering_pdf(
        &self,
        _ray: &Ray,
        _hit_record: &HitRecord,
        _scattered: &Ray,
        _rng: &mut SmallRng,
    ) -> Option<Float> {
        // TODO: what should this be for GLTF materials?
        todo!()
    }
}
