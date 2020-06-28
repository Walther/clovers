use super::Scene;
use crate::{
    camera::Camera,
    color::Color,
    hitable::{Hitable, HitableList},
    materials::{
        dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian, metal::Metal,
        Material,
    },
    objects::{
        boxy::Boxy, constant_medium::ConstantMedium, moving_sphere::MovingSphere, rotate::RotateY,
        sphere::Sphere, translate::Translate,
    },
    perlin::Perlin,
    rect::{XYRect, XZRect, YZRect},
    textures::{noise_texture::NoiseTexture, solid_color::SolidColor, Texture},
    Float, Vec3, HEIGHT, WIDTH,
};
use rand::prelude::*;
use std::sync::Arc;
pub fn load(rng: ThreadRng) -> Scene {
    let time_0: Float = 0.0;
    let time_1: Float = 1.0;
    let mut world: HitableList = HitableList::new();

    // blue middle sphere
    world.hitables.push(Arc::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
            0.1, 0.2, 0.5,
        ))))),
    )));

    // large green ground sphere
    world.hitables.push(Arc::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
            0.8, 0.8, 0.0,
        ))))),
    )));

    // metal sphere
    world.hitables.push(Arc::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Arc::new(Metal::new(
            Arc::new(SolidColor::new(Color::new(0.8, 0.6, 0.2))),
            0.0,
        )),
    )));
    // glass sphere
    world.hitables.push(Arc::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Arc::new(Dielectric::new(1.5)),
    )));

    let camera_position: Vec3 = Vec3::new(0.0, 0.0, 5.0);
    let camera_target: Vec3 = Vec3::new(0.0, 0.0, -1.0);
    let camera_up: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let fov: Float = 25.0;
    let aspect_ratio: Float = WIDTH as Float / HEIGHT as Float;
    let aperture: Float = 0.0;
    let focus_distance: Float = 1.0;
    let camera = Camera::new(
        camera_position,
        camera_target,
        camera_up,
        fov,
        aspect_ratio,
        aperture,
        focus_distance,
        time_0,
        time_1,
    );

    let background: Color = Color::new(0.7, 0.7, 0.7); // TODO: gradient from first book

    Scene::new(world, camera, time_0, time_1, background, rng)
}
