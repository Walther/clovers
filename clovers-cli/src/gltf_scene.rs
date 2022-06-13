use clovers::bvhnode::BVHNode;
use clovers::camera::Camera;
use clovers::color::Color;
use clovers::hitable::Hitable;
use clovers::materials::{DiffuseLight, Lambertian, Material, Metal};
use clovers::objects::{Object, QuadInit, TriangleInit};
use clovers::scenes::Scene;
use clovers::textures::{SolidColor, Texture};
use clovers::Vec3;

use super::Opts;
use std::error::Error;
use std::path::Path;

pub(crate) fn initialize(path: &Path, _opts: &Opts) -> Result<Scene, Box<dyn Error>> {
    let background_color = Color::new(0.0, 0.0, 0.0);
    let mut objects: Vec<Object> = Vec::new();
    let mut priority_objects: Vec<Object> = Vec::new();

    // Add a default lamp to have some lighting in the scene
    let light = DiffuseLight::new(Texture::SolidColor(SolidColor::new(Color::new(
        10.0, 10.0, 10.0,
    ))));
    let lamp = Object::Quad(QuadInit {
        q: Vec3::new(-2.0, 4.0, -2.0),
        u: Vec3::new(4.0, 0.0, 0.0),
        v: Vec3::new(0.0, 0.0, 4.0),
        material: Material::DiffuseLight(light),
    });
    objects.push(lamp.clone());
    priority_objects.push(lamp);

    // Add a floor for letting the light bounce a bit
    let floor_mat =
        Material::Lambertian(Lambertian::new(SolidColor::new(Color::new(0.7, 0.7, 0.7))));
    let floor = Object::Quad(QuadInit {
        q: Vec3::new(-100.0, -4.0, -100.0),
        u: Vec3::new(200.0, 0.0, 0.0),
        v: Vec3::new(0.0, 0.0, 200.0),
        material: floor_mat,
    });
    objects.push(floor);

    // Go through the objects in the gltf file
    let (document, buffers, _images) = gltf::import(path)?;
    for scene in document.scenes() {
        for node in scene.nodes() {
            if let Some(mesh) = node.mesh() {
                for primitive in mesh.primitives() {
                    match primitive.mode() {
                        gltf::mesh::Mode::Triangles => {
                            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                            let mut all_positions: Vec<Vec3> = Vec::new();
                            if let Some(iter) = reader.read_positions() {
                                for vertex_position in iter {
                                    all_positions.push(vertex_position.into());
                                }
                            }

                            let gltf_material = primitive.material();
                            let roughness =
                                gltf_material.pbr_metallic_roughness().roughness_factor();
                            let metalness =
                                gltf_material.pbr_metallic_roughness().metallic_factor();

                            let material: Material = if metalness == 0.0 {
                                Material::Lambertian(Lambertian::new(SolidColor::new(
                                    Color::default(),
                                )))
                            } else {
                                Material::Metal(Metal::new(
                                    Texture::SolidColor(SolidColor::new(Color::default())),
                                    roughness,
                                ))
                            };

                            // Go through all the vertex positions in groups of three
                            for (_index, triangle) in all_positions.chunks_exact(3).enumerate() {
                                // NOTE: gLTF format uses absolute vertex positions for all three corners of the triangle
                                // NOTE: internal representation uses triangle origin q, and relative u,v edge vectors
                                let triangle = Object::Triangle(TriangleInit {
                                    q: triangle[0],
                                    u: triangle[1] - triangle[0],
                                    v: triangle[2] - triangle[0],
                                    material,
                                });
                                objects.push(triangle);
                            }
                        }
                        _ => unimplemented!(),
                    }
                }
            }
        }
    }
    dbg!(&objects.len());

    // Various into() transformations
    let objects: Vec<Hitable> = objects.iter().map(|o| o.clone().into()).collect();
    let objects: BVHNode = BVHNode::from_list(objects, 0.0, 1.0);
    let priority_objects: Vec<Hitable> =
        priority_objects.iter().map(|o| o.clone().into()).collect();
    let priority_objects: Hitable =
        Hitable::BVHNode(BVHNode::from_list(priority_objects, 0.0, 1.0));

    // Example hardcoded camera
    let camera = Camera::new(
        Vec3::new(4.0, 0.0, 4.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        40.0,
        1.0,
        0.0,
        10.0,
        0.0,
        1.0,
    );

    Ok(Scene {
        objects,
        camera,
        background_color,
        priority_objects,
    })
}
