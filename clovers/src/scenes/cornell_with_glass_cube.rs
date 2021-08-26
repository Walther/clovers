use super::Scene;
use crate::{
    camera::Camera,
    color::Color,
    hitable::HitableList,
    materials::{Dielectric, DiffuseLight, Lambertian},
    objects::{Boxy, FlipFace},
    objects::{XYRect, XZRect, YZRect},
    textures::SolidColor,
    Float, Vec3,
};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

pub fn load(width: u32, height: u32, rng: SmallRng) -> Scene {
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

    // glass cube
    let cube = Boxy::new(
        Vec3::new(178.0, 178.0, 178.0),
        Vec3::new(378.0, 378.0, 378.0),
        Dielectric::new(1.5),
    );
    world.add(cube);

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
