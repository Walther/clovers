//! Wrapper for GLTF materials.

#![allow(clippy::pedantic)]

#[cfg(feature = "gl_tf")]
use gltf::image::Data;
use nalgebra::Unit;
use palette::{
    chromatic_adaptation::AdaptInto, convert::IntoColorUnclamped, white_point::E, LinSrgb, Srgb,
    Srgba, Xyz,
};
use rand::rngs::SmallRng;

use crate::{
    pdf::{ZeroPDF, PDF},
    random::random_unit_vector,
    ray::Ray,
    spectrum::spectral_power,
    textures::TextureTrait,
    wavelength::Wavelength,
    Direction, Float, HitRecord, Vec2, Vec3, Vec4, PI,
};

use super::{reflect, MaterialTrait, MaterialType, ScatterRecord};

#[derive(Debug, Clone)]
// #[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// GLTF Material wrapper type
pub struct GLTFMaterial {
    material: &'static gltf::Material<'static>,
    tex_coords: [[Float; 2]; 3],
    images: &'static [Data],
    tangents: Option<[Vec3; 3]>,
    normals: Option<[Vec3; 3]>,
    bitangents: Option<[Vec3; 3]>,
}

impl GLTFMaterial {
    /// Initialize a new GLTF material wrapper
    #[must_use]
    pub fn new(
        material: &'static gltf::Material,
        tex_coords: [[Float; 2]; 3],
        normals: Option<[[Float; 3]; 3]>,
        tangents: Option<[[Float; 4]; 3]>,
        images: &'static [Data],
    ) -> Self {
        let normals: Option<[Vec3; 3]> = normals.map(|ns| ns.map(Vec3::from));
        let tangents: Option<[Vec4; 3]> = tangents.map(|ns| ns.map(Vec4::from));
        let ws: Option<[Float; 3]> = tangents.map(|ts| ts.map(|t| t[3]));
        let tangents: Option<[Vec3; 3]> = tangents.map(|ts| ts.map(|t| Vec4::xyz(&t)));
        // TODO: fix this horrendous mess
        let bitangents = if let Some(normals) = normals {
            if let Some(tangents) = tangents {
                ws.map(|ws| {
                    [
                        normals[0].cross(&tangents[0]) * ws[0],
                        normals[1].cross(&tangents[1]) * ws[1],
                        normals[2].cross(&tangents[2]) * ws[2],
                    ]
                })
            } else {
                None
            }
        } else {
            None
        };

        Self {
            material,
            tex_coords,
            normals,
            tangents,
            bitangents,
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
        let (metalness, roughness) = self.sample_metalness_roughness(hit_record);
        let normal: Direction = self.sample_normal(hit_record);

        // TODO: correct scatter model
        if metalness > 0.0 {
            // TODO: borrowed from metal, should this be different?
            let reflected: Direction = reflect(ray.direction, normal);
            let direction = *reflected + roughness * *random_unit_vector(rng);
            let direction = Unit::new_normalize(direction);

            Some(ScatterRecord {
                specular_ray: Some(Ray {
                    origin: hit_record.position,
                    direction,
                    time: ray.time,
                    wavelength: ray.wavelength,
                }),
                material_type: MaterialType::Specular,
                pdf_ptr: PDF::ZeroPDF(ZeroPDF::new()),
            })
        } else {
            Some(ScatterRecord {
                specular_ray: None,
                material_type: MaterialType::Diffuse,
                pdf_ptr: PDF::ZeroPDF(ZeroPDF::new()),
            })
        }
    }

    fn scattering_pdf(&self, hit_record: &HitRecord, scattered: &Ray) -> Option<Float> {
        // TODO: what should this be for GLTF materials?
        // Borrowed from Lambertian
        let cosine = hit_record.normal.dot(&scattered.direction.normalize());
        if cosine < 0.0 {
            None
        } else {
            Some(cosine / PI)
        }
    }

    fn emit(&self, ray: &Ray, wavelength: Wavelength, hit_record: &HitRecord) -> Float {
        self.emit(ray, wavelength, hit_record)
    }

    fn color(&self, ray: &Ray, wavelength: Wavelength, hit_record: &HitRecord) -> Float {
        self.color(ray, wavelength, hit_record)
    }
}

impl GLTFMaterial {
    #[must_use]
    fn color(&self, _ray: &Ray, wavelength: Wavelength, hit_record: &HitRecord) -> Float {
        let base_color: LinSrgb = self.sample_base_color(hit_record);
        let occlusion: Float = self.sample_occlusion(hit_record);
        // TODO: full color model
        let attenuation: LinSrgb = base_color * occlusion;
        let attenuation: Xyz<E> = attenuation.adapt_into();
        spectral_power(attenuation, wavelength)
    }

    #[must_use]
    fn emit(&self, _ray: &Ray, wavelength: Wavelength, hit_record: &HitRecord) -> Float {
        // TODO: full color model
        let emission: Xyz<E> = self.sample_emissive(hit_record).adapt_into();
        spectral_power(emission, wavelength)
    }

