# simple_native_image_compress

A simple native image compression library for Flutter written in rust using [flutter_rust_bridge][1], [image][2], [cargokit][3] and [kamadak-exif][4]

## Why?

1. For some reason, image compression in Dart is slow. Even with isolate.
2. There is no native libraries that supports WINDOWS & LINUX when it comes to image compression.

## What does it do?

- If path for an image file is given, it will resize and return Jpeg/WebP image as Uint8List.

## Prerequisite

1. Rust
2. Android NDK for Android

## Supported Output Formats

- Jpeg
- WebP

## Sampling Filter Types

- 5 types of sampling filters are available and can be selected
  1. Nearest, // Nearest Neighbor
  2. Triangle, // Linear Filter (DEFAULT)
  3. CatmullRom, // Cubic Filter
  4. Gaussian, // Gaussian Filter
  5. Lanczos3, // Lanczos with window 3

- when samplingFilter is null, FilterType.Triangle is set to be default.
- you can read more about sampling filters here [image crate doc][5]

## Example

### Call Library as a Singleton
```shell
final compress = SimpleNativeImageCompress();
```

### "contain" will make the image fit into the given max width/height.
```shell
try{
  final bytes = await compress.contain(
    filePath: yourFilePath,
    compressFormat: CompressFormat.Jpeg,
    quality: 90,
    maxWidth: 512,
    maxHeight: 512,
    samplingFilter: FilterType.Lanczos3
  );
} catch (e) {
  print(e);
}
```

### "fitWidth" will make the image fit into the given max width.
```shell
try{
  final bytes = await compress.fitWidth(
    filePath: yourFilePath,
    compressFormat: CompressFormat.WebP,
    maxWidth: 512,
    samplingFilter: FilterType.Lanczos3
  );
} catch (e) {
  print(e);
}
```

### "fitHeight" will make the image fit into the given max height.
```shell
try{
  final bytes = await compress.fitHeight(
    filePath: yourFilePath,
    compressFormat: CompressFormat.WebP,
    maxHeight: 512,
    samplingFilter: FilterType.Lanczos3
  );
} catch (e) {
  print(e);
}
```

## Default values

- Default value for width and/or height is 1024 px
- Default value for Jpeg quality is 80 (For webP Quality does nothing)

[1]: <https://github.com/fzyzcjy/flutter_rust_bridge> "flutter rust bridge github"
[2]: <https://github.com/image-rs/image> "rust image crate github"
[3]: <https://github.com/irondash/cargokit> "cargokit github"
[4]: <https://github.com/kamadak/exif-rs> "exif-rs github"
[5]: <https://docs.rs/image/latest/image/imageops/enum.FilterType.html> "sampling filters page"