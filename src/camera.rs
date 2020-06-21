use crate::{Float, Ray, Vec3};
pub struct Camera {
  pub lower_left_corner: Vec3,
  pub horizontal: Vec3,
  pub vertical: Vec3,
  pub origin: Vec3,
}

impl Camera {
  pub fn new(
    look_from: Vec3,
    look_at: Vec3,
    up: Vec3,
    vertical_fov: Float,
    aspect_ratio: Float,
  ) -> Self {
    let theta: Float = vertical_fov * std::f32::consts::PI / 180.0; // TODO: make this a Float too, for global precision
    let half_height: Float = (theta / 2.0).tan();
    let half_width: Float = aspect_ratio * half_height;
    let origin: Vec3 = look_from;
    let w: Vec3 = (look_from - look_at).normalize();
    let u: Vec3 = (up.cross(&w)).normalize();
    let v: Vec3 = w.cross(&u);

    let lower_left_corner: Vec3 = origin - half_width * u - half_height * v - w;
    let horizontal: Vec3 = 2.0 * half_width * u;
    let vertical: Vec3 = 2.0 * half_height * v;
    Camera {
      lower_left_corner,
      horizontal,
      vertical,
      origin,
    }
  }

  pub fn get_ray(&self, u: Float, v: Float) -> Ray {
    return Ray::new(
      self.origin,
      self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
    );
  }
}
