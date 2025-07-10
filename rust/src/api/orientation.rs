use exif::{Exif, In, Tag};
use image::{imageops, DynamicImage};

#[inline]
pub(crate) fn check_from_filepath(path_str: &str) -> anyhow::Result<u32> {
    let file = std::fs::File::open(path_str)?;
    let mut buf_reader = std::io::BufReader::new(&file);
    let exif_reader = exif::Reader::new();
    let exif: Exif = match exif_reader.read_from_container(&mut buf_reader) {
        Result::Ok(exif) => exif,
        Result::Err(error) => match error {
            exif::Error::NotFound(_) | exif::Error::BlankValue(_) => return Ok(1),
            _ => return Err(anyhow::anyhow!(error.to_string())),
        },
    };

    let orientation = match exif.get_field(Tag::Orientation, In::PRIMARY) {
        Some(orientation) => match orientation.value.get_uint(0) {
            Some(v @ 1..=8) => v,
            _ => 1,
        },
        None => 1,
    };
    return Ok(orientation);
}

// pub(crate) fn check_from_bytes(buf: Vec<u8>) -> anyhow::Result<u32> {
//     let exif_reader = exif::Reader::new();
//     let exif: Exif = match exif_reader.read_raw(buf) {
//         Result::Ok(exif) => exif,
//         Result::Err(error) => match error {
//             exif::Error::NotFound(_) | exif::Error::BlankValue(_) => return Ok(1),
//             _ => return Err(anyhow::anyhow!(error.to_string())),
//         },
//     };

//     let orientation = match exif.get_field(Tag::Orientation, In::PRIMARY) {
//         Some(orientation) => match orientation.value.get_uint(0) {
//             Some(v @ 1..=8) => v,
//             _ => 1,
//         },
//         None => 1,
//     };
//     return Ok(orientation);
// }

/*
    https://magnushoff.com/articles/jpeg-orientation/
*/
#[inline]
pub(crate) fn rotate(orientation: u32, dyn_img: DynamicImage) -> DynamicImage {
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
