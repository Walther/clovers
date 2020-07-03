use super::Scene;
use crate::{
    camera::Camera,
    color::Color,
    hitable::{Hitable, HitableList},
    materials::{Dielectric, DiffuseLight, Lambertian, Material},
    objects::{Boxy, RotateY, Sphere, Translate},
    objects::{XYRect, XZRect, YZRect},
    textures::{Checkered, SolidColor, Texture},
    Float, Vec3,
};
use rand::prelude::*;
use std::sync::Arc;
pub fn load(width: u32, height: u32, rng: ThreadRng) -> Scene {
    let time_0: Float = 0.0;
    let time_1: Float = 1.0;
    let mut world: HitableList = HitableList::new();

    // Cornell box
    let red_texture: Arc<dyn Texture> = Arc::new(SolidColor::new(Color::new(0.65, 0.05, 0.05)));
    let red: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&red_texture)));
    let green_texture: Arc<dyn Texture> = Arc::new(SolidColor::new(Color::new(0.12, 0.45, 0.15)));
    let white_texture: Arc<dyn Texture> = Arc::new(SolidColor::new(Color::new(0.73, 0.73, 0.73)));
    let white: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&white_texture)));
    let green: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&green_texture)));

    // TODO: checkered textures may have bugs with boxes / rects
    let oddity: Arc<dyn Texture> = Arc::new(Checkered::new(
        Arc::new(Checkered::new(
            Arc::clone(&red_texture),
            Arc::clone(&green_texture),
            0.2,
        )),
        Arc::clone(&white_texture),
        0.1,
    ));
    let odd_material: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::clone(&oddity)));

    let box1 = Arc::new(Boxy::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        Arc::clone(&odd_material),
    ));

    let box1 = RotateY::new(box1, 15.0);
    let box1 = Translate::new(Arc::new(box1), Vec3::new(265.0, 0.0, 295.0));
    world.hitables.push(Arc::new(box1));

    let light = DiffuseLight::new(Arc::new(SolidColor::new(Color::new(7.0, 7.0, 7.0))));

    world.hitables.push(Arc::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&green),
    )));
    world.hitables.push(Arc::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Arc::clone(&red),
    )));
    world.hitables.push(Arc::new(XZRect::new(
        113.0,
        443.0,
        127.0,
        432.0,
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

    // glass sphere
    let sphere: Arc<dyn Hitable> = Arc::new(Sphere::new(
        Vec3::new(278.0, 278.0, 278.0),
        120.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.hitables.push(Arc::clone(&sphere));

    let camera_position: Vec3 = Vec3::new(278.0, 278.0, -800.0);
    let camera_target: Vec3 = Vec3::new(278.0, 278.0, 0.0);
    let camera_up: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let fov: Float = 40.0;
    let aspect_ratio: Float = width as Float / height as Float;

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
