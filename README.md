# simple_native_image_compress

A simple native image compression library for Flutter written in rust using [flutter_rust_bridge][1], [image][2], [cargokit][3] and [kamadak-exif][4]

## Why?

1. For some reason, image compression in Dart is slow. Even with isolate.
2. There is no native libraries that supports WINDOWS & LINUX when it comes to image compression.

## What does it do?

- If path for an image file is given, it will resize and return Jpeg/WebP image as Uint8List.

## What it does not do

- Web Support with WASM. Since I use `Angular` for Web not `Flutter`

## Prerequisite

1. Rust
2. Android NDK for Android

## Setup

1. Follow the instructions [here][5] and install `Rust`
2. If installed already
```shell
rustup update
```

#### Android 
1. Install rust tools for Android
```shell
cargo install cargo-ndk
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
```
2. Since `simple_native_image_compress` requires Android NDK, install `Android NDK` via `Android Studio` and add ndkVersion in `android/app/build.gradle`.
```groovy
android {
    // by default the project uses NDK version from flutter plugin.
    ndkVersion flutter.ndkVersion
```

#### Apple
1. Install rust tools for Apple
```shell
cargo install cargo-xcode
cargo install cargo-lipo
```
2. If you are targeting iOS
```shell
rustup target add aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim
```

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
- you can read more about sampling filters here [image crate doc][6]

## Example

#### Call Library as a Singleton
```shell
final compress = SimpleNativeImageCompress();
```

#### "contain" will make the image fit into the given max width/height.
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

#### "fitWidth" will make the image fit into the given max width.
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

#### "fitHeight" will make the image fit into the given max height.
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

- Default value for width and/or height is `1024 px`
- Default value for Jpeg quality is `80` (For webP Quality does nothing)
- Default value for samplingFilter is `FilterType.Triangle`

[1]: <https://github.com/fzyzcjy/flutter_rust_bridge> "flutter rust bridge github"
[2]: <https://github.com/image-rs/image> "rust image crate github"
[3]: <https://github.com/irondash/cargokit> "cargokit github"
[4]: <https://github.com/kamadak/exif-rs> "exif-rs github"
[5]: <https://www.rust-lang.org/tools/install> "rust install page"
[6]: <https://docs.rs/image/latest/image/imageops/enum.FilterType.html> "sampling filters page"