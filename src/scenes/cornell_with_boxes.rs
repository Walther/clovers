use super::Scene;
use crate::{
    camera::Camera,
    color::Color,
    hitable::HitableList,
    materials::{DiffuseLight, Lambertian, Material},
    objects::{Boxy, RotateY, Translate},
    rect::{XYRect, XZRect, YZRect},
    textures::SolidColor,
    Float, Vec3, HEIGHT, WIDTH,
};
use rand::prelude::*;
use std::sync::Arc;

pub fn load(rng: ThreadRng) -> Scene {
    let time_0: Float = 0.0;
    let time_1: Float = 1.0;
    let mut world: HitableList = HitableList::new();

    // Cornell box

    let red = Lambertian::new(Arc::new(SolidColor::new(Color::new(0.65, 0.05, 0.05))));
    let white: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::new(SolidColor::new(
        Color::new(0.73, 0.73, 0.73),
    ))));
    let green = Lambertian::new(Arc::new(SolidColor::new(Color::new(0.12, 0.45, 0.15))));
    let light = DiffuseLight::new(Arc::new(SolidColor::new(Color::new(15.0, 15.0, 15.0))));

    world.hitables.push(Arc::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::new(green),
    )));
    world.hitables.push(Arc::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Arc::new(red),
    )));
    world.hitables.push(Arc::new(XZRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        Arc::new(light),
    )));
    world.hitables.push(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Arc::clone(&white),
    )));
    world.hitables.push(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&white),
    )));
    world.hitables.push(Arc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&white),
    )));

    // Boxes

    let box1 = Arc::new(Boxy::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        Arc::clone(&white),
    ));

    let box1 = RotateY::new(box1, 15.0);
    let box1 = Translate::new(Arc::new(box1), Vec3::new(265.0, 0.0, 295.0));
    world.hitables.push(Arc::new(box1));

    let box2 = Arc::new(Boxy::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        Arc::clone(&white),
    ));

    let box2 = RotateY::new(box2, -18.0);
    let box2 = Translate::new(Arc::new(box2), Vec3::new(130.0, 0.0, 65.0));
    world.hitables.push(Arc::new(box2));

    let camera_position: Vec3 = Vec3::new(278.0, 278.0, -800.0);
    let camera_target: Vec3 = Vec3::new(278.0, 278.0, 0.0);
    let camera_up: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let fov: Float = 40.0;
    let aspect_ratio: Float = WIDTH as Float / HEIGHT as Float;
    let aperture: Float = 0.0;
    let focus_distance: Float = 10.0;
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

    let background: Color = Color::new(0.0, 0.0, 0.0); // Black background = only lit by the light, no ambient
    Scene::new(world, camera, time_0, time_1, background, rng)
}
