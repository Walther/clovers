//! Camera. Used for creating [Rays](crate::ray::Ray) towards the scene, with directions defined by the camera properties.

#![allow(clippy::too_many_arguments)] // TODO: Camera::new() has a lot of arguments.

use crate::wavelength::Wavelength;
use crate::{Direction, Float, PI, Position, Vec2, Vec3, ray::Ray};
use nalgebra::Unit;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// The main [Camera] object used in the ray tracing.
pub struct Camera {
    /// Coordinate of the lower left corner of the camera.
    pub lower_left_corner: Position,
    /// Defines the horizontal axis for the camera.
    pub horizontal: Vec3,
    /// Defines the vertical axis for the camera.
    pub vertical: Vec3,
    /// Defines the origin of the camera.
    pub origin: Position,
    /// Defines the lens radius for the camera. TODO: understand and explain better
    pub lens_radius: Float,
    /// Defines the earliest starting time for the camera, used when generating [Rays](Ray).
    pub time_0: Float,
    /// Defines the latest ending time for the camera, used when generating [Rays](Ray).
    pub time_1: Float,
    // TODO: clarify these odd one-letter variables
    /// U
    pub u: Direction,
    /// V
    pub v: Direction,
    /// The forward direction of the camera; the difference between `look_from` and `look_at` normalized to a unit vector.
    pub w: Direction,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// Represents the fields that can be described in a Scene file. Some other fields the main Camera struct requires (such as `aspect_ratio`) are derived from other info (such as width, height)
pub struct CameraInit {
    /// Describes where the camera is
    pub look_from: Position,
    /// Describes where the camera is looking at
    pub look_at: Position,
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
        look_from: Position,
        look_at: Position,
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
        let origin: Position = look_from;
        let w: Direction = Unit::new_normalize(look_from - look_at);
        let u: Direction = Unit::new_normalize(up.cross(&w));
        let v: Direction = Unit::new_normalize(w.cross(&u));

        // TODO: understand this defocus
        let lower_left_corner: Vec3 = origin
            - half_width * focus_distance * *u
            - half_height * focus_distance * *v
            - focus_distance * *w;
        let horizontal: Vec3 = 2.0 * half_width * focus_distance * *u;
        let vertical: Vec3 = 2.0 * half_height * focus_distance * *v;

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

    /// Generates a new [Ray] from the camera
    #[must_use]
    pub fn get_ray(
        &self,
        pixel_uv: Vec2, // pixel location in image uv coordinates, range 0..1
        mut lens_offset: Vec2,
        time: Float,
        wavelength: Wavelength,
    ) -> Ray {
        let (x_offset, y_offset) = (pixel_uv.x, pixel_uv.y);
        // TODO: add a better defocus blur / depth of field implementation
        lens_offset.x *= &self.lens_radius;
        lens_offset.y *= &self.lens_radius;
        let offset: Vec3 = *self.u * lens_offset.x + *self.v * lens_offset.y;
        let direction =
            self.lower_left_corner + x_offset * self.horizontal + y_offset * self.vertical
                - self.origin
                - offset;
        let direction = Unit::new_normalize(direction);
        Ray {
            origin: self.origin + offset,
            direction,
            time,
            wavelength,
        }
    }
}
