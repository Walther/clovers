use std::fs::File;
use std::io::Cursor;

use humantime::FormattedDuration;
use image::{ImageBuffer, ImageFormat, Rgb, RgbImage};
use img_parts::png::{Png, PngChunk};
use palette::{chromatic_adaptation::AdaptInto, white_point::E, Srgb, Xyz};
use tracing::info;

use crate::render::{RenderMode, RenderOptions};

pub fn png(
    pixelbuffer: Vec<Xyz<E>>,
    target: &String,
    duration: FormattedDuration,
    render_options: RenderOptions,
) -> Result<(), String> {
    let RenderOptions {
        input,
        output: _,
        width,
        height,
        samples,
        max_depth,
        mode,
        sampler: _,
        bvh: _,
        format: _,
    } = render_options;
    info!("Converting pixelbuffer to an image");
    let mut img: RgbImage = ImageBuffer::new(width, height);
    img.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let index = y * width + x;
        let color: Srgb = pixelbuffer[index as usize].adapt_into();
        let color: Srgb<u8> = color.into_format();
        *pixel = Rgb([color.red, color.green, color.blue]);
    });

    // Graphics assume origin at bottom left corner of the screen
    // Our buffer writes pixels from top left corner. Simple fix, just flip it!
    image::imageops::flip_vertical_in_place(&mut img);
    // TODO: fix the coordinate system

    info!("Writing an image file");

    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
        .or(Err("Unable to write bytes"))?;
    let mut png = Png::from_bytes(bytes.into()).or(Err("Unable to write bytes"))?;

    let common = format!(
        "Comment\0Rendered with the clovers path tracing engine. Scene file {input} rendered using the {mode:?} rendering mode at {width}x{height} resolution"
    );
    let details = match mode {
        RenderMode::PathTracing => {
            format!(", {samples} samples per pixel, {max_depth} max ray bounce depth.")
        }
        _ => ".".to_owned(),
    };
    let threads =
        std::thread::available_parallelism().or(Err("Unable to detect available parallelism"))?;
    let stats = format!("Rendering finished in {duration}, using {threads} threads.");
    let comment = format!("{common}{details} {stats}");

    let software = "Software\0https://github.com/walther/clovers".to_string();

    for metadata in [comment, software] {
        let bytes = metadata.as_bytes().to_owned();
        let chunk = PngChunk::new([b't', b'E', b'X', b't'], bytes.into());
        png.chunks_mut().push(chunk);
    }

    let output = File::create(target).or(Err("Unable to create file"))?;
    png.encoder()
        .write_to(output)
        .or(Err("Unable to write to file"))?;

    Ok(())
}
