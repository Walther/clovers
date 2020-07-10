use crate::{random::random_in_unit_disk, Float, Ray, Vec3, PI};
use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Camera {
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
    pub lens_radius: Float,
    pub time_0: Float,
    pub time_1: Float,
    // TODO: clarify these odd one-letter variables
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

#[derive(Serialize, Deserialize, Debug)]
/// Represents the fields that can be described in a Scene file. Some other fields the main Camera struct requires (such as aspect_ratio) are derived from other info (such as width, height)
pub struct CameraInit {
    pub look_from: Vec3,
    pub look_at: Vec3,
    pub up: Vec3,
    pub vertical_fov: Float,
    pub aperture: Float,
    pub focus_distance: Float,
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
        time_0: Float,
        time_1: Float,
    ) -> Self {
        let lens_radius: Float = aperture / 2.0;
        let theta: Float = vertical_fov * PI / 180.0;
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
            time_0,
            time_1,
            u,
            v,
            w,
        }
    }

    // TODO: fix the mysterious (u,v) vs (s,t) change that came from the tutorial
    pub fn get_ray(&self, s: Float, t: Float, mut rng: ThreadRng) -> Ray {
        // TODO: add a better defocus blur / depth of field implementation
        let rd: Vec3 = self.lens_radius * random_in_unit_disk(&mut rng);
        let offset: Vec3 = self.u * rd.x + self.v * rd.y;
        // Randomized time used for motion blur
        let time: Float = rng.gen_range(self.time_0, self.time_1);
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            time,
        )
    }
}
