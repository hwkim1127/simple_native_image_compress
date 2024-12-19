use image::{imageops, DynamicImage, GenericImageView, ImageReader};

use super::constants::{self, CompressFormat};
use super::{encode, orientation};


fn compress(
    img: &DynamicImage,
    img_height: u32,
    img_width: u32,
    scale: f64,
    compress_format: CompressFormat,
    quality: u8,
    sampling_filter: constants::FilterType,
    speed: Option<u8>,
) -> Result<Vec<u8>, anyhow::Error> {
    if scale < 1.0 {
        let output_width: u32 = (img_width as f64 * scale) as u32;
        let output_height: u32 = (img_height as f64 * scale) as u32;
        match compress_format {
            CompressFormat::WebP | CompressFormat::AVIF => {
                let img_buf: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = imageops::resize(
                    img,
                    output_width,
                    output_height,
                    constants::convert_filter_type(sampling_filter),
                );
                return encode::img_buffer_to_bytes_with_alpha(compress_format, &img_buf, quality, speed);
            },
            _ => {
                let img_buf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = imageops::resize(
                    &img.to_rgb8(),
                    output_width,
                    output_height,
                    constants::convert_filter_type(sampling_filter),
                );
                return encode::img_buffer_to_bytes_without_alpha(&img_buf, quality);
            },
        }

    }
    return encode::dyn_img_to_bytes(img, compress_format, quality, speed);
}

pub struct ImageCompress {}

impl ImageCompress {
    pub fn fit_width(
        file_path: String,
        compress_format: Option<CompressFormat>,
        max_width: Option<i32>,
        quality: Option<u8>,
        sampling_filter: Option<constants::FilterType>,
        speed: Option<u8>,
    ) -> Result<Vec<u8>, anyhow::Error> {
        let orientation = orientation::check(&file_path)?;
        let mut dyn_img = ImageReader::open(file_path)?.with_guessed_format()?.decode()?;
        dyn_img = orientation::rotate(orientation, dyn_img);
    
        let compress_format = compress_format.unwrap_or(CompressFormat::Jpeg);
        let quality = quality.unwrap_or(80);
        let sampling_filter = sampling_filter.unwrap_or(constants::FilterType::Triangle);
    
        let (img_width, img_height) = dyn_img.dimensions();
        let max_width = max_width.unwrap_or(1024);
    
        let ratio_x: f64 = max_width as f64 / img_width as f64;
        let scale: f64 = ratio_x;
    
        return Ok(compress(
            &dyn_img,
            img_height,
            img_width,
            scale,
            compress_format,
            quality,
            sampling_filter,
            speed,
        )?);
    }
    
    pub fn fit_height(
        file_path: String,
        compress_format: Option<CompressFormat>,
        max_height: Option<i32>,
        quality: Option<u8>,
        sampling_filter: Option<constants::FilterType>,
        speed: Option<u8>,
    ) -> Result<Vec<u8>, anyhow::Error> {
        let orientation = orientation::check(&file_path)?;
        let mut dyn_img = ImageReader::open(file_path)?.with_guessed_format()?.decode()?;
        dyn_img = orientation::rotate(orientation, dyn_img);
    
        let compress_format = compress_format.unwrap_or(CompressFormat::Jpeg);
        let quality = quality.unwrap_or(80);
        let sampling_filter = sampling_filter.unwrap_or(constants::FilterType::Triangle);
    
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
            speed,
        )?;
        return Ok(res);
    }
    
    pub fn contain(
        file_path: String,
        compress_format: Option<CompressFormat>,
        max_width: Option<i32>,
        max_height: Option<i32>,
        quality: Option<u8>,
        sampling_filter: Option<constants::FilterType>,
        speed: Option<u8>,
    ) -> Result<Vec<u8>, anyhow::Error>{
        let orientation = orientation::check(&file_path)?;
        let mut dyn_img = ImageReader::open(file_path)?.with_guessed_format()?.decode()?;
        dyn_img = orientation::rotate(orientation, dyn_img);
    
        let compress_format = compress_format.unwrap_or(CompressFormat::Jpeg);
        let quality = quality.unwrap_or(80);
        let sampling_filter = sampling_filter.unwrap_or(constants::FilterType::Triangle);
    
        let (img_width, img_height) = dyn_img.dimensions();
        let max_width = max_width.unwrap_or(1024);
        let max_height = max_height.unwrap_or(1024);
    
        let ratio_x: f64 = max_width as f64 / img_width as f64;
        let ratio_y: f64 = max_height as f64 / img_height as f64;
        let mut scale: f64 = ratio_y;
        if ratio_x < ratio_y {
            scale = ratio_x;
        }
    
        return Ok(compress(
            &dyn_img,
            img_height,
            img_width,
            scale,
            compress_format,
            quality,
            sampling_filter,
            speed,
        )?);
    }
}

