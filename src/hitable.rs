use crate::{Float, Material, Ray, Vec3};
use std::sync::Arc;

pub struct HitRecord {
  pub distance: Float,
  pub position: Vec3,
  pub normal: Vec3,
  pub material: Arc<dyn Material>,
}

pub trait Hitable: Sync {
  fn hit(&self, ray: &Ray, distance_min: Float, distance_max: Float) -> Option<HitRecord>;
}

/// Helper struct for storing multiple `Hitable` objects. This list has a `Hitable` implementation too, returning the closest possible hit
pub struct HitableList {
  pub hitables: Vec<Box<dyn Hitable>>,
}

impl Hitable for HitableList {
  fn hit(&self, ray: &Ray, distance_min: Float, distance_max: Float) -> Option<HitRecord> {
    let mut hit_record: Option<HitRecord> = None;
    let mut closest = distance_max;
    for hitable in self.hitables.iter() {
      if let Some(record) = hitable.hit(&ray, distance_min, closest) {
        closest = record.distance;
        hit_record = Some(record);
      }
    }
    return hit_record;
  }
}
