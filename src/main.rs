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
}

struct HitRecord {
    distance: Float,
    position: Vec3,
    normal: Vec3,
}

trait Hitable {
    fn hit(&self, ray: &Ray, distance_min: Float, distance_max: Float) -> Option<HitRecord>;
}

/// Helper struct for storing multiple `Hitable` objects. This list has a `Hitable` implementation too, returning the closest possible hit
struct HitableList {
    hitables: Vec<Box<dyn Hitable>>,
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, distance_min: Float, distance_max: Float) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest = distance_max;
        for hitable in self.hitables.iter() {
            if let Some(record) = hitable.hit(&ray, distance_min, closest) {
                closest = record.distance;
                hit_record = Some(record);
            }
        }
        return hit_record;
    }
}

struct Sphere {
    center: Vec3,
    radius: Float,
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, distance_min: Float, distance_max: Float) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            // First possible root
            let distance = (-b - discriminant.sqrt()) / a;
            if distance < distance_max && distance > distance_min {
                let position: Vec3 = ray.point_at_parameter(distance);
                let normal = (position - self.center) / self.radius;
                return Some(HitRecord {
                    distance,
                    position,
                    normal,
                });
            }
            // Second possible root
            let distance = (-b + discriminant.sqrt()) / a;
            if distance < distance_max && distance > distance_min {
                let position: Vec3 = ray.point_at_parameter(distance);
                let normal = (position - self.center) / self.radius;
                return Some(HitRecord {
                    distance,
                    position,
                    normal,
                });
            }
        }
        None
    }
}

/// The main coloring function
fn color(ray: &Ray, world: &dyn Hitable) -> Rgb<u8> {
    let color: Vec3;

    if let Some(hit_record) = world.hit(&ray, 0.0, Float::MAX) {
        // Hit an object, colorize based on surface normals
        color = 0.5
            * Vec3::new(
                hit_record.normal.x + 1.0,
                hit_record.normal.y + 1.0,
                hit_record.normal.z + 1.0,
            );
    } else {
        // Background, blue-white gradient. Magic from tutorial.
        let unit_direction: Vec3 = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        color = (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);
    }

    // Integer-i-fy
    let r = (255.99 * color.x).floor() as u8;
    let g = (255.99 * color.y).floor() as u8;
    let b = (255.99 * color.z).floor() as u8;
    Rgb([r, g, b])
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

    let sphere1 = Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };
    let sphere2 = Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    };
    let world: HitableList = HitableList {
        hitables: vec![Box::new(sphere1), Box::new(sphere2)],
    };

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let u: Float = x as Float / width as Float;
        let v: Float = y as Float / height as Float;
        let ray = Ray::new(origin, upper_left_corner + u * horizontal + v * vertical);
        let color = color(&ray, &world);
        *pixel = color;
    }

    img.save("renders/image.png")
}
