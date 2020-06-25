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

pub fn scene(mut rng: ThreadRng) -> HitableList {
    let mut world: HitableList = HitableList::new();

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    let ground_sphere = Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(ground_material),
    );
    world.hitables.push(Box::new(ground_sphere));

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
                    let albedo = Color::random(rng);
                    let sphere_material = Lambertian::new(albedo);
                    let center2 = center + Vec3::new(0.0, rng.gen_range(0.0, 0.5), 0.0);
                    world.hitables.push(Box::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        Arc::new(sphere_material),
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(rng);
                    let fuzz = rng.gen_range(0.0, 0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.hitables.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(sphere_material),
                    )));
                } else {
                    // glass
                    let sphere_material = Dielectric::new(1.5);
                    world.hitables.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(sphere_material),
                    )));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.hitables.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(material1),
    )));

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.hitables.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(material2),
    )));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.hitables.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(material3),
    )));

    return world;
}

pub fn camera() -> Camera {
    let camera_position: Vec3 = Vec3::new(13.0, 2.0, 3.0);
    let camera_target: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let camera_up: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let fov: Float = 25.0;
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
        0.0,
        1.0,
    );

    camera
}
