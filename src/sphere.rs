use crate::{Float, HitRecord, Hitable, Ray, Vec3};

pub struct Sphere {
  center: Vec3,
  radius: Float,
}

impl Sphere {
  pub fn new(center: Vec3, radius: Float) -> Self {
    Sphere { center, radius }
  }
}

impl Hitable for Sphere {
  fn hit(&self, ray: &Ray, distance_min: Float, distance_max: Float) -> Option<HitRecord> {
    let oc = ray.origin - self.center;
    let a = ray.direction.dot(&ray.direction);
    let b = oc.dot(&ray.direction);
    let c = oc.dot(&oc) - self.radius * self.radius;
    let discriminant = b * b - a * c;
    if discriminant > 0.0 {
      // First possible root
      let distance = (-b - discriminant.sqrt()) / a;
      if distance < distance_max && distance > distance_min {
        let position: Vec3 = ray.point_at_parameter(distance);
        let normal = (position - self.center) / self.radius;
        return Some(HitRecord {
          distance,
          position,
          normal,
        });
      }
      // Second possible root
      let distance = (-b + discriminant.sqrt()) / a;
      if distance < distance_max && distance > distance_min {
        let position: Vec3 = ray.point_at_parameter(distance);
        let normal = (position - self.center) / self.radius;
        return Some(HitRecord {
          distance,
          position,
          normal,
        });
      }
    }
    None
  }
}
