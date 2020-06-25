use crate::{
    color::Color,
    hitable::HitableList,
    material::{Dielectric, Lambertian, Metal},
    moving_sphere::MovingSphere,
    sphere::Sphere,
    Float, Vec3,
};
use rand::prelude::*;
use std::sync::Arc;

pub fn random_scene(mut rng: ThreadRng) -> HitableList {
    let mut world: HitableList = HitableList::new();

    let ground_material = Lambertian::new(Vec3::new(0.5, 0.5, 0.5));
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
                    let sphere_material = Lambertian::new(albedo.into());
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
                    let sphere_material = Metal::new(albedo.into());
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

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1).into());
    world.hitables.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(material2),
    )));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5).into());
    world.hitables.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(material3),
    )));

    return world;
}
