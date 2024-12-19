## 2.3.1
* **FIX**: ignoring AVIF compression format for small images fixed.

## 2.3.0
* **FEAT**: added `CompressFormat.avif`
* **ANDROID JAVA UPDATE**: JAVA 8 -> JAVA 17

## 2.2.0
* **DEPENDENCY UPDATE**: `flutter_rust_bridge` 2.6.0 -> 2.7.0

## 2.1.0
* Rust 1.60 or later is now required to build
* **DEPENDENCY UPDATE**: `flutter_rust_bridge` 2.5.0 -> 2.6.0 and 
* **DEPENDENCY UPDATE**: `plugin_platform_interface` 2.0.2 -> 2.1.8 

## 2.0.4
* **DEPENDENCY UPDATE**: `flutter_rust_bridge` 2.4.0 -> 2.5.0

## 2.0.3
* **FEAT**: now guesses image file format instead of using file extensions to figure out

## 2.0.2 - RETRACTED DUE TO CRLF LINE ENDING
<strike>
* **FEAT**: now guesses image file format instead of using file extensions to figure out
</strike>

## 2.0.1
> Note: This release has breaking changes.
* **DEPENDENCY UPDATE**: `flutter_rust_bridge` 2.3.0 -> 2.4.0

## 2.0.0
> Note: This release has breaking changes.
* **DOCS**: README.md update
* **FEAT**: bump flutter_rust_bridge to version 2.3.0
* **BREAKING** **REFACTOR**:
    - 1. The library should not be called as `singleton`. Just add `await NativeImageCompress.init();` for initialization.
    - 2. Methods are now static with different class name. Therefore, the methods now should be called with `ImageCompress` class. i.e) `ImageCompress.contain`

## 1.1.1+2

* README.md link fix

## 1.1.1+1

* README.md update

## 1.1.1

* README.md update
* cargokit update

## 1.1.0

* Now keeps the image orientation when converted
* Sampling Filter can be set

## 1.0.2

* FIX: fixed CR LF problem

## 1.0.1+6

* podspec description update

## 1.0.1+5

* Example and README.md updated to show a way to call the library as a singleton.

## 1.0.1+4

* homepage update just to get pub.dev score
* CHANGELOG.md update

## 1.0.1+2

* Now panic when wrong image file path is given
* ios info.plist for example updated with permission

## 1.0.1+1

* Added missing Android Plugin
* flutter debug mode now compiles optimized rust code

## 1.0.1

* Lowered minimum Dart SDK version

## 1.0.0

* Initial release.
