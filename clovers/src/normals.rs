//! Alternative render method to [colorize](crate::colorize::colorize).

use crate::CloversRng;
use crate::{color::Color, ray::Ray, scenes::Scene, Float, Vec3, EPSILON_SHADOW_ACNE};

/// Rendering function for getting a normal map in tangent space. Sends a [Ray] to the [Scene], sees what it hits, gets the normal at that point, and returns a color based on the normal mapping colorization. Wikipedia: [Normal mapping](https://en.wikipedia.org/wiki/Normal_mapping).
// TODO: better name
pub fn normal_map(ray: &Ray, scene: &Scene, rng: &mut CloversRng) -> Color {
    let hit_record = match scene.objects.hit(ray, EPSILON_SHADOW_ACNE, Float::MAX, rng) {
        // If the ray hits nothing, early return black
        None => return Color::new(0.0, 0.0, 0.0),
        // Hit something, continue
        Some(hit_record) => hit_record,
    };

    let normal: Vec3 = hit_record.normal;
    let color: Color = normal_to_color(normal);

    color
}

/// Given a surface normal, return a color based on normal mapping colorization.
pub fn normal_to_color(normal: Vec3) -> Color {
    // normalize just in case
    let normal: Vec3 = normal.normalize();
    // flip the Z and X axes because the wikipedia example uses left-handed coordinate system and my renderer uses a right-handed one for some reason.
    // TODO: figure out a good coordinate system to use... See also https://twitter.com/FreyaHolmer/status/1325556229410861056
    let normal: Vec3 = Vec3::new(-normal.x, normal.y, -normal.z);
    // TODO: verify correctness
    let r: Float = 0.5 + 0.5 * normal.x; // X -1 to 1 = 0.0 to 1.0
    let g: Float = 0.5 + 0.5 * normal.y; // Y -1 to 1 = 0.0 to 1.0
                                         // Z  0 to 1 = 0.0 to 1.0
    let b: Float = if normal.z < 0.0 { 0.0 } else { normal.z };

    Color::new(r, g, b)
}
