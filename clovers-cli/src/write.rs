use std::fs::File;
use std::io::Cursor;

use clovers::Float;
use humantime::FormattedDuration;
use image::{ImageBuffer, ImageFormat, Rgb32FImage, RgbImage};
use img_parts::png::{Png, PngChunk};
use palette::{chromatic_adaptation::AdaptInto, white_point::E, Xyz};
use tracing::info;

use crate::render::{RenderMode, RenderOptions};

pub fn png(
    pixelbuffer: &[Xyz<E>],
    target: &String,
    duration: &FormattedDuration,
    render_options: &RenderOptions,
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
        formats: _,
    } = render_options;

    info!("Converting pixelbuffer to an image");
    let mut img: RgbImage = ImageBuffer::new(*width, *height);
    img.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let index = y * width + x;
        let color: palette::Srgb<Float> = pixelbuffer[index as usize].adapt_into();
        let color: palette::Srgb<u8> = color.into_format();
        *pixel = image::Rgb([color.red, color.green, color.blue]);
    });

    info!("Writing an image file");
    let mut bytes: Vec<u8> = Vec::new();
    match img.write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png) {
        Ok(it) => it,
        Err(err) => return Err(err.to_string()),
    }
    let mut png = Png::from_bytes(bytes.into()).or(Err("Unable to read bytes"))?;

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

pub fn exr(
    pixelbuffer: &[Xyz<E>],
    target: &String,
    _duration: &FormattedDuration,
    render_options: &RenderOptions,
) -> Result<(), String> {
    let RenderOptions {
        input: _,
        output: _,
        width,
        height,
        samples: _,
        max_depth: _,
        mode: _,
        sampler: _,
        bvh: _,
        formats: _,
    } = render_options;
    // TODO: metadata?

    info!("Converting pixelbuffer to an image");
    let mut img: Rgb32FImage = ImageBuffer::new(*width, *height);
    img.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let index = y * width + x;
        // NOTE: EXR format expects linear rgb
        let color: palette::LinSrgb<Float> = pixelbuffer[index as usize].adapt_into();
        *pixel = image::Rgb([color.red, color.green, color.blue]);
    });

    img.save_with_format(target, ImageFormat::OpenExr)
        .or(Err("Unable to write to file".to_owned()))
}
