use image::{
    codecs::{avif, jpeg::JpegEncoder, webp},
    DynamicImage, GenericImageView,
};

use super::constants::CompressFormat;

#[inline]
pub(crate) fn img_buffer_to_bytes_with_alpha(
    compress_format: CompressFormat,
    buffer: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    quality: u8,
    speed: Option<u8>,
) -> anyhow::Result<Vec<u8>> {
    let mut bytes = Vec::with_capacity(buffer.width() as usize * buffer.height() as usize * 3);
    
    match compress_format {
        CompressFormat::WebP => {
            let encoder = webp::WebPEncoder::new_lossless(&mut bytes);
            buffer.write_with_encoder(encoder)?;
        }
        CompressFormat::AVIF => {
            let encoder = avif::AvifEncoder::new_with_speed_quality(&mut bytes, speed.unwrap_or(10), quality);
            buffer.write_with_encoder(encoder)?;
        }
        _ => unreachable!()
    }

    Ok(bytes)
}

#[inline]
pub(crate) fn img_buffer_to_bytes_without_alpha(
    buffer: &image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    quality: u8,
) -> anyhow::Result<Vec<u8>> {
    let mut bytes = Vec::with_capacity(buffer.width() as usize * buffer.height() as usize * 3);
    let encoder = JpegEncoder::new_with_quality(&mut bytes, quality);
    buffer.write_with_encoder(encoder)?;
    Ok(bytes)
}

#[inline]
pub(crate) fn dyn_img_to_bytes(
    dyn_img: &DynamicImage,
    compress_format: CompressFormat,
    quality: u8,
    speed: Option<u8>,
) -> anyhow::Result<Vec<u8>> {
    let (width, height) = dyn_img.dimensions();
    let mut bytes = Vec::with_capacity(width as usize * height as usize * 3);
    
    match compress_format {
        CompressFormat::WebP => {
            let encoder = webp::WebPEncoder::new_lossless(&mut bytes);
            dyn_img.write_with_encoder(encoder)?;
        }
        CompressFormat::AVIF => {
            let encoder = avif::AvifEncoder::new_with_speed_quality(&mut bytes, speed.unwrap_or(10), quality);
            dyn_img.write_with_encoder(encoder)?;
        }
        CompressFormat::Jpeg => {
            let encoder = JpegEncoder::new_with_quality(&mut bytes, quality);
            dyn_img.to_rgb8().write_with_encoder(encoder)?;
        }
    }
    Ok(bytes)
}
