//! GLTF format support for the renderer

use alloc::string::String;
use alloc::vec::Vec;
#[cfg(feature = "gl_tf")]
use gltf::{image::Data, Mesh, Node};
use nalgebra::Unit;
use rand::rngs::SmallRng;
#[cfg(feature = "traces")]
use tracing::debug;

use crate::{
    aabb::AABB,
    bvh::build::utils::vec_bounding_box,
    hitable::{get_orientation, Hitable, HitableTrait},
    interval::Interval,
    materials::gltf::GLTFMaterial,
    ray::Ray,
    wavelength::Wavelength,
    Direction, Float, HitRecord, Position, Vec3, EPSILON_RECT_THICKNESS, EPSILON_SHADOW_ACNE,
};

/// GLTF initialization structure
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
pub struct GLTFInit {
    /// Used for multiple importance sampling
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub priority: bool,
    /// Path of the .gltf file
    pub path: String,
}

impl From<GLTFInit> for Vec<Hitable<'static>> {
    fn from(gltf: GLTFInit) -> Self {
        let mut hitables: Vec<Hitable> = Vec::new();

        // Go through the objects in the gltf file
        let (document, buffers, images) = gltf::import(gltf.path).unwrap();
        let document: &'static gltf::Document = Box::leak(Box::new(document));
        let images: &'static Vec<Data> = Box::leak(Box::new(images));
        let materials: &'static Vec<gltf::Material> =
            Box::leak(Box::new(document.materials().collect()));

        for scene in document.scenes() {
            debug!("found scene");
            for node in scene.nodes() {
                debug!("found node");
                parse_node(&node, &mut hitables, &buffers, materials, images);
            }
        }
        debug!("hitable count: {}", &hitables.len());

        hitables
    }
}

/// Internal GLTF object representation after initialization.
#[derive(Debug, Clone)]
pub struct GLTF<'scene> {
    /// Hitables of the `GLTF` object. Most likely a list of `GLTFTriangle`s.
    pub hitables: Vec<Hitable<'scene>>,
    /// Axis-aligned bounding box of the object
    pub aabb: AABB,
}

impl GLTF<'_> {
    #[must_use]
    /// Create a new STL object with the given initialization parameters.
    ///
    /// # Panics
    /// This method may panic if no finite bounding box can be created for the given `hitables`.
    pub fn new(gltf_init: GLTFInit) -> Self {
        let hitables: Vec<Hitable> = gltf_init.into();
        let aabb = vec_bounding_box(&hitables).expect("No bounding box for hitables");

        GLTF { hitables, aabb }
    }
}

fn parse_node(
    node: &Node,
    hitables: &mut Vec<Hitable<'static>>,
    buffers: &Vec<gltf::buffer::Data>,
    materials: &'static Vec<gltf::Material>,
    images: &'static Vec<Data>,
) {
    // Handle direct meshes
    if let Some(mesh) = node.mesh() {
        parse_mesh(&mesh, hitables, buffers, materials, images);
    }
    // Handle nesting
    for child in node.children() {
        parse_node(&child, hitables, buffers, materials, images);
    }
}

fn parse_mesh(
    mesh: &Mesh,
    hitables: &mut Vec<Hitable<'static>>,
    buffers: &[gltf::buffer::Data],
    materials: &'static [gltf::Material],
    images: &'static [Data],
) {
    debug!("found mesh");
    for primitive in mesh.primitives() {
        debug!("found primitive");
        match primitive.mode() {
            gltf::mesh::Mode::Triangles => {
                let mut trianglelist: Vec<Hitable> = Vec::new();

                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                let mut all_positions: Vec<Vec3> = Vec::new();
                if let Some(iter) = reader.read_positions() {
                    for vertex_position in iter {
                        all_positions.push(vertex_position.into());
                    }
                }

                // Note that in the GLTF format the same positions can be re-used for multiple triangles, as a sort of a compression method
                // Read the indices array in order to assemble triangles from positions
                if let Some(accessor) = reader.read_indices() {
                    let accessor = accessor.into_u32();
                    let len = accessor.len();
                    let indices: Vec<u32> = accessor.collect();
                    let indices: Vec<usize> = indices.iter().map(|&x| x as usize).collect();
                    let mut i = 0;
                    let material_index = primitive.material().index().unwrap();
                    let material = &materials[material_index];

                    let coordset = match material.pbr_metallic_roughness().base_color_texture() {
                        Some(texture) => texture.tex_coord(),
                        None => 0,
                    };
                    let all_tex_coords: Vec<[Float; 2]> = reader
                        .read_tex_coords(coordset)
                        .unwrap()
                        .into_f32()
                        .collect();
                    let all_normals: Option<Vec<_>> = reader.read_normals().map(Iterator::collect);
                    let all_tangents: Option<Vec<_>> =
                        reader.read_tangents().map(Iterator::collect);

                    while i < len {
                        let triangle = [
                            all_positions[indices[i]],
                            all_positions[indices[i + 1]],
                            all_positions[indices[i + 2]],
                        ];
                        let tex_coords = [
                            all_tex_coords[indices[i]],
                            all_tex_coords[indices[i + 1]],
                            all_tex_coords[indices[i + 2]],
                        ];
                        let normals = all_normals.as_ref().map(|normals| {
                            [
                                normals[indices[i]],
                                normals[indices[i + 1]],
                                normals[indices[i + 2]],
                            ]
                        });
                        let tangents = all_tangents.as_ref().map(|tangents| {
                            [
                                tangents[indices[i]],
                                tangents[indices[i + 1]],
                                tangents[indices[i + 2]],
                            ]
                        });

                        // TODO: don't leak memory
                        let material: &'static GLTFMaterial = Box::leak(Box::new(
                            GLTFMaterial::new(material, tex_coords, normals, tangents, images),
                        ));

                        let gltf_triangle = GLTFTriangle::new(triangle, material);
                        trianglelist.push(Hitable::GLTFTriangle(gltf_triangle));
                        i += 3;
                    }
                }

                hitables.append(&mut trianglelist);
            }
            _ => unimplemented!(),
        }
    }
}

