use anyhow::Ok;
use exif::{self, Exif, In, Tag};
use flutter_rust_bridge::ZeroCopyBuffer;
use image::{
    codecs::{jpeg::JpegEncoder, webp},
    imageops, DynamicImage, GenericImageView,
};

#[derive(PartialEq, Eq)]
pub enum CompressFormat {
    Jpeg,
    WebP,
}

/*
    This is required since Flutter Rust Bridge panics...
    https://docs.rs/image/latest/image/imageops/enum.FilterType.html
*/
#[derive(PartialEq, Eq)]
pub enum FilterType {
    Nearest,    // Nearest Neighbor
    Triangle,   // Linear Filter
    CatmullRom, // Cubic Filter
    Gaussian,   // Gaussian Filter
    Lanczos3,   // Lanczos with window 3
}

/*
    it's kinda dumb, but flutter rust bridge panics if the enum is not exposed in api.rs
*/
impl FilterType {
    pub fn to_imageops(&self) -> imageops::FilterType {
        match self {
            FilterType::Nearest => imageops::FilterType::Nearest,
            FilterType::Triangle => imageops::FilterType::Triangle,
            FilterType::CatmullRom => imageops::FilterType::CatmullRom,
            FilterType::Gaussian => imageops::FilterType::Gaussian,
            FilterType::Lanczos3 => imageops::FilterType::Lanczos3,
        }
    }
}

fn check_orientation(path_str: &String) -> anyhow::Result<u32> {
    let file = std::fs::File::open(path_str)?;
    let mut buf_reader = std::io::BufReader::new(&file);
    let exif_reader = exif::Reader::new();
    let exif: Exif = match exif_reader.read_from_container(&mut buf_reader) {
        std::result::Result::Ok(exif) => exif,
        std::result::Result::Err(error) => match error {
            exif::Error::NotFound(_) | exif::Error::BlankValue(_) => return Ok(1),
            _ => return Err(anyhow::anyhow!(error)),
        },
    };

    let orientation = match exif.get_field(Tag::Orientation, In::PRIMARY) {
        Some(orientation) => match orientation.value.get_uint(0) {
            Some(v @ 1..=8) => v,
            _ => 1,
        },
        None => 1,
    };
    Ok(orientation)
}

/*
    https://magnushoff.com/articles/jpeg-orientation/
*/
fn rotate(orientation: u32, dyn_img: DynamicImage) -> DynamicImage {
    match orientation {
        // filp Horizontally
        2 | 4 => return DynamicImage::ImageRgba8(imageops::flip_horizontal(&dyn_img)),
        // rotate180
        3 => return DynamicImage::ImageRgba8(imageops::rotate180(&dyn_img)),
        // rotate 90 & filp Horizontally
        5 => {
            let temp_img = DynamicImage::ImageRgba8(imageops::rotate90(&dyn_img));
            return DynamicImage::ImageRgba8(imageops::flip_horizontal(&temp_img));
        }
        // rotate90
        6 => return DynamicImage::ImageRgba8(imageops::rotate90(&dyn_img)),
        // filp Horizontally & rotate 270
        7 => {
            let temp_img = DynamicImage::ImageRgba8(imageops::flip_horizontal(&dyn_img));
            return DynamicImage::ImageRgba8(imageops::rotate270(&temp_img));
        }
        // rotate 270
        8 => return DynamicImage::ImageRgba8(imageops::rotate270(&dyn_img)),
        // don't do anything
        _ => return dyn_img,
    }
}

fn encode_img_buffer_to_bytes(
    buffer: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    compress_format: CompressFormat,
    quality: u8,
) -> anyhow::Result<Vec<u8>> {
    let mut bytes: Vec<u8> = Vec::new();

    if compress_format == CompressFormat::WebP {
        let encoder = webp::WebPEncoder::new_lossless(&mut bytes);
        buffer.write_with_encoder(encoder)?;
    } else {
        let encoder = JpegEncoder::new_with_quality(&mut bytes, quality);
        buffer.write_with_encoder(encoder)?;
    }
    return Ok(bytes);
}

fn encode_dyn_img_to_bytes(
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
        dyn_img.write_with_encoder(encoder)?;
    }
    return Ok(bytes);
}

