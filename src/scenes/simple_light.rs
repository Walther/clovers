use crate::{
    camera::Camera,
    color::Color,
    hitable::HitableList,
    material::{Dielectric, DiffuseLight, Lambertian, Material, Metal},
    moving_sphere::MovingSphere,
    perlin::Perlin,
    sphere::Sphere,
    texture::{Checkered, NoiseTexture, SolidColor, Texture},
    xy_rect::XYRect,
    Float, Vec3, HEIGHT, WIDTH,
};
use rand::prelude::*;
use std::sync::Arc;

pub fn scene(mut rng: ThreadRng) -> HitableList {
    let mut world: HitableList = HitableList::new();

    let perlin = Perlin::new(256, rng);
    let perlin2 = Perlin::new(256, rng);

    world.hitables.push(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Arc::new(NoiseTexture::new(perlin, 4.0)))),
    )));
    world.hitables.push(Arc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(Arc::new(NoiseTexture::new(perlin2, 4.0)))),
    )));

    let difflight: Arc<dyn Material> = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(
        Color::new(4.0, 4.0, 4.0),
    ))));
    world.hitables.push(Arc::new(Sphere::new(
        Vec3::new(0.0, 7.0, 0.0),
        2.0,
        Arc::clone(&difflight),
    )));
    world.hitables.push(Arc::new(XYRect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        Arc::clone(&difflight),
    )));

    return world;
}

pub fn camera() -> Camera {
    let camera_position: Vec3 = Vec3::new(20.0, 5.0, 2.0);
    let camera_target: Vec3 = Vec3::new(0.0, 2.0, 0.0);
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
        0.0,
        1.0,
    );

    camera
}
