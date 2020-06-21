use crate::{random_in_unit_sphere, HitRecord, Ray, ThreadRng, Vec3};

pub trait Material: Sync + Send {
  /// Returns `None`, if the ray gets absorbed.
  /// Returns `Some(scattered, attenuation)`, if the ray gets scattered
  fn scatter(&self, ray: &Ray, hit_record: &HitRecord, rng: ThreadRng) -> Option<(Ray, Vec3)>;
}

#[derive(Clone)]
pub struct Lambertian {
  albedo: Vec3,
}

impl Material for Lambertian {
  fn scatter(&self, _ray: &Ray, hit_record: &HitRecord, rng: ThreadRng) -> Option<(Ray, Vec3)> {
    let target = hit_record.position + hit_record.normal + random_in_unit_sphere(rng);
    let scattered: Ray = Ray::new(hit_record.position, target - hit_record.position);
    let attenuation: Vec3 = self.albedo;
    Some((scattered, attenuation))
  }
}

impl Lambertian {
  pub fn new(albedo: Vec3) -> Self {
    Lambertian { albedo }
  }
}

#[derive(Clone)]
pub struct Metal {
  albedo: Vec3,
}

fn reflect(vector: Vec3, normal: Vec3) -> Vec3 {
  vector - 2.0 * vector.dot(&normal) * normal
}

impl Material for Metal {
  fn scatter(&self, ray: &Ray, hit_record: &HitRecord, _rng: ThreadRng) -> Option<(Ray, Vec3)> {
    let reflected: Vec3 = reflect(ray.direction.normalize(), hit_record.normal);
    let scattered: Ray = Ray::new(hit_record.position, reflected);
    let attenuation: Vec3 = self.albedo;
    if scattered.direction.dot(&hit_record.normal) > 0.0 {
      Some((scattered, attenuation))
    } else {
      None
    }
  }
}

impl Metal {
  pub fn new(albedo: Vec3) -> Self {
    Metal { albedo }
  }
}
