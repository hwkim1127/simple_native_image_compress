use image::imageops;

#[derive(PartialEq, Eq, Clone, Copy)]
// #[cfg_attr(debug_assertions, derive(Debug))]
pub enum CompressFormat {
    Jpeg,
    WebP,
    AVIF,
}

/*
    flutter rust bridge cannot import imageops::FilterType... so here we go...
    https://docs.rs/image/latest/image/imageops/enum.FilterType.html
*/
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum FilterType {
    Nearest,    // Nearest Neighbor
    Triangle,   // Linear Filter
    CatmullRom, // Cubic Filter
    Gaussian,   // Gaussian Filter
    Lanczos3,   // Lanczos with window 3
}

#[inline]
pub(crate) fn convert_filter_type(filter_type: FilterType) -> imageops::FilterType {
    match filter_type {
        FilterType::Nearest => imageops::FilterType::Nearest,
        // FilterType::Triangle => imageops::FilterType::Triangle,
        FilterType::CatmullRom => imageops::FilterType::CatmullRom,
        FilterType::Gaussian => imageops::FilterType::Gaussian,
        FilterType::Lanczos3 => imageops::FilterType::Lanczos3,
        _ => imageops::FilterType::Triangle,
    }
}
