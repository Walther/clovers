use image::{ImageBuffer, ImageResult, Rgb, RgbImage};

use nalgebra::{Unit, Vector3};

fn main() -> ImageResult<()> {
    println!("clovers - ray tracing in rust <3");
    draw()
}

// Handy aliases for internal use
type Float = f32;
type Vec3 = Vector3<Float>;

struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    fn point_at_parameter(&self, t: Float) -> Vec3 {
        self.origin + t * self.direction
    }

    fn color(&self) -> Rgb<u8> {
        // Chapter 4: sphere and background
        let color: Vec3;
        let sphere_center: Vec3 = Vector3::new(0.0, 0.0, -1.0);
        let sphere_radius = 0.5;
        let sphere = Sphere::new(sphere_center, sphere_radius);
        let unit_direction: Unit<Vec3> = Unit::new_normalize(self.direction);

        if let Some(distance) = sphere.hit(&self) {
            // Sphere
            let normal = self.point_at_parameter(distance) - sphere_center;
            let normal = normal.normalize();
            color = 0.5 * Vec3::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0);
        } else {
            // Background, blue-white gradient. Magic from tutorial.
            let t = 0.5 * (unit_direction.y + 1.0);
            color = (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);
        }

        // Integer-i-fy
        let r = (255.99 * color.x).floor() as u8;
        let g = (255.99 * color.y).floor() as u8;
        let b = (255.99 * color.z).floor() as u8;
        Rgb([r, g, b])
    }
}

struct Sphere {
    center: Vec3,
    radius: Float,
}

impl Sphere {
    fn new(center: Vec3, radius: Float) -> Sphere {
        Sphere { center, radius }
    }

    /// Calculate whether the ray hits the sphere. If, return the distance from ray origin to the hitpoint. If not, return `None`
    fn hit(&self, ray: &Ray) -> Option<Float> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            None
        } else {
            Some((-b - discriminant.sqrt()) / (2.0 * a))
        }
    }
}

/// The main drawing function, returns an `ImageResult`.
fn draw() -> ImageResult<()> {
    // Let's start dirty & hardcoded
    let width = 2000;
    let height = 1000;
    let upper_left_corner: Vec3 = Vec3::new(-2.0, 1.0, -1.0);
    let horizontal: Vec3 = Vec3::new(4.0, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, -2.0, 0.0);
    let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    let mut img: RgbImage = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let u: Float = x as Float / width as Float;
        let v: Float = y as Float / height as Float;
        let ray = Ray::new(origin, upper_left_corner + u * horizontal + v * vertical);
        let color = ray.color();

        *pixel = color;
    }

    img.save("renders/image.png")
}
