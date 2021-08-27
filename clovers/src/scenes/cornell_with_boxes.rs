use super::Scene;
use crate::{
    camera::Camera,
    color::Color,
    hitable::HitableList,
    materials::{DiffuseLight, Lambertian, Metal},
    objects::{Boxy, FlipFace, RotateY, Translate},
    objects::{XYRect, XZRect, YZRect},
    textures::SolidColor,
    Float, Vec3,
};
use rand::prelude::*;

pub fn load(width: u32, height: u32, rng: &mut ThreadRng) -> Scene {
    let time_0: Float = 0.0;
    let time_1: Float = 1.0;
    let mut world = HitableList::new();

    // Cornell box
    let red = Lambertian::new(SolidColor::new(Color::new(0.65, 0.05, 0.05)));
    let white = Lambertian::new(SolidColor::new(Color::new(0.73, 0.73, 0.73)));
    let green = Lambertian::new(SolidColor::new(Color::new(0.12, 0.45, 0.15)));
    let small_light = DiffuseLight::new(SolidColor::new(Color::new(15.0, 15.0, 15.0)));

    // Lights are one-sided. Flip this one!
    let small_light_obj = XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, small_light);
    let small_light_obj = FlipFace::new(small_light_obj);
    world.add(small_light_obj);

    world.add(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green));
    world.add(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red));
    world.add(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white));
    world.add(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white));
    world.add(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white));

    // Boxes

    let aluminum = Metal::new(SolidColor::new(Color::new(0.8, 0.85, 0.88)), 0.0);
    let box1 = Box::new(Boxy::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        aluminum,
    ));

    let box1 = RotateY::new(box1, 15.0);
    let box1 = Translate::new(Box::new(box1), Vec3::new(265.0, 0.0, 295.0));
    world.add(box1);

    let box2 = Box::new(Boxy::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        white,
    ));

    let box2 = RotateY::new(box2, -18.0);
    let box2 = Translate::new(Box::new(box2), Vec3::new(130.0, 0.0, 65.0));
    world.add(box2);

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
    Scene::new(world, camera, time_0, time_1, background, &mut rng)
}