fn compress(
    img: &DynamicImage,
    img_height: u32,
    img_width: u32,
    scale: f64,
    compress_format: CompressFormat,
    quality: u8,
    sampling_filter: FilterType,
) -> anyhow::Result<Vec<u8>> {
    if scale < 1.0 {
        let output_width: u32 = (img_width as f64 * scale) as u32;
        let output_height: u32 = (img_height as f64 * scale) as u32;

        let img_buf: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = imageops::resize(
            img,
            output_width,
            output_height,
            sampling_filter.to_imageops(),
        );
        return encode_img_buffer_to_bytes(&img_buf, compress_format, quality);
    }
    return encode_dyn_img_to_bytes(img, compress_format, quality);
}

pub fn fit_width(
    path_str: String,
    compress_format: Option<CompressFormat>,
    max_width: Option<i32>,
    quality: Option<u8>,
    sampling_filter: Option<FilterType>,
) -> anyhow::Result<ZeroCopyBuffer<Vec<u8>>> {
    let orientation = check_orientation(&path_str)?;
    let mut dyn_img = image::open(path_str)?;
    dyn_img = rotate(orientation, dyn_img);

    let compress_format = compress_format.unwrap_or(CompressFormat::Jpeg);
    let quality = quality.unwrap_or(80);
    let sampling_filter = sampling_filter.unwrap_or(FilterType::Triangle);

    let (img_width, img_height) = dyn_img.dimensions();
    let max_width = max_width.unwrap_or(1024);

    let ratio_x: f64 = max_width as f64 / img_width as f64;
    let scale: f64 = ratio_x;

    let res = compress(
        &dyn_img,
        img_height,
        img_width,
        scale,
        compress_format,
        quality,
        sampling_filter,
    )?;
    return Ok(ZeroCopyBuffer(res));
}

pub fn fit_height(
    path_str: String,
    compress_format: Option<CompressFormat>,
    max_height: Option<i32>,
    quality: Option<u8>,
    sampling_filter: Option<FilterType>,
) -> anyhow::Result<ZeroCopyBuffer<Vec<u8>>> {
    let orientation = check_orientation(&path_str)?;
    let mut dyn_img = image::open(path_str)?;
    dyn_img = rotate(orientation, dyn_img);

    let compress_format = compress_format.unwrap_or(CompressFormat::Jpeg);
    let quality = quality.unwrap_or(80);
    let sampling_filter = sampling_filter.unwrap_or(FilterType::Triangle);

    let (img_width, img_height) = dyn_img.dimensions();
    let max_height = max_height.unwrap_or(1024);

    let ratio_y: f64 = max_height as f64 / img_height as f64;
    let scale: f64 = ratio_y;

    let res = compress(
        &dyn_img,
        img_height,
        img_width,
        scale,
        compress_format,
        quality,
        sampling_filter,
    )?;
    return Ok(ZeroCopyBuffer(res));
}

pub fn contain(
    path_str: String,
    compress_format: Option<CompressFormat>,
    max_width: Option<i32>,
    max_height: Option<i32>,
    quality: Option<u8>,
    sampling_filter: Option<FilterType>,
) -> anyhow::Result<ZeroCopyBuffer<Vec<u8>>> {
    let orientation = check_orientation(&path_str)?;
    let mut dyn_img = image::open(path_str)?;
    dyn_img = rotate(orientation, dyn_img);

    let compress_format = compress_format.unwrap_or(CompressFormat::Jpeg);
    let quality = quality.unwrap_or(80);
    let sampling_filter = sampling_filter.unwrap_or(FilterType::Triangle);

    let (img_width, img_height) = dyn_img.dimensions();
    let max_width = max_width.unwrap_or(1024);
    let max_height = max_height.unwrap_or(1024);

    let ratio_x: f64 = max_width as f64 / img_width as f64;
    let ratio_y: f64 = max_height as f64 / img_height as f64;
    let mut scale: f64 = ratio_y;
    if ratio_x < ratio_y {
        scale = ratio_x;
    }

    let res = compress(
        &dyn_img,
        img_height,
        img_width,
        scale,
        compress_format,
        quality,
        sampling_filter,
    )?;
    return Ok(ZeroCopyBuffer(res));
}
