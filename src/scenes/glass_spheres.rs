use super::Scene;
use crate::{
    camera::Camera,
    color::Color,
    hitable::HitableList,
    materials::{Dielectric, Lambertian, Metal},
    objects::sphere::Sphere,
    textures::SolidColor,
    Float, Vec3,
};
use rand::prelude::*;

pub fn load(width: u32, height: u32, rng: ThreadRng) -> Scene {
    let time_0: Float = 0.0;
    let time_1: Float = 1.0;
    let mut world: HitableList = HitableList::new();

    // blue middle sphere
    world.add(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Lambertian::new(SolidColor::new(Color::new(0.1, 0.2, 0.5))),
    ));

    // large green ground sphere
    world.add(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Lambertian::new(SolidColor::new(Color::new(0.8, 0.8, 0.0))),
    ));

    // metal sphere
    world.add(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Metal::new(SolidColor::new(Color::new(0.8, 0.6, 0.2)), 0.0),
    ));
    // glass sphere
    world.add(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Dielectric::new(1.5),
    ));

    let camera_position: Vec3 = Vec3::new(0.0, 0.0, 5.0);
    let camera_target: Vec3 = Vec3::new(0.0, 0.0, -1.0);
    let camera_up: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let fov: Float = 25.0;
    let aspect_ratio: Float = width as Float / height as Float;

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
