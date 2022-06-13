use clovers::bvhnode::BVHNode;
use clovers::camera::Camera;
use clovers::color::Color;
use clovers::hitable::Hitable;
use clovers::materials::{DiffuseLight, Material};
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
                            for x in all_positions.chunks_exact(3) {
                                let material: Material = Default::default();
                                // NOTE: gLTF format uses absolute vertex positions for all three corners of the triangle
                                // NOTE: internal representation uses triangle origin q, and relative u,v edge vectors
                                let triangle = Object::Triangle(TriangleInit {
                                    q: x[0],
                                    u: x[1] - x[0],
                                    v: x[2] - x[0],
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
        Vec3::new(0.0, 0.0, 6.0),
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
