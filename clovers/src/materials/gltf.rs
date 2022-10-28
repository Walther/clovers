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
    #[cfg_attr(feature = "serde-derive", serde(skip))]
    normal_texture: Option<&'static Data>,
    #[cfg_attr(feature = "serde-derive", serde(skip))]
    metallic_roughness_texture: Option<&'static Data>,
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

        let normal_texture = if let Some(info) = material.normal_texture() {
            let index = info.texture().index();
            Some(&images[index])
        } else {
            None
        };

        let metallic_roughness_texture = if let Some(info) = material
            .pbr_metallic_roughness()
            .metallic_roughness_texture()
        {
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
            normal_texture,
            metallic_roughness_texture,
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
        let texture_normal = match &self.normal_texture {
            Some(normal_texture) => {
                match normal_texture.format {
                    // TODO: deduplicate
                    gltf::image::Format::R8G8B8 => {
                        let (x, y) = self.get_texture_coords(hit_record, normal_texture);
                        let index = 3 * (x + normal_texture.width as usize * y);
                        let r = normal_texture.pixels[index];
                        let g = normal_texture.pixels[index + 1];
                        let b = normal_texture.pixels[index + 2];
                        // Converted to color 0..1
                        let color = Color::from([r, g, b]);
                        let normal: Vec3 = color.into();
                        // Scaled and moved to -1..1
                        let normal = normal * 2.0 - Vec3::new(1.0, 1.0, 1.0);
                        normal.normalize()
                    }
                    gltf::image::Format::R8G8B8A8 => {
                        let (x, y) = self.get_texture_coords(hit_record, normal_texture);
                        let index = 4 * (x + normal_texture.width as usize * y);
                        let r = normal_texture.pixels[index];
                        let g = normal_texture.pixels[index + 1];
                        let b = normal_texture.pixels[index + 2];
                        // Converted to color 0..1
                        let color = Color::from([r, g, b]);
                        // Scaled and moved to -1..1
                        let normal: Vec3 = color.into();
                        let normal = normal * 2.0 - Vec3::new(1.0, 1.0, 1.0);
                        normal.normalize()
                    }
                    _ => todo!(),
                }
            }
            None => hit_record.normal,
        };
        // TODO: this is wrong, take into account the tangent space
        let normal = texture_normal;

        let (metalness, roughness) = match &self.metallic_roughness_texture {
            Some(metallic_roughness_texture) => {
                match metallic_roughness_texture.format {
                    // TODO: deduplicate
                    gltf::image::Format::R8G8B8 => {
                        let (x, y) =
                            self.get_texture_coords(hit_record, metallic_roughness_texture);
                        let index = 3 * (x + metallic_roughness_texture.width as usize * y);
                        let r = metallic_roughness_texture.pixels[index];
                        let g = metallic_roughness_texture.pixels[index + 1];
                        let b = metallic_roughness_texture.pixels[index + 2];
                        // Converted to color 0..1
                        let color = Color::from([r, g, b]);
                        let roughness = color.g;
                        let metalness = color.b;
                        (metalness, roughness)
                    }
                    gltf::image::Format::R8G8B8A8 => {
                        let (x, y) =
                            self.get_texture_coords(hit_record, metallic_roughness_texture);
                        let index = 4 * (x + metallic_roughness_texture.width as usize * y);
                        let r = metallic_roughness_texture.pixels[index];
                        let g = metallic_roughness_texture.pixels[index + 1];
                        let b = metallic_roughness_texture.pixels[index + 2];
                        // Converted to color 0..1
                        let color = Color::from([r, g, b]);
                        let roughness = color.g;
                        let metalness = color.b;
                        (metalness, roughness)
                    }
                    _ => todo!(),
                }
            }
            None => (1.0, 1.0),
        };
        let _metalness = metalness * self.metallic_factor;
        let roughness = roughness * self.roughness_factor;

        // TODO: borrowed from metal, should this be different?
        let reflected: Vec3 = reflect(ray.direction.normalize(), normal);
        let direction = reflected + roughness * random_in_unit_sphere(rng);

        // TODO: proper fully correct coloring
        let base_color = match &self.base_color_texture {
            Some(base_color_texture) => {
                match base_color_texture.format {
                    // TODO: deduplicate
                    gltf::image::Format::R8G8B8 => {
                        let (x, y) = self.get_texture_coords(hit_record, base_color_texture);
                        let index = 3 * (x + base_color_texture.width as usize * y);

                        let r = base_color_texture.pixels[index];
                        let g = base_color_texture.pixels[index + 1];
                        let b = base_color_texture.pixels[index + 2];
                        let color = Color::from([r, g, b]);
                        self.base_color_factor * color
                    }
                    gltf::image::Format::R8G8B8A8 => {
                        let (x, y) = self.get_texture_coords(hit_record, base_color_texture);
                        let index = 4 * (x + base_color_texture.width as usize * y);

                        let r = base_color_texture.pixels[index];
                        let g = base_color_texture.pixels[index + 1];
                        let b = base_color_texture.pixels[index + 2];
                        let color = Color::from([r, g, b]);
                        self.base_color_factor * color
                    }
                    _ => todo!(),
                }
            }
            None => self.base_color_factor,
        };

        // Combine
        let attenuation = base_color;

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
