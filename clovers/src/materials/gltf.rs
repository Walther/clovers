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
    Float, Vec2, Vec3, PI,
};

use super::{reflect, MaterialTrait, MaterialType, ScatterRecord};

#[derive(Debug, Clone)]
// #[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// GLTF Material wrapper type
pub struct GLTFMaterial {
    material: &'static Material<'static>,
    tex_coords: [[Float; 2]; 3],
    images: &'static [Data],
}

impl Default for GLTFMaterial {
    fn default() -> Self {
        todo!()
    }
}

impl GLTFMaterial {
    /// Initialize a new GLTF material wrapper
    #[must_use]
    pub fn new(
        material: &'static Material,
        tex_coords: [[Float; 2]; 3],
        images: &'static [Data],
    ) -> Self {
        Self {
            material,
            tex_coords,
            images,
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
        let base_color = self.sample_base_color(hit_record);
        let emissive = self.sample_emissive(hit_record);
        let (metalness, roughness) = self.sample_metalness_roughness(hit_record);
        let normal = self.sample_normal(hit_record);

        // TODO: full color model
        let attenuation = emissive + base_color;

        // TODO: better metalness model
        if metalness > 0.0 {
            // TODO: borrowed from metal, should this be different?
            let reflected: Vec3 = reflect(ray.direction.normalize(), normal);
            let direction = reflected + roughness * random_in_unit_sphere(rng);

            Some(ScatterRecord {
                specular_ray: Some(Ray::new(hit_record.position, direction, ray.time)),
                attenuation,
                material_type: MaterialType::Specular,
                pdf_ptr: PDF::ZeroPDF(ZeroPDF::new()),
            })
        } else {
            Some(ScatterRecord {
                specular_ray: None,
                attenuation,
                material_type: MaterialType::Diffuse,
                pdf_ptr: PDF::ZeroPDF(ZeroPDF::new()),
            })
        }
    }

    fn scattering_pdf(
        &self,
        _ray: &Ray,
        hit_record: &HitRecord,
        scattered: &Ray,
        _rng: &mut SmallRng,
    ) -> Option<Float> {
        // TODO: what should this be for GLTF materials?
        // Borrowed from Lambertian
        let cosine = hit_record.normal.dot(&scattered.direction.normalize());
        if cosine < 0.0 {
            None
        } else {
            Some(cosine / PI)
        }
    }
}

impl GLTFMaterial {
    fn sample_base_color(&self, hit_record: &HitRecord) -> Color {
        let base_color_texture = self
            .material
            .pbr_metallic_roughness()
            .base_color_texture()
            .map(|info| &self.images[info.texture().source().index()]);
        // TODO: proper fully correct coloring
        let base_color = match &base_color_texture {
            Some(texture) => {
                let (x, y) = self.sample_texture_coords(hit_record, texture);
                get_color_rgb(texture, x, y)
            }
            None => Color::new(1.0, 1.0, 1.0),
        };
        let base_color_factor: Color = self
            .material
            .pbr_metallic_roughness()
            .base_color_factor()
            .into();

        base_color * base_color_factor
    }

    fn sample_emissive(&self, hit_record: &HitRecord) -> Color {
        let emissive_texture = self
            .material
            .emissive_texture()
            .map(|info| &self.images[info.texture().source().index()]);
        // TODO: proper fully correct coloring
        let emissive = match &emissive_texture {
            Some(texture) => {
                let (x, y) = self.sample_texture_coords(hit_record, texture);
                get_color_rgb(texture, x, y)
            }
            None => Color::new(1.0, 1.0, 1.0),
        };
        let emissive_factor: Color = self.material.emissive_factor().into();

        emissive * emissive_factor
    }

    fn sample_metalness_roughness(&self, hit_record: &HitRecord) -> (Float, Float) {
        let metallic_roughness_texture = self
            .material
            .pbr_metallic_roughness()
            .metallic_roughness_texture()
            .map(|info| &self.images[info.texture().source().index()]);
        let (metalness, roughness) = match &metallic_roughness_texture {
            Some(texture) => {
                let (x, y) = self.sample_texture_coords(hit_record, texture);
                let sampled_color = get_color_rgb(texture, x, y);
                let roughness = sampled_color.g;
                let metalness = sampled_color.b;
                (metalness, roughness)
            }
            None => (1.0, 1.0),
        };
        let metalness = metalness * self.material.pbr_metallic_roughness().metallic_factor();
        let roughness = roughness * self.material.pbr_metallic_roughness().roughness_factor();
        (metalness, roughness)
    }

    fn sample_normal(&self, hit_record: &HitRecord) -> Vec3 {
        let normal_texture = self
            .material
            .normal_texture()
            .map(|info| &self.images[info.texture().source().index()]);
        let texture_normal = match &normal_texture {
            Some(texture) => {
                let (x, y) = self.sample_texture_coords(hit_record, texture);
                let sampled_color = get_color_rgb(texture, x, y);
                // Convert from Color to Vec 0..1, scale and move to -1..1
                let normal: Vec3 = sampled_color.into();
                let normal = normal * 2.0 - Vec3::new(1.0, 1.0, 1.0);
                normal.normalize()
            }
            None => hit_record.normal,
        };

        // TODO: this is wrong, take into account the tangent space
        texture_normal * 1.0
        // fallback to triangle normal, no details
        // hit_record.normal
    }

    /// Find the correct texture coordinates in pixel space
    fn sample_texture_coords(&self, hit_record: &HitRecord, image: &Data) -> (usize, usize) {
        // Full triangle coordinates on the full texture file
        let tex_corner0: Vec2 = Vec2::from(self.tex_coords[0]);
        let tex_corner1: Vec2 = Vec2::from(self.tex_coords[1]);
        let tex_corner2: Vec2 = Vec2::from(self.tex_coords[2]);
        // Side vectors on the texture triangle
        let tex_u: Vec2 = tex_corner1 - tex_corner0;
        let tex_v: Vec2 = tex_corner2 - tex_corner0;
        // Specific surface space coordinate for hit point
        let coord: Vec2 = tex_corner0 + hit_record.u * tex_u + hit_record.v * tex_v;
        let x = coord[0];
        let y = coord[1];
        // TODO: other wrapping modes, this is "repeat"
        let x = if x < 0.0 { 1.0 + x.fract() } else { x.fract() };
        let y = if y < 0.0 { 1.0 + y.fract() } else { y.fract() };
        // Pixel space coordinates on the texture
        let x = x * (image.width as Float);
        let y = y * ((image.height - 1) as Float); // TODO: fix overflows better

        // Cast
        let x = x.floor() as usize;
        let y = y.floor() as usize;
        (x, y)
    }
}

/// Given a reference to a texture and pixel space coordinates, returns the color at that pixel
fn get_color_rgb(texture: &&Data, x: usize, y: usize) -> Color {
    let index = match texture.format {
        gltf::image::Format::R8G8B8 => 3 * (x + texture.width as usize * y),
        gltf::image::Format::R8G8B8A8 => 4 * (x + texture.width as usize * y),
        _ => todo!("Unsupported gltf::image::Format"),
    };
    let r = texture.pixels[index];
    let g = texture.pixels[index + 1];
    let b = texture.pixels[index + 2];
    Color::from([r, g, b])
}
