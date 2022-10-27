//! Wrapper for GLTF materials.

#![allow(clippy::pedantic)]

use gltf::{image::Data, Material};
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
    #[cfg_attr(feature = "serde-derive", serde(skip))]
    base_color_texture: Option<&'static Data>,
    #[cfg_attr(feature = "serde-derive", serde(skip))]
    tex_coords: [[Float; 2]; 3],
}

impl Default for GLTFMaterial {
    fn default() -> Self {
        todo!()
    }
}

impl GLTFMaterial {
    /// Initialize a new GLTF material wrapper
    #[must_use]
    pub fn new(material: &Material, tex_coords: [[Float; 2]; 3], images: &'static [Data]) -> Self {
        let metallic_factor = material.pbr_metallic_roughness().metallic_factor();
        let roughness_factor = material.pbr_metallic_roughness().roughness_factor();
        let base_color_factor = material.pbr_metallic_roughness().base_color_factor().into();
        let emissive_factor = material.emissive_factor().into();
        let base_color_texture =
            if let Some(info) = material.pbr_metallic_roughness().base_color_texture() {
                let index = info.texture().index();
                Some(&images[index])
            } else {
                None
            };

        Self {
            metallic_factor,
            roughness_factor,
            base_color_factor,
            emissive_factor,
            base_color_texture,
            tex_coords,
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
        let direction = reflected + self.roughness_factor * random_in_unit_sphere(rng);

        // TODO: proper fully correct coloring
        let base_color = match &self.base_color_texture {
            Some(image) => {
                match image.format {
                    // TODO: deduplicate
                    gltf::image::Format::R8G8B8 => {
                        let (x, y) = self.get_texture_coords(hit_record, image);
                        let index = 3 * (x + image.width as usize * y);

                        let r = image.pixels[index];
                        let g = image.pixels[index + 1];
                        let b = image.pixels[index + 2];
                        let color = Color::from([r, g, b]);
                        self.base_color_factor * color
                    }
                    gltf::image::Format::R8G8B8A8 => {
                        let (x, y) = self.get_texture_coords(hit_record, image);
                        let index = 4 * (x + image.width as usize * y);

                        let r = image.pixels[index];
                        let g = image.pixels[index + 1];
                        let b = image.pixels[index + 2];
                        let color = Color::from([r, g, b]);
                        self.base_color_factor * color
                    }
                    _ => todo!(),
                }
            }
            None => self.base_color_factor,
        };

        // Combine
        let attenuation = self.emissive_factor + base_color;

        Some(ScatterRecord {
            specular_ray: Some(Ray::new(hit_record.position, direction, ray.time)),
            attenuation,
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

impl GLTFMaterial {
    fn get_texture_coords(&self, hit_record: &HitRecord, image: &&Data) -> (usize, usize) {
        // Find the correct texture coordinates
        let tex_corner0 = Vec3::from([self.tex_coords[0][0], self.tex_coords[0][1], 0.0]);
        let tex_corner1 = Vec3::from([self.tex_coords[1][0], self.tex_coords[1][1], 0.0]);
        let tex_corner2 = Vec3::from([self.tex_coords[2][0], self.tex_coords[2][1], 0.0]);
        let tex_u = tex_corner1 - tex_corner0;
        let tex_v = tex_corner2 - tex_corner0;
        let coord = tex_corner0 + hit_record.u * tex_u + hit_record.v * tex_v;
        let x = coord[0];
        let y = coord[1];
        // TODO: other wrapping modes, this is "repeat"
        let x = x.fract();
        let y = y.fract();
        let x = x * (image.width as f32);
        let y = y * (image.height as f32);
        let x = x as usize;
        let y = y as usize;
        (x, y)
    }
}
