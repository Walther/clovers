use super::Scene;
use crate::{
    camera::Camera,
    color::Color,
    hitable::HitableList,
    material::Lambertian,
    sphere::Sphere,
    texture::{Checkered, SolidColor, Texture},
    Float, Vec3, HEIGHT, WIDTH,
};
use rand::prelude::*;
use std::sync::Arc;

pub fn load(rng: ThreadRng) -> Scene {
    let time_0: Float = 0.0;
    let time_1: Float = 1.0;
    let mut world: HitableList = HitableList::new();

    let checker: Arc<dyn Texture> = Arc::new(Checkered::new(
        Arc::new(SolidColor::new(Color::new(0.2, 0.3, 0.1))),
        Arc::new(SolidColor::new(Color::new(0.9, 0.9, 0.9))),
        10.0,
    ));

    world.hitables.push(Arc::new(Sphere::new(
        Vec3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(Arc::clone(&checker))),
    )));
    world.hitables.push(Arc::new(Sphere::new(
        Vec3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(Arc::clone(&checker))),
    )));

    let camera_position: Vec3 = Vec3::new(13.0, 2.0, 3.0);
    let camera_target: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let camera_up: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let fov: Float = 20.0;
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
