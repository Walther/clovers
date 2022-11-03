//! Camera. Used for creating [Rays](crate::ray::Ray) towards the scene, with directions defined by the camera properties.

#![allow(clippy::too_many_arguments)] // TODO: Camera::new() has a lot of arguments.

use crate::{random::random_in_unit_disk, ray::Ray, Float, Vec3, PI};
use rand::rngs::SmallRng;
use rand::Rng;

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// The main [Camera] object used in the ray tracing.
pub struct Camera {
    /// Coordinate of the lower left corner of the camera.
    pub lower_left_corner: Vec3,
    /// Defines the horizontal axis for the camera.
    pub horizontal: Vec3,
    /// Defines the vertical axis for the camera.
    pub vertical: Vec3,
    /// Defines the origin of the camera.
    pub origin: Vec3,
    /// Defines the lens radius for the camera. TODO: understand and explain better
    pub lens_radius: Float,
    /// Defines the earliest starting time for the camera, used when generating [Rays](Ray).
    pub time_0: Float,
    /// Defines the latest ending time for the camera, used when generating [Rays](Ray).
    pub time_1: Float,
    // TODO: clarify these odd one-letter variables
    /// U
    pub u: Vec3,
    /// V
    pub v: Vec3,
    /// W
    pub w: Vec3,
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// Represents the fields that can be described in a Scene file. Some other fields the main Camera struct requires (such as `aspect_ratio`) are derived from other info (such as width, height)
pub struct CameraInit {
    /// Describes where the camera is
    pub look_from: Vec3,
    /// Describes where the camera is looking at
    pub look_at: Vec3,
    /// Describes the subjective "up" direction for the camera to define the orientation
    pub up: Vec3,
    /// Describes the vertical field of view for the camera
    pub vertical_fov: Float,
    /// Describes the size of the aperture of the camera.
    // TODO: does it really though
    pub aperture: Float,
    /// Describes the distance at which the camera has been focused to
    pub focus_distance: Float,
}

impl Camera {
    /// Creates a new [Camera] with the given parameters.
    #[must_use]
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
    /// Generates a new [Ray] from the camera, at a random location of the aperture, at a random time interval between `time_0`, `time_1` of the camera.
    #[must_use]
    pub fn get_ray(self, s: Float, t: Float, rng: &mut SmallRng) -> Ray {
        // TODO: add a better defocus blur / depth of field implementation
        let rd: Vec3 = self.lens_radius * random_in_unit_disk(rng);
        let offset: Vec3 = self.u * rd.x + self.v * rd.y;
        // Randomized time used for motion blur
        let time: Float = rng.gen_range(self.time_0..self.time_1);
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            time,
        )
    }
}
