// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.10.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'constants.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// These functions are ignored because they are not marked as `pub`: `compress`

class ImageCompress {
  const ImageCompress();

  static Future<Uint8List> contain(
          {required String filePath,
          CompressFormat? compressFormat,
          int? maxWidth,
          int? maxHeight,
          int? quality,
          FilterType? samplingFilter,
          int? speed}) =>
      NativeImageCompress.instance.api.crateApiCompressImageCompressContain(
          filePath: filePath,
          compressFormat: compressFormat,
          maxWidth: maxWidth,
          maxHeight: maxHeight,
          quality: quality,
          samplingFilter: samplingFilter,
          speed: speed);

  static Future<Uint8List> containFromBytes(
          {required List<int> bytes,
          CompressFormat? compressFormat,
          int? maxWidth,
          int? maxHeight,
          int? quality,
          FilterType? samplingFilter,
          int? speed}) =>
      NativeImageCompress.instance.api
          .crateApiCompressImageCompressContainFromBytes(
              bytes: bytes,
              compressFormat: compressFormat,
              maxWidth: maxWidth,
              maxHeight: maxHeight,
              quality: quality,
              samplingFilter: samplingFilter,
              speed: speed);

  static Future<Uint8List> containFromFilepath(
          {required String filePath,
          CompressFormat? compressFormat,
          int? maxWidth,
          int? maxHeight,
          int? quality,
          FilterType? samplingFilter,
          int? speed}) =>
      NativeImageCompress.instance.api
          .crateApiCompressImageCompressContainFromFilepath(
              filePath: filePath,
              compressFormat: compressFormat,
              maxWidth: maxWidth,
              maxHeight: maxHeight,
              quality: quality,
              samplingFilter: samplingFilter,
              speed: speed);

  static Future<Uint8List> fitHeight(
          {required String filePath,
          CompressFormat? compressFormat,
          int? maxHeight,
          int? quality,
          FilterType? samplingFilter,
          int? speed}) =>
      NativeImageCompress.instance.api.crateApiCompressImageCompressFitHeight(
          filePath: filePath,
          compressFormat: compressFormat,
          maxHeight: maxHeight,
          quality: quality,
          samplingFilter: samplingFilter,
          speed: speed);

  static Future<Uint8List> fitHeightFromBytes(
          {required List<int> bytes,
          CompressFormat? compressFormat,
          int? maxHeight,
          int? quality,
          FilterType? samplingFilter,
          int? speed}) =>
      NativeImageCompress.instance.api
          .crateApiCompressImageCompressFitHeightFromBytes(
              bytes: bytes,
              compressFormat: compressFormat,
              maxHeight: maxHeight,
              quality: quality,
              samplingFilter: samplingFilter,
              speed: speed);

  static Future<Uint8List> fitHeightFromFilepath(
          {required String filePath,
          CompressFormat? compressFormat,
          int? maxHeight,
          int? quality,
          FilterType? samplingFilter,
          int? speed}) =>
      NativeImageCompress.instance.api
          .crateApiCompressImageCompressFitHeightFromFilepath(
              filePath: filePath,
              compressFormat: compressFormat,
              maxHeight: maxHeight,
              quality: quality,
              samplingFilter: samplingFilter,
              speed: speed);

  static Future<Uint8List> fitWidth(
          {required String filePath,
          CompressFormat? compressFormat,
          int? maxWidth,
          int? quality,
          FilterType? samplingFilter,
          int? speed}) =>
      NativeImageCompress.instance.api.crateApiCompressImageCompressFitWidth(
          filePath: filePath,
          compressFormat: compressFormat,
          maxWidth: maxWidth,
          quality: quality,
          samplingFilter: samplingFilter,
          speed: speed);

  static Future<Uint8List> fitWidthFromBytes(
          {required List<int> bytes,
          CompressFormat? compressFormat,
          int? maxWidth,
          int? quality,
          FilterType? samplingFilter,
          int? speed}) =>
      NativeImageCompress.instance.api
          .crateApiCompressImageCompressFitWidthFromBytes(
              bytes: bytes,
              compressFormat: compressFormat,
              maxWidth: maxWidth,
              quality: quality,
              samplingFilter: samplingFilter,
              speed: speed);

  static Future<Uint8List> fitWidthFromFilepath(
          {required String filePath,
          CompressFormat? compressFormat,
          int? maxWidth,
          int? quality,
          FilterType? samplingFilter,
          int? speed}) =>
      NativeImageCompress.instance.api
          .crateApiCompressImageCompressFitWidthFromFilepath(
              filePath: filePath,
              compressFormat: compressFormat,
              maxWidth: maxWidth,
              quality: quality,
              samplingFilter: samplingFilter,
              speed: speed);

  @override
  int get hashCode => 0;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ImageCompress && runtimeType == other.runtimeType;
}
