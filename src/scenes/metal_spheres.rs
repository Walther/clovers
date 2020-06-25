use crate::{
    camera::Camera,
    color::Color,
    hitable::HitableList,
    material::{Dielectric, Lambertian, Metal},
    moving_sphere::MovingSphere,
    sphere::Sphere,
    Float, Vec3, HEIGHT, WIDTH,
};
use rand::prelude::*;
use std::sync::Arc;

pub fn scene(mut _rng: ThreadRng) -> HitableList {
    let mut world: HitableList = HitableList::new();

    world.hitables.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3))),
    )));

    world.hitables.push(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0))),
    )));

    world.hitables.push(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.3)),
    )));
    world.hitables.push(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 1.0)),
    )));

    world
}

pub fn camera() -> Camera {
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
        0.0,
        1.0,
    );

    camera
}