/// Internal GLTF object representation after initialization.
#[derive(Debug, Clone)]
pub struct GLTFTriangle<'scene> {
    /// Axis-aligned bounding box of the object
    pub aabb: AABB,
    /// Material of the object
    pub material: &'scene GLTFMaterial,
    q: Vec3,
    u: Vec3,
    v: Vec3,
    d: Float,
    w: Vec3,
    area: Float,
    normal: Direction,
}

impl<'scene> GLTFTriangle<'scene> {
    #[must_use]
    #[allow(clippy::many_single_char_names)]
    /// Initialize a new GLTF object
    pub fn new(triangle: [Vec3; 3], material: &'scene GLTFMaterial) -> Self {
        // TODO: mostly adapted from Triangle, verify correctness!

        let [a, b, c] = triangle;
        let interval_x = Interval::new(a[0].min(b[0]).min(c[0]), a[0].max(b[0]).max(c[0]));
        let interval_y = Interval::new(a[1].min(b[1]).min(c[1]), a[1].max(b[1]).max(c[1]));
        let interval_z = Interval::new(a[2].min(b[2]).min(c[2]), a[2].max(b[2]).max(c[2]));
        let mut aabb: AABB = AABB::new(interval_x, interval_y, interval_z);
        aabb.pad();

        // TODO: Check orientation and make into a corner + edge vectors triangle
        let q = a;
        let u = b - q;
        let v = c - q;

        let n: Vec3 = u.cross(&v);
        let normal = Unit::new_normalize(n);
        // TODO: what is this?
        let d = -(normal.dot(&q));
        // TODO: what is this?
        let w: Vec3 = n / n.dot(&n);
        // Compared to quad, triangle has half the area
        let area = n.magnitude() / 2.0;

        GLTFTriangle {
            aabb,
            material,
            q,
            u,
            v,
            d,
            w,
            area,
            normal,
        }
    }
}

impl HitableTrait for GLTFTriangle<'_> {
    fn hit(
        &self,
        ray: &Ray,
        distance_min: Float,
        distance_max: Float,
        _rng: &mut SmallRng,
    ) -> Option<HitRecord> {
        // TODO: mostly adapted from Triangle, verify correctness!

        let denom = self.normal.dot(&ray.direction);

        // No hit if the ray is parallel to the plane.
        if denom.abs() < EPSILON_RECT_THICKNESS {
            return None;
        }

        // Return false if the hit point parameter t is outside the ray interval
        let t = (-self.d - self.normal.dot(&ray.origin)) / denom;
        if t < distance_min || t > distance_max {
            return None;
        }

        // Determine the hit point lies within the planar shape using its plane coordinates.
        let intersection: Vec3 = ray.evaluate(t);
        let planar_hitpt_vector: Vec3 = intersection - self.q;
        let alpha: Float = self.w.dot(&planar_hitpt_vector.cross(&self.v));
        let beta: Float = self.w.dot(&self.u.cross(&planar_hitpt_vector));

        // Do we hit a coordinate within the surface of the plane?
        if !hit_ab(alpha, beta) {
            return None;
        }

        // Ray hits the 2D shape; set the rest of the hit record and return

        let (front_face, normal) = get_orientation(ray, self.normal);

        Some(HitRecord {
            distance: t,
            position: intersection,
            normal,
            u: alpha,
            v: beta,
            material: self.material,
            front_face,
        })
    }

    fn aabb(&self) -> Option<&AABB> {
        Some(&self.aabb)
    }

    fn pdf_value(
        &self,
        origin: Position,
        direction: Direction,
        wavelength: Wavelength,
        time: Float,
        rng: &mut SmallRng,
    ) -> Float {
        let ray = Ray {
            origin,
            direction,
            time,
            wavelength,
        };
        // TODO: this is from quad and not updated!
        match self.hit(&ray, EPSILON_SHADOW_ACNE, Float::INFINITY, rng) {
            Some(hit_record) => {
                let distance_squared =
                    hit_record.distance * hit_record.distance * direction.norm_squared();
                let cosine = direction.dot(&hit_record.normal).abs() / direction.magnitude();

                distance_squared / (cosine * self.area)
            }
            None => 0.0,
        }
    }

    // TODO: correctness
    fn centroid(&self) -> Position {
        self.q + (self.u / 4.0) + (self.v / 4.0)
    }
}

#[must_use]
fn hit_ab(a: Float, b: Float) -> bool {
    // Given the hit point in plane coordinates, return false if it is outside the
    // primitive, otherwise return true.
    // Triangle: a+b must be <=1.0
    (0.0..=1.0).contains(&a) && (0.0..=1.0).contains(&b) && (a + b <= 1.0)
}
