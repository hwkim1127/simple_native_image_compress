# simple_native_image_compress

A simple native image compression library for Flutter written in rust using [flutter_rust_bridge][1] and [image][2]

## Why?

1. For some reason, image compression in Dart is slow. Even with isolate.
2. There is no native libraries that supports WINDOWS & LINUX when it comes to image compression.

## What does it do?

- If path for an image file is given, it will resize and return Jpeg/WebP image as Uint8List.

## Prerequisite

1. Rust
2. Android NDK

## Example

### Call Library as a Singleton
```shell
final _compress = SimpleNativeImageCompress();
```

### "contain" will make the image fit into the given max width/height.
```shell
try{
  final bytes = await _compress.contain(
    filePath: yourFilePath,
    compressFormat: CompressFormat.Jpeg,
    quality: 90,
    maxWidth: 512,
    maxHeight: 512,
  );
} catch (e) {
  print(e);
}
```

### "fitWidth" will make the image fit into the given max width.
```shell
try{
  final bytes = await _compress.fitWidth(
    filePath: yourFilePath,
    compressFormat: CompressFormat.WebP,
    maxWidth: 512,
  );
} catch (e) {
  print(e);
}
```

### "fitHeight" will make the image fit into the given max height.
```shell
try{
  final bytes = await _compress.fitHeight(
    filePath: yourFilePath,
    compressFormat: CompressFormat.WebP,
    maxHeight: 512,
  );
} catch (e) {
  print(e);
}
```

## Default values

- Default value for width and/or height is 1024 px
- Default value for Jpeg quality is 80 (For webP Quality does nothing)

## Supported Formats

- Jpeg
- WebP

## Miscellaneous

- May run slow on debug mode, but it is fine with release mode

## TODO

Since I work on this on my spare time, I don't know when the TODO list will be completed.

[1]: <https://github.com/fzyzcjy/flutter_rust_bridge> "flutter rust bridge github"
[2]: <https://github.com/image-rs/image> "rust image crate github"
