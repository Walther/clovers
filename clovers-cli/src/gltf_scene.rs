use clovers::bvhnode::BVHNode;
use clovers::camera::Camera;
use clovers::color::Color;
use clovers::hitable::Hitable;
use clovers::materials::{DiffuseLight, Lambertian, Material};
use clovers::objects::gltf::GLTFTriangle;
use clovers::objects::{Object, QuadInit};
use clovers::scenes::Scene;
use clovers::textures::{SolidColor, Texture};
use clovers::{Float, Vec3};
use gltf::{Mesh, Node};
use tracing::debug;

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

    // Switch the type to Hitable to be able to add GLTF Hitables
    let mut hitables: Vec<Hitable> = objects.iter().map(|o| o.clone().into()).collect();

    // Go through the objects in the gltf file
    let (document, buffers, images) = gltf::import(path)?;
    let images: &'static Vec<gltf::image::Data> = Box::leak(Box::new(images));

    for scene in document.scenes() {
        debug!("found scene");
        for node in scene.nodes() {
            debug!("found node");
            parse_node(node, &mut hitables, &buffers, images);
        }
    }
    debug!("hitable count: {}", &hitables.len());

    // Create BVHNodes
    let objects: BVHNode = BVHNode::from_list(hitables, 0.0, 1.0);
    let priority_objects: Vec<Hitable> =
        priority_objects.iter().map(|o| o.clone().into()).collect();
    let priority_objects: Hitable =
        Hitable::BVHNode(BVHNode::from_list(priority_objects, 0.0, 1.0));

    // Example hardcoded camera
    let camera = Camera::new(
        Vec3::new(2.0, 2.0, 2.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        60.0,
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

fn parse_node(
    node: Node,
    objects: &mut Vec<Hitable>,
    buffers: &Vec<gltf::buffer::Data>,
    images: &'static Vec<gltf::image::Data>,
) {
    // Handle direct meshes
    if let Some(mesh) = node.mesh() {
        parse_mesh(mesh, objects, buffers, images);
    }
    // Handle nesting
    for child in node.children() {
        parse_node(child, objects, buffers, images);
    }
}

fn parse_mesh(
    mesh: Mesh,
    objects: &mut Vec<Hitable>,
    buffers: &[gltf::buffer::Data],
    images: &'static [gltf::image::Data],
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
                    let material = primitive.material();
                    let coordset = material
                        .pbr_metallic_roughness()
                        .base_color_texture()
                        .unwrap()
                        .tex_coord();
                    let all_tex_coords: Vec<[Float; 2]> = reader
                        .read_tex_coords(coordset)
                        .unwrap()
                        .into_f32()
                        .collect();

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
                        let gltf_triangle =
                            GLTFTriangle::new(triangle, tex_coords, &material, images);
                        trianglelist.push(Hitable::GLTFTriangle(gltf_triangle));
                        i += 3;
                    }
                }

                let bvh: BVHNode = BVHNode::from_list(trianglelist, 0.0, 1.0);
                objects.push(Hitable::BVHNode(bvh));
            }
            _ => unimplemented!(),
        }
    }
}
