//! Wrapper for GLTF materials.

#![allow(clippy::pedantic)]

#[cfg(feature = "gl_tf")]
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

#[derive(Debug, Copy, Clone)]
// #[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// GLTF Material wrapper type
pub struct GLTFMaterial {
    material: &'static Material<'static>,
    tex_coords: [[Float; 2]; 3],
    images: &'static [Data],
    tangents: Option<[[Float; 4]; 3]>,
    normals: Option<[[Float; 3]; 3]>,
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
        normals: Option<[[Float; 3]; 3]>,
        tangents: Option<[[Float; 4]; 3]>,
        images: &'static [Data],
    ) -> Self {
        Self {
            material,
            tex_coords,
            normals,
            tangents,
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
        let occlusion = self.sample_occlusion(hit_record);

        // TODO: full color model
        let attenuation = emissive + base_color * occlusion;

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

    fn sample_occlusion(&self, hit_record: &HitRecord) -> Float {
        let occlusion_texture = self
            .material
            .occlusion_texture()
            .map(|info| &self.images[info.texture().source().index()]);

        match &occlusion_texture {
            Some(texture) => {
                let (x, y) = self.sample_texture_coords(hit_record, texture);
                let sampled_color = get_color_rgb(texture, x, y);
                // Only the red channel is taken into account
                sampled_color.r
            }
            None => 1.0,
        }
    }

    fn sample_normal(&self, hit_record: &HitRecord) -> Vec3 {
        let normals = match self.normals {
            Some(ns) => ns.map(Vec3::from),
            None => {
                // If we don't have normals, early return with the triangle normal
                return hit_record.normal;
            }
        };

        let tangents = match self.tangents {
            Some(ts) => ts.map(|t| Vec3::new(t[0], t[1], t[2])),
            None => {
                // If we don't have tangents, early return with the triangle normal
                // TODO: compute normals here or at construction time as per gltf spec
                return hit_record.normal;
            }
        };
        let ws = self.tangents.unwrap().map(|t| t[3]);

        let normal_texture = self
            .material
            .normal_texture()
            .map(|info| &self.images[info.texture().source().index()]);
        let texture_normal = match &normal_texture {
            Some(texture) => {
                let (x, y) = self.sample_texture_coords(hit_record, texture);
                let sampled_color = get_color_rgb(texture, x, y);
                // Convert from Color to Vec 0..1, scale and move to -1..1
                let normal: Vec3 = Vec3::from(sampled_color) * 2.0 - Vec3::new(1.0, 1.0, 1.0);
                normal.normalize()
            }
            // If we don't have a normal texture, early return with the triangle normal
            None => return hit_record.normal,
        };

        // Barycentric coordinates and interpolation on the triangle surface
        let bitangents: [Vec3; 3] = [
            normals[0].cross(&tangents[0]) * ws[0],
            normals[1].cross(&tangents[1]) * ws[1],
            normals[2].cross(&tangents[2]) * ws[2],
        ];
        let normal = (hit_record.u * normals[1]
            + hit_record.v * normals[2]
            + (1.0 - hit_record.u - hit_record.v) * normals[0])
            .normalize();
        let tangent = (hit_record.u * tangents[1]
            + hit_record.v * tangents[2]
            + (1.0 - hit_record.u - hit_record.v) * tangents[0])
            .normalize();
        let bitangent = (hit_record.u * bitangents[1]
            + hit_record.v * bitangents[2]
            + (1.0 - hit_record.u - hit_record.v) * bitangents[0])
            .normalize();

        let matrix: nalgebra::Matrix3<Float> =
            nalgebra::Matrix3::from_columns(&[tangent, bitangent, normal]);

        // Transform the texture normal from tangent space to world space
        (matrix * texture_normal).normalize()
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
