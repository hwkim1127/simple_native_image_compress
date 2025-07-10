use rayon::prelude::*;
use std::io::Cursor;
use image::{ImageReader, GenericImageView};

use super::constants::{CompressFormat, FilterType};
use super::{compress, orientation};

pub struct BatchImageCompress {}

impl BatchImageCompress {
    pub fn file_paths(
        file_paths: Vec<String>,
        compress_format: Option<CompressFormat>,
        max_width: Option<i32>,
        max_height: Option<i32>,
        quality: Option<u8>,
        sampling_filter: Option<FilterType>,
        speed: Option<u8>,
    ) -> Result<Vec<Result<Vec<u8>, String>>, anyhow::Error> {
        let compress_format = compress_format.unwrap_or(CompressFormat::Jpeg);
        let quality = quality.unwrap_or(80);
        let sampling_filter = sampling_filter.unwrap_or(FilterType::Triangle);
        let max_width = max_width.unwrap_or(1024);
        let max_height = max_height.unwrap_or(1024);

        let results: Vec<Result<Vec<u8>, String>> = file_paths
            .into_par_iter()
            .map(|file_path| {
                match process_single_file(
                    file_path,
                    compress_format,
                    max_width,
                    max_height,
                    quality,
                    sampling_filter,
                    speed,
                ) {
                    Ok(bytes) => Ok(bytes),
                    Err(e) => Err(e.to_string()),
                }
            })
            .collect();

        Ok(results)
    }

    pub fn bytes(
        bytes_list: Vec<Vec<u8>>,
        compress_format: Option<CompressFormat>,
        max_width: Option<i32>,
        max_height: Option<i32>,
        quality: Option<u8>,
        sampling_filter: Option<FilterType>,
        speed: Option<u8>,
    ) -> Result<Vec<Result<Vec<u8>, String>>, anyhow::Error> {
        let compress_format = compress_format.unwrap_or(CompressFormat::Jpeg);
        let quality = quality.unwrap_or(80);
        let sampling_filter = sampling_filter.unwrap_or(FilterType::Triangle);
        let max_width = max_width.unwrap_or(1024);
        let max_height = max_height.unwrap_or(1024);

        let results: Vec<Result<Vec<u8>, String>> = bytes_list
            .into_par_iter()
            .map(|bytes| {
                match process_single_bytes(
                    bytes,
                    compress_format,
                    max_width,
                    max_height,
                    quality,
                    sampling_filter,
                    speed,
                ) {
                    Ok(bytes) => Ok(bytes),
                    Err(e) => Err(e.to_string()),
                }
            })
            .collect();

        Ok(results)
    }
}

#[inline]
fn process_single_file(
    file_path: String,
    compress_format: CompressFormat,
    max_width: i32,
    max_height: i32,
    quality: u8,
    sampling_filter: FilterType,
    speed: Option<u8>,
) -> anyhow::Result<Vec<u8>> {
    let orientation = orientation::check_from_filepath(&file_path)?;
    let mut dyn_img = ImageReader::open(file_path)?.with_guessed_format()?.decode()?;
    dyn_img = orientation::rotate(orientation, dyn_img);

    let (img_width, img_height) = dyn_img.dimensions();
    let ratio_x = (max_width as f64) / (img_width as f64);
    let ratio_y = (max_height as f64) / (img_height as f64);
    let scale = ratio_x.min(ratio_y);

    compress::compress(
        &dyn_img,
        img_height,
        img_width,
        scale,
        compress_format,
        quality,
        sampling_filter,
        speed,
    )
}

#[inline]
fn process_single_bytes(
    bytes: Vec<u8>,
    compress_format: CompressFormat,
    max_width: i32,
    max_height: i32,
    quality: u8,
    sampling_filter: FilterType,
    speed: Option<u8>,
) -> anyhow::Result<Vec<u8>> {
    let dyn_img = ImageReader::new(Cursor::new(bytes)).with_guessed_format()?.decode()?;

    let (img_width, img_height) = dyn_img.dimensions();
    let ratio_x = (max_width as f64) / (img_width as f64);
    let ratio_y = (max_height as f64) / (img_height as f64);
    let scale = ratio_x.min(ratio_y);

    compress::compress(
        &dyn_img,
        img_height,
        img_width,
        scale,
        compress_format,
        quality,
        sampling_filter,
        speed,
    )
}