    fn sample_base_color(&self, hit_record: &HitRecord) -> LinSrgb {
        let base_color_texture = self
            .material
            .pbr_metallic_roughness()
            .base_color_texture()
            .map(|info| &self.images[info.texture().source().index()]);
        // TODO: proper fully correct coloring
        let base_color = match &base_color_texture {
            Some(texture) => {
                let (x, y) = self.sample_texture_coords(hit_record, texture);
                get_color_srgb(texture, x, y)
            }
            None => Srgb::new(1.0, 1.0, 1.0),
        };
        let base_color_factor: Srgba = self
            .material
            .pbr_metallic_roughness()
            .base_color_factor()
            .into();
        let base_color_factor: Srgb = base_color_factor.into_color_unclamped();

        (base_color * base_color_factor).into_color_unclamped()
    }

    fn sample_emissive(&self, hit_record: &HitRecord) -> Srgb {
        let emissive_texture = self
            .material
            .emissive_texture()
            .map(|info| &self.images[info.texture().source().index()]);
        // TODO: proper fully correct coloring
        match &emissive_texture {
            Some(texture) => {
                let (x, y) = self.sample_texture_coords(hit_record, texture);
                let emissive: Srgb = get_color_srgb(texture, x, y);
                let factor: [Float; 3] = self.material.emissive_factor();

                Srgb::new(
                    emissive.red * factor[0],
                    emissive.green * factor[0],
                    emissive.blue * factor[0],
                )
            }
            None => Srgb::new(0.0, 0.0, 0.0),
        }
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
                let sampled_color = get_color_linsrgb(texture, x, y);
                let roughness = sampled_color.green;
                let metalness = sampled_color.blue;
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
                let sampled_color = get_color_linsrgb(texture, x, y);
                // Only the red channel is taken into account
                sampled_color.red
            }
            None => 1.0,
        }
    }

    fn sample_normal(&self, hit_record: &HitRecord) -> Direction {
        let Some(normals) = self.normals else {
            // If we don't have normals, early return with the triangle normal
            return hit_record.normal;
        };

        let Some(tangents) = self.tangents else {
            // If we don't have tangents, early return with the triangle normal
            // TODO: compute normals here or at construction time as per gltf spec
            return hit_record.normal;
        };

        let Some(bitangents) = self.bitangents else {
            return hit_record.normal;
        };

        let normal_texture = self
            .material
            .normal_texture()
            .map(|info| &self.images[info.texture().source().index()]);
        let texture_normal = match &normal_texture {
            Some(texture) => {
                let (x, y) = self.sample_texture_coords(hit_record, texture);
                let sampled_color = get_color_linsrgb(texture, x, y);
                // Convert from Color to Vec 0..1, scale and move to -1..1
                let (r, g, b) = sampled_color.into_components();
                let normal: Vec3 = Vec3::new(r, g, b) * 2.0 - Vec3::new(1.0, 1.0, 1.0);
                normal.normalize()
            }
            // If we don't have a normal texture, early return with the triangle normal
            None => return hit_record.normal,
        };

        // Barycentric coordinates and interpolation on the triangle surface
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
        Unit::new_normalize(matrix * texture_normal)
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

// TODO: better ideas?
impl TextureTrait for &GLTFMaterial {
    fn color(&self, ray: &Ray, wavelength: Wavelength, hit_record: &HitRecord) -> Float {
        GLTFMaterial::color(self, ray, wavelength, hit_record)
    }

    fn emit(&self, ray: &Ray, wavelength: Wavelength, hit_record: &HitRecord) -> Float {
        GLTFMaterial::emit(self, ray, wavelength, hit_record)
    }
}

/// Given a reference to a texture and pixel space coordinates, returns the raw byte triple `(r,g,b)`
fn sample_texture_raw(texture: &&Data, x: usize, y: usize) -> (u8, u8, u8) {
    let index = match texture.format {
        gltf::image::Format::R8G8B8 => 3 * (x + texture.width as usize * y),
        gltf::image::Format::R8G8B8A8 => 4 * (x + texture.width as usize * y),
        _ => todo!("Unsupported gltf::image::Format"),
    };
    let r = texture.pixels[index];
    let g = texture.pixels[index + 1];
    let b = texture.pixels[index + 2];
    (r, g, b)
}

/// Given a reference to a texture and pixel space coordinates, returns the color at that pixel, sRGB with gamma.
fn get_color_srgb(texture: &&Data, x: usize, y: usize) -> Srgb {
    let (r, g, b) = sample_texture_raw(texture, x, y);
    let color: Srgb<u8> = Srgb::from_components((r, g, b));
    color.into_format()
}

/// Given a reference to a texture and pixel space coordinates, returns the color at that pixel, linear sRGB
fn get_color_linsrgb(texture: &&Data, x: usize, y: usize) -> LinSrgb {
    let (r, g, b) = sample_texture_raw(texture, x, y);
    let color: LinSrgb<u8> = LinSrgb::from_components((r, g, b));
    color.into_format()
}
