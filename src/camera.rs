use crate::{Float, Ray, Vec3};
pub struct Camera {
  pub upper_left_corner: Vec3,
  pub horizontal: Vec3,
  pub vertical: Vec3,
  pub origin: Vec3,
}

impl Default for Camera {
  fn default() -> Self {
    Camera {
      upper_left_corner: Vec3::new(-2.0, 1.0, -1.0),
      horizontal: Vec3::new(4.0, 0.0, 0.0),
      vertical: Vec3::new(0.0, -2.0, 0.0),
      origin: Vec3::new(0.0, 0.0, 0.0),
    }
  }
}

impl Camera {
  pub fn get_ray(&self, u: Float, v: Float) -> Ray {
    return Ray::new(
      self.origin,
      self.upper_left_corner + u * self.horizontal + v * self.vertical - self.origin,
    );
  }
}
