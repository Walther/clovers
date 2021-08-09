//! Various checkered textures: spatial and surface variants.

use crate::{color::Color, Float, Vec3, PI};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Deserialize, Serialize, Debug)]
/// A standard checkered texture based on spatial 3D texturing.
pub struct SpatialChecker {
    #[serde(default = "default_even")]
    /// Uniform color for the even-numbered checkers of the texture.
    pub even: Color,
    #[serde(default = "default_odd")]
    /// Uniform color for the odd-numbered checkers of the texture.
    pub odd: Color,
    #[serde(default = "default_density_spatial")]
    /// Controls the density of the checkered pattern. Default value is 1.0, which corresponds to filling a 1.0 unit cube in the coordinate system with one color of the pattern. Even values preferred - odd values may create a visually thicker stripe due to two stripes with same color being next to each other.
    pub density: Float,
}

fn default_even() -> Color {
    // White minus middle gray 18%
    Color::new(0.82, 0.82, 0.82)
}

fn default_odd() -> Color {
    // Middle gray 18%
    Color::new(0.18, 0.18, 0.18)
}

fn default_density_spatial() -> Float {
    1.0
}

fn default_density_surface() -> Float {
    10.0
}

impl SpatialChecker {
    /// Create a new SpatialChecker object with the specified colors and density.
    pub fn new(color1: Color, color2: Color, density: Float) -> Self {
        SpatialChecker {
            even: color1,
            odd: color2,
            density,
        }
    }

    /// Evaluates the color at the given spatial position coordinate. Note that the SpatialChecker is spatial - surface coordinates are ignored.
    pub fn color(self, _u: Float, _v: Float, position: Vec3) -> Color {
        // TODO: convert ahead-of-time. NOTE: take into account serde-i-fication; not enough to do in `new` alone
        let density = self.density * PI;
        let sines = 1.0 // cosmetic 1 for readability of following lines :)
            * (density * position.x).sin()
            * (density * position.y).sin()
            * (density * position.z).sin();
        if sines < 0.0 {
            self.odd
        } else {
            self.even
        }
    }
}

#[derive(Copy, Clone, Deserialize, Serialize, Debug)]
/// A standard checkered texture based on 2D surface UV coordinates.
pub struct SurfaceChecker {
    #[serde(default = "default_even")]
    even: Color,
    #[serde(default = "default_odd")]
    odd: Color,
    #[serde(default = "default_density_surface")]
    /// Controls the density of the checkered pattern. Default value is 10, which corresponds to using 10 tiles over the width of the object. On spheres, this means 10 tiles around the sphere.
    density: Float,
}

impl SurfaceChecker {
    /// Create a new SurfaceChecker object with the specified colors and density.
    pub fn new(color1: Color, color2: Color, density: Float) -> Self {
        SurfaceChecker {
            even: color1,
            odd: color2,
            density,
        }
    }

    /// Evaluates the color at the given surface position coordinates. Note that SurfaceChecker is surface-based, and thus ignores the spatial position coordinate.
    pub fn color(self, u: Float, v: Float, _position: Vec3) -> Color {
        // TODO: convert ahead-of-time. NOTE: take into account serde-i-fication; not enough to do in `new` alone
        let density = self.density * PI;
        let sines = 1.0 // cosmetic 1 for readability of following lines :)
            * (density * u).sin()
            * (density * v).sin();
        if sines < 0.0 {
            self.odd
        } else {
            self.even
        }
    }
}
