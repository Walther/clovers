use crate::{Float, Ray, Vec3};
use rand::prelude::*;

fn random_in_unit_disk(rng: &mut ThreadRng) -> Vec3 {
  let mut position: Vec3;
  loop {
    // TODO: understand this defocus disk thingy
    position = 2.0 * Vec3::new(rng.gen(), rng.gen(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
    if position.dot(&position) >= 1.0 {
      return position;
    }
  }
}

pub struct Camera {
  pub lower_left_corner: Vec3,
  pub horizontal: Vec3,
  pub vertical: Vec3,
  pub origin: Vec3,
  pub lens_radius: Float,
  pub u: Vec3,
  pub v: Vec3,
  pub w: Vec3,
}

impl Camera {
  pub fn new(
    look_from: Vec3,
    look_at: Vec3,
    up: Vec3,
    vertical_fov: Float,
    aspect_ratio: Float,
    aperture: Float,
    focus_distance: Float,
  ) -> Self {
    let lens_radius = aperture / 2.0;
    let theta: Float = vertical_fov * std::f32::consts::PI / 180.0; // TODO: make this a Float too, for global precision
    let half_height: Float = (theta / 2.0).tan();
    let half_width: Float = aspect_ratio * half_height;
    let origin: Vec3 = look_from;
    let w: Vec3 = (look_from - look_at).normalize();
    let u: Vec3 = (up.cross(&w)).normalize();
    let v: Vec3 = w.cross(&u);

    // TODO: understand this defocus
    let lower_left_corner: Vec3 = origin
      - half_width * focus_distance * u
      - half_height * focus_distance * v
      - focus_distance * w;
    let horizontal: Vec3 = 2.0 * half_width * focus_distance * u;
    let vertical: Vec3 = 2.0 * half_height * focus_distance * v;
    Camera {
      lower_left_corner,
      horizontal,
      vertical,
      origin,
      lens_radius,
      u,
      v,
      w,
    }
  }

  // TODO: fix the mysterious (u,v) vs (s,t) change that came from the tutorial
  pub fn get_ray(&self, s: Float, t: Float, mut rng: ThreadRng) -> Ray {
    let rd: Vec3 = self.lens_radius * random_in_unit_disk(&mut rng);
    let offset: Vec3 = self.u * rd.x + self.v * rd.y;
    return Ray::new(
      self.origin + offset,
      self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
    );
  }
}
