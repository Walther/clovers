use super::Scene;
use crate::{
    camera::Camera,
    color::Color,
    hitable::HitableList,
    materials::{Dielectric, Lambertian, Metal},
    objects::{MovingSphere, Sphere},
    textures::{Checkered, SolidColor},
    Float, Vec3,
};
use rand::prelude::*;

pub fn load(width: u32, height: u32, mut rng: ThreadRng) -> Scene {
    let time_0: Float = 0.0;
    let time_1: Float = 1.0;
    let mut world: HitableList = HitableList::new();

    let ground_color1 = Color::new(0.2, 0.3, 0.1);
    let ground_color2 = Color::new(0.9, 0.9, 0.9);
    let ground_texture = Checkered::new(ground_color1, ground_color2, 10.0);
    let ground_material = Lambertian::new(ground_texture);
    let ground_sphere = Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_material);
    world.add(ground_sphere);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<Float>();
            let center = Vec3::new(
                a as Float + 0.9 * rng.gen::<Float>(),
                0.2,
                b as Float + 0.9 * rng.gen::<Float>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let color = Color::random(rng);
                    let texture = SolidColor::new(color);
                    let sphere_material = Lambertian::new(texture);
                    let center2 = center + Vec3::new(0.0, rng.gen_range(0.0, 0.5), 0.0);
                    world.add(MovingSphere::new(
                        center,
                        center2,
                        time_0,
                        time_1,
                        0.2,
                        sphere_material,
                    ));
                } else if choose_mat < 0.95 {
                    // metal
                    let color = Color::random(rng);
                    let texture = SolidColor::new(color);
                    let fuzz = rng.gen_range(0.0, 0.5);
                    let sphere_material = Metal::new(texture, fuzz);
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else {
                    // glass
                    let sphere_material = Dielectric::new(1.5);
                    world.add(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.add(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Lambertian::new(SolidColor::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Metal::new(SolidColor::new(Color::new(0.7, 0.6, 0.5)), 0.0);
    world.add(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3));

    let camera_position: Vec3 = Vec3::new(13.0, 2.0, 3.0);
    let camera_target: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let camera_up: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let fov: Float = 25.0;
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

    let background: Color = Color::new(0.7, 0.7, 0.7); // TODO: gradient from first book

    Scene::new(world, camera, time_0, time_1, background, rng)
}
