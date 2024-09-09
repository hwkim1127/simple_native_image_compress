use image::{
    codecs::{jpeg::JpegEncoder, webp},
    DynamicImage,
};

use super::constants::CompressFormat;

pub(crate) fn img_buffer_to_bytes_with_alpha(
    buffer: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
) -> anyhow::Result<Vec<u8>> {
    let mut bytes: Vec<u8> = Vec::new();
    let encoder = webp::WebPEncoder::new_lossless(&mut bytes);
    buffer.write_with_encoder(encoder)?;
    return Ok(bytes);
}

pub(crate) fn img_buffer_to_bytes_without_alpha(
    buffer: &image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    quality: u8,
) -> anyhow::Result<Vec<u8>> {
    let mut bytes: Vec<u8> = Vec::new();
    let encoder = JpegEncoder::new_with_quality(&mut bytes, quality);
    buffer.write_with_encoder(encoder)?;
    return Ok(bytes);
}

pub(crate) fn dyn_img_to_bytes(
    dyn_img: &DynamicImage,
    compress_format: CompressFormat,
    quality: u8,
) -> anyhow::Result<Vec<u8>> {
    let mut bytes: Vec<u8> = Vec::new();
    if compress_format == CompressFormat::WebP {
        let encoder = webp::WebPEncoder::new_lossless(&mut bytes);
        dyn_img.write_with_encoder(encoder)?;
    } else {
        let encoder = JpegEncoder::new_with_quality(&mut bytes, quality);
        dyn_img.to_rgb8().write_with_encoder(encoder)?;
    }
    return Ok(bytes);
}
