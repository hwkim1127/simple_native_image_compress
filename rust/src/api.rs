use flutter_rust_bridge::ZeroCopyBuffer;
use image::{
    codecs::{
        jpeg::JpegEncoder,
        webp,
    },
    imageops, DynamicImage, GenericImageView,
};

#[derive(PartialEq, Eq)]
pub enum CompressFormat {
    Jpeg,
    WebP,
}

fn encode_img_buffer_to_bytes(
    buffer: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    compress_format: CompressFormat,
    quality: u8,
) -> anyhow::Result<Vec<u8>> {
    let mut bytes: Vec<u8> = Vec::new();
    
    if compress_format == CompressFormat::WebP {
        let encoder = webp::WebPEncoder::new_lossless(&mut bytes);
        let _res = buffer.write_with_encoder(encoder);
    } else {
        let encoder = JpegEncoder::new_with_quality(&mut bytes, quality);
        let _res = buffer.write_with_encoder(encoder);
    }
    return Ok(bytes)
}

fn encode_dyn_img_to_bytes(
    dyn_img: &DynamicImage, 
    compress_format: CompressFormat, 
    quality: u8
) -> anyhow::Result<Vec<u8>> {
    let mut bytes: Vec<u8> = Vec::new();
    if compress_format == CompressFormat::WebP {
        let encoder = webp::WebPEncoder::new_lossless(&mut bytes);
        let _res = dyn_img.write_with_encoder(encoder);
    } else {
        let encoder = JpegEncoder::new_with_quality(&mut bytes, quality);
        let _res = dyn_img.write_with_encoder(encoder);   
    }
    return Ok(bytes)
}

fn compress(
    img: &DynamicImage,
    img_height: u32,
    img_width: u32,
    scale: f64,
    compress_format: CompressFormat,
    quality: u8
) -> anyhow::Result<Vec<u8>, > {
    if scale < 1.0 {
        let output_width: u32 = (img_width as f64 * scale) as u32;
        let output_height: u32 = (img_height as f64 * scale) as u32;

        let img_buf: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = imageops::resize(
            img,
            output_width,
            output_height,
            imageops::FilterType::Triangle,
        );
        return encode_img_buffer_to_bytes(&img_buf, compress_format, quality);
    } else {
        return encode_dyn_img_to_bytes(img, compress_format, quality);
    }
}

pub fn fit_width(
    path_str: String,
    compress_format: Option<CompressFormat>,
    max_width: Option<i32>,
    quality: Option<u8>,
) -> anyhow::Result<ZeroCopyBuffer<Vec<u8>>> {
    let img: DynamicImage = image::open(path_str).unwrap();
    let compress_format = compress_format.unwrap_or(CompressFormat::Jpeg);
    let quality = quality.unwrap_or(80);

    let (img_width, img_height) = img.dimensions();
    let max_width = max_width.unwrap_or(1024);

    let ratio_x: f64 = max_width as f64 / img_width as f64;
    let scale: f64 = ratio_x;

    let res = compress(&img, img_height, img_width, scale, compress_format, quality);
    Ok(ZeroCopyBuffer(res.unwrap()))
}

pub fn fit_height(
    path_str: String,
    compress_format: Option<CompressFormat>,
    max_height: Option<i32>,
    quality: Option<u8>,
) -> anyhow::Result<ZeroCopyBuffer<Vec<u8>>> {
    let img: DynamicImage = image::open(path_str).unwrap();
    let compress_format = compress_format.unwrap_or(CompressFormat::Jpeg);
    let quality = quality.unwrap_or(80);

    let (img_width, img_height) = img.dimensions();
    let max_height = max_height.unwrap_or(1024);

    let ratio_y: f64 = max_height as f64 / img_height as f64;
    let scale: f64 = ratio_y;

    let res = compress(&img, img_height, img_width, scale, compress_format, quality);
    Ok(ZeroCopyBuffer(res.unwrap()))
}

pub fn contain(
    path_str: String,
    compress_format: Option<CompressFormat>,
    max_width: Option<i32>,
    max_height: Option<i32>,
    quality: Option<u8>,
) -> anyhow::Result<ZeroCopyBuffer<Vec<u8>>> {
    let img: DynamicImage = image::open(path_str).unwrap();
    let compress_format = compress_format.unwrap_or(CompressFormat::Jpeg);
    let quality = quality.unwrap_or(80);

    let (img_width, img_height) = img.dimensions();
    let max_width = max_width.unwrap_or(1024);
    let max_height = max_height.unwrap_or(1024);

    let ratio_x: f64 = max_width as f64 / img_width as f64;
    let ratio_y: f64 = max_height as f64 / img_height as f64;
    let mut scale: f64 = ratio_y;
    if ratio_x < ratio_y {
        scale = ratio_x;
    }

    let res = compress(&img, img_height, img_width, scale, compress_format, quality);
    Ok(ZeroCopyBuffer(res.unwrap()))
}