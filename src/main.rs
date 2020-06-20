use image::{ImageBuffer, ImageResult, Rgb, RgbImage};

use nalgebra::{Unit, Vector3};

fn main() -> ImageResult<()> {
    println!("clovers - ray tracing in rust <3");
    draw()
}

// Handy alias for internal use
type Vec3 = Vector3<f32>;

struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }

    fn color(&self) -> Rgb<u8> {
        let unit_direction: Unit<Vec3> = Unit::new_normalize(self.direction);
        // magic hardcoded values from tutorial
        let t = 0.5 * (unit_direction.y + 1.0);
        let magic_vec: Vec3 =
            (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);

        // Integer-i-fy
        let r = (255.99 * magic_vec.x).floor() as u8;
        let g = (255.99 * magic_vec.y).floor() as u8;
        let b = (255.99 * magic_vec.z).floor() as u8;
        Rgb([r, g, b])
    }
}

fn draw() -> ImageResult<()> {
    // Let's start dirty & hardcoded
    let width = 200;
    let height = 100;
    let upper_left_corner: Vec3 = Vec3::new(-2.0, 1.0, -1.0);
    let horizontal: Vec3 = Vec3::new(4.0, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, -2.0, 0.0);
    let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    let mut img: RgbImage = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let u: f32 = x as f32 / width as f32;
        let v: f32 = y as f32 / height as f32;
        let ray = Ray::new(origin, upper_left_corner + u * horizontal + v * vertical);
        let color = ray.color();

        *pixel = color;
    }

    img.save("renders/image.png")
}
