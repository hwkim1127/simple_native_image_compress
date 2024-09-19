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
