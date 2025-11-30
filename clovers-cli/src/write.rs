use std::fs::File;
use std::io::Cursor;

use clovers::Float;
use exr::{
    image::{Encoding, Layer, PixelImage},
    meta::attribute::Chromaticities,
    prelude::{LayerAttributes, WritableImage},
};
use humantime::FormattedDuration;
use image::{ImageBuffer, ImageFormat, RgbImage};
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

    let software = "Software\0clovers".to_string();

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

/// Save the pixelbuffer as an OpenEXR file.
///
/// From the specification:
/// > In an OpenEXR file whose pixels represent CIE XYZ tristimulus values,
/// > the pixels’ X, Y and Z components should be stored in the file’s R, G and B channels.
/// > The file header should contain a chromaticities attribute with the following values:
/// > |       | CIE x,y  |
/// > |-------|----------|
/// > | red   |     1, 0 |
/// > | green |     0, 1 |
/// > | blue  |     0, 0 |
/// > | white | 1/3, 1/3 |
/// <cite><https://openexr.com/en/latest/TechnicalIntroduction.html#cie-xyz-color></cite>
pub fn exr(pixelbuffer: &[Xyz<E>], width: u32, height: u32, target: &String) -> Result<(), String> {
    info!("Converting pixelbuffer to an image");
    let dimensions = (width as usize, height as usize);
    let layer_attributes = LayerAttributes {
        // capture_date: todo!(),
        software_name: Some("clovers".into()),
        ..Default::default()
    };
    let encoding = Encoding::SMALL_FAST_LOSSLESS;
    let channels = exr::prelude::SpecificChannels::build()
        .with_channel::<f32>("R")
        .with_channel::<f32>("G")
        .with_channel::<f32>("B")
        .with_pixel_fn(|coord| {
            let index = coord.y() * width as usize + coord.x();
            let pixel: Xyz<E> = pixelbuffer[index];
            pixel.into_components()
        });
    let mut image =
        PixelImage::from_layer(Layer::new(dimensions, layer_attributes, encoding, channels));
    image.attributes.chromaticities = Some(Chromaticities {
        red: (1.0, 0.0).into(),
        green: (0.0, 1.0).into(),
        blue: (0.0, 0.0).into(),
        white: (1.0 / 3.0, 1.0 / 3.0).into(),
    });
    image.write().to_file(target).unwrap();
    Ok(())
}
