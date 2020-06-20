use image::{ImageBuffer, ImageResult, RgbImage};

fn main() -> ImageResult<()> {
    println!("clovers - ray tracing in rust <3");
    gradient()
}

fn gradient() -> ImageResult<()> {
    // Let's start dirty & hardcoded
    let width = 1920;
    let height = 1080;

    let mut img: RgbImage = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        // Magic gradient
        let r: f64 = x as f64 / width as f64;
        let g: f64 = y as f64 / height as f64;
        let b: f64 = 0.2;

        // Integer-i-fy
        let r = (255.99 * r).floor() as u8;
        let g = (255.99 * g).floor() as u8;
        let b = (255.99 * b).floor() as u8;

        *pixel = image::Rgb([r, g, b]);
    }

    img.save("renders/image.png")
}
