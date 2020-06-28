use crate::{color::Color, texture::Texture, Float, HitRecord, Ray, ThreadRng, Vec3, PI};
use rand::prelude::*;
use std::sync::Arc;

// Internal helper. Originally used for lambertian reflection with flaws
fn random_in_unit_sphere(mut rng: ThreadRng) -> Vec3 {
    let mut position: Vec3;
    loop {
        position = 2.0 * Vec3::new(rng.gen(), rng.gen(), rng.gen()) - Vec3::new(1.0, 1.0, 1.0);
        if position.magnitude_squared() >= 1.0 {
            return position;
        }
    }
}

// Internal helper. Use this for the more correct "True Lambertian" reflection
fn random_unit_vector(mut rng: ThreadRng) -> Vec3 {
    let a: Float = rng.gen_range(0.0, 2.0 * PI);
    let z: Float = rng.gen_range(-1.0, 1.0);
    let r: Float = (1.0 - z * z).sqrt();
    return Vec3::new(r * a.cos(), r * a.sin(), z);
}

pub trait Material: Sync + Send {
    /// Returns `None`, if the ray gets absorbed.
    /// Returns `Some(scattered, attenuation)`, if the ray gets scattered
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, rng: ThreadRng) -> Option<(Ray, Color)>;

    /// Returns the amount of light the material emits. By default, materials do not emit light, returning black.
    fn emitted(&self, _u: Float, _v: Float, _position: Vec3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

#[derive(Clone)]
pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, rng: ThreadRng) -> Option<(Ray, Color)> {
        let scatter_direction: Vec3 = hit_record.normal + random_unit_vector(rng);
        let scattered = Ray::new(hit_record.position, scatter_direction, ray.time);
        let attenuation = self
            .albedo
            .color(hit_record.u, hit_record.v, hit_record.position);
        Some((scattered, attenuation))

        // old backup
        //     let target = hit_record.position + hit_record.normal + random_unit_vector(rng);
        //     let scattered: Ray = Ray::new(hit_record.position, target - hit_record.position, ray.time);
        //     let attenuation: Color = self
        //         .albedo
        //         .color(hit_record.u, hit_record.v, hit_record.position);
        //     Some((scattered, attenuation))
    }
}

impl Lambertian {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Lambertian { albedo }
    }
}

#[derive(Clone)]
pub struct Metal {
    albedo: Arc<dyn Texture>,
    fuzz: Float,
}

fn reflect(vector: Vec3, normal: Vec3) -> Vec3 {
    vector - 2.0 * vector.dot(&normal) * normal
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, rng: ThreadRng) -> Option<(Ray, Color)> {
        let reflected: Vec3 = reflect(ray.direction.normalize(), hit_record.normal);
        let scattered: Ray = Ray::new(
            hit_record.position,
            reflected + self.fuzz * random_in_unit_sphere(rng),
            ray.time,
        );
        let attenuation: Color = self
            .albedo
            .color(hit_record.u, hit_record.v, hit_record.position);
        if scattered.direction.dot(&hit_record.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}

impl Metal {
    pub fn new(albedo: Arc<dyn Texture>, fuzz: Float) -> Self {
        Metal {
            albedo: albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

fn refract(uv: Vec3, normal: Vec3, etai_over_etat: Float) -> Vec3 {
    let cos_theta: Float = -uv.dot(&normal);
    let r_out_parallel: Vec3 = etai_over_etat * (uv + cos_theta * normal);
    let r_out_perp: Vec3 = -(1.0 - r_out_parallel.norm_squared()).sqrt() * normal;
    return r_out_parallel + r_out_perp;
}

fn schlick(cosine: Float, refractive_index: Float) -> Float {
    let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * ((1.0 - cosine).powf(5.0))
}

#[derive(Clone)]
pub struct Dielectric {
    refractive_index: Float,
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        mut rng: ThreadRng,
    ) -> Option<(Ray, Color)> {
        let attenuation: Color = Color::new(1.0, 1.0, 1.0); // Glass does not attenuate
        let scattered: Ray;
        let etai_over_etat: Float = match hit_record.front_face {
            true => 1.0 / self.refractive_index,
            false => self.refractive_index,
        };

        let unit_direction: Vec3 = ray.direction.normalize();
        let cos_theta: Float = (-unit_direction.dot(&hit_record.normal)).min(1.0);
        let sin_theta: Float = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            let reflected: Vec3 = reflect(unit_direction, hit_record.normal);
            scattered = Ray::new(hit_record.position, reflected, ray.time)
        } else {
            let reflect_probability: Float = schlick(cos_theta, etai_over_etat);
            if rng.gen::<Float>() < reflect_probability {
                let reflected: Vec3 = reflect(unit_direction, hit_record.normal);
                scattered = Ray::new(hit_record.position, reflected, ray.time);
            } else {
                let refracted: Vec3 = refract(unit_direction, hit_record.normal, etai_over_etat);
                scattered = Ray::new(hit_record.position, refracted, ray.time);
            }
        }

        Some((scattered, attenuation))
    }
}

impl Dielectric {
    pub fn new(refractive_index: Float) -> Self {
        Dielectric { refractive_index }
    }
}

pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _ray: &Ray,
        _hit_record: &HitRecord,
        _rng: ThreadRng,
    ) -> Option<(Ray, Color)> {
        None
    }
    fn emitted(&self, u: Float, v: Float, position: Vec3) -> Color {
        self.emit.color(u, v, position)
    }
}

// TODO: figure out why this sometimes returns odd black reflections
impl DiffuseLight {
    pub fn new(emission: Arc<dyn Texture>) -> Self {
        DiffuseLight { emit: emission }
    }
}
