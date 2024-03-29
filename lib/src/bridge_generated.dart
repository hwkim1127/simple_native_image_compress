// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.82.6.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, unnecessary_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names, invalid_use_of_internal_member, prefer_is_empty, unnecessary_const

import "bridge_definitions.dart";
import 'dart:convert';
import 'dart:async';
import 'package:meta/meta.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:uuid/uuid.dart';

import 'dart:convert';
import 'dart:async';
import 'package:meta/meta.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:uuid/uuid.dart';

import 'dart:ffi' as ffi;

class NativeImageCompressImpl implements NativeImageCompress {
  final NativeImageCompressPlatform _platform;
  factory NativeImageCompressImpl(ExternalLibrary dylib) =>
      NativeImageCompressImpl.raw(NativeImageCompressPlatform(dylib));

  /// Only valid on web/WASM platforms.
  factory NativeImageCompressImpl.wasm(FutureOr<WasmModule> module) =>
      NativeImageCompressImpl(module as ExternalLibrary);
  NativeImageCompressImpl.raw(this._platform);
  Future<Uint8List> fitWidth(
      {required String pathStr,
      CompressFormat? compressFormat,
      int? maxWidth,
      int? quality,
      FilterType? samplingFilter,
      dynamic hint}) {
    var arg0 = _platform.api2wire_String(pathStr);
    var arg1 =
        _platform.api2wire_opt_box_autoadd_compress_format(compressFormat);
    var arg2 = _platform.api2wire_opt_box_autoadd_i32(maxWidth);
    var arg3 = _platform.api2wire_opt_box_autoadd_u8(quality);
    var arg4 = _platform.api2wire_opt_box_autoadd_filter_type(samplingFilter);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) =>
          _platform.inner.wire_fit_width(port_, arg0, arg1, arg2, arg3, arg4),
      parseSuccessData: _wire2api_ZeroCopyBuffer_Uint8List,
      parseErrorData: _wire2api_FrbAnyhowException,
      constMeta: kFitWidthConstMeta,
      argValues: [pathStr, compressFormat, maxWidth, quality, samplingFilter],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kFitWidthConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "fit_width",
        argNames: [
          "pathStr",
          "compressFormat",
          "maxWidth",
          "quality",
          "samplingFilter"
        ],
      );

  Future<Uint8List> fitHeight(
      {required String pathStr,
      CompressFormat? compressFormat,
      int? maxHeight,
      int? quality,
      FilterType? samplingFilter,
      dynamic hint}) {
    var arg0 = _platform.api2wire_String(pathStr);
    var arg1 =
        _platform.api2wire_opt_box_autoadd_compress_format(compressFormat);
    var arg2 = _platform.api2wire_opt_box_autoadd_i32(maxHeight);
    var arg3 = _platform.api2wire_opt_box_autoadd_u8(quality);
    var arg4 = _platform.api2wire_opt_box_autoadd_filter_type(samplingFilter);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) =>
          _platform.inner.wire_fit_height(port_, arg0, arg1, arg2, arg3, arg4),
      parseSuccessData: _wire2api_ZeroCopyBuffer_Uint8List,
      parseErrorData: _wire2api_FrbAnyhowException,
      constMeta: kFitHeightConstMeta,
      argValues: [pathStr, compressFormat, maxHeight, quality, samplingFilter],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kFitHeightConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "fit_height",
        argNames: [
          "pathStr",
          "compressFormat",
          "maxHeight",
          "quality",
          "samplingFilter"
        ],
      );

  Future<Uint8List> contain(
      {required String pathStr,
      CompressFormat? compressFormat,
      int? maxWidth,
      int? maxHeight,
      int? quality,
      FilterType? samplingFilter,
      dynamic hint}) {
    var arg0 = _platform.api2wire_String(pathStr);
    var arg1 =
        _platform.api2wire_opt_box_autoadd_compress_format(compressFormat);
    var arg2 = _platform.api2wire_opt_box_autoadd_i32(maxWidth);
    var arg3 = _platform.api2wire_opt_box_autoadd_i32(maxHeight);
    var arg4 = _platform.api2wire_opt_box_autoadd_u8(quality);
    var arg5 = _platform.api2wire_opt_box_autoadd_filter_type(samplingFilter);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) => _platform.inner
          .wire_contain(port_, arg0, arg1, arg2, arg3, arg4, arg5),
      parseSuccessData: _wire2api_ZeroCopyBuffer_Uint8List,
      parseErrorData: _wire2api_FrbAnyhowException,
      constMeta: kContainConstMeta,
      argValues: [
        pathStr,
        compressFormat,
        maxWidth,
        maxHeight,
        quality,
        samplingFilter
      ],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kContainConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "contain",
        argNames: [
          "pathStr",
          "compressFormat",
          "maxWidth",
          "maxHeight",
          "quality",
          "samplingFilter"
        ],
      );

  void dispose() {
    _platform.dispose();
  }
// Section: wire2api

  FrbAnyhowException _wire2api_FrbAnyhowException(dynamic raw) {
    return FrbAnyhowException(raw as String);
  }

  String _wire2api_String(dynamic raw) {
    return raw as String;
  }

  Uint8List _wire2api_ZeroCopyBuffer_Uint8List(dynamic raw) {
    return raw as Uint8List;
  }

  int _wire2api_u8(dynamic raw) {
    return raw as int;
  }

  Uint8List _wire2api_uint_8_list(dynamic raw) {
    return raw as Uint8List;
  }
}

// Section: api2wire

@protected
int api2wire_compress_format(CompressFormat raw) {
  return api2wire_i32(raw.index);
}

@protected
int api2wire_filter_type(FilterType raw) {
  return api2wire_i32(raw.index);
}

@protected
int api2wire_i32(int raw) {
  return raw;
}

@protected
int api2wire_u8(int raw) {
  return raw;
}

// Section: finalizer

class NativeImageCompressPlatform
    extends FlutterRustBridgeBase<NativeImageCompressWire> {
  NativeImageCompressPlatform(ffi.DynamicLibrary dylib)
      : super(NativeImageCompressWire(dylib));

// Section: api2wire

  @protected
  ffi.Pointer<wire_uint_8_list> api2wire_String(String raw) {
    return api2wire_uint_8_list(utf8.encoder.convert(raw));
  }

  @protected
  ffi.Pointer<ffi.Int32> api2wire_box_autoadd_compress_format(
      CompressFormat raw) {
    return inner
        .new_box_autoadd_compress_format_0(api2wire_compress_format(raw));
  }

  @protected
  ffi.Pointer<ffi.Int32> api2wire_box_autoadd_filter_type(FilterType raw) {
    return inner.new_box_autoadd_filter_type_0(api2wire_filter_type(raw));
  }

  @protected
  ffi.Pointer<ffi.Int32> api2wire_box_autoadd_i32(int raw) {
    return inner.new_box_autoadd_i32_0(api2wire_i32(raw));
  }

  @protected
  ffi.Pointer<ffi.Uint8> api2wire_box_autoadd_u8(int raw) {
    return inner.new_box_autoadd_u8_0(api2wire_u8(raw));
  }

  @protected
  ffi.Pointer<ffi.Int32> api2wire_opt_box_autoadd_compress_format(
      CompressFormat? raw) {
    return raw == null
        ? ffi.nullptr
        : api2wire_box_autoadd_compress_format(raw);
  }

  @protected
  ffi.Pointer<ffi.Int32> api2wire_opt_box_autoadd_filter_type(FilterType? raw) {
    return raw == null ? ffi.nullptr : api2wire_box_autoadd_filter_type(raw);
  }

  @protected
  ffi.Pointer<ffi.Int32> api2wire_opt_box_autoadd_i32(int? raw) {
    return raw == null ? ffi.nullptr : api2wire_box_autoadd_i32(raw);
  }

  @protected
  ffi.Pointer<ffi.Uint8> api2wire_opt_box_autoadd_u8(int? raw) {
    return raw == null ? ffi.nullptr : api2wire_box_autoadd_u8(raw);
  }

  @protected
  ffi.Pointer<wire_uint_8_list> api2wire_uint_8_list(Uint8List raw) {
    final ans = inner.new_uint_8_list_0(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }
// Section: finalizer

// Section: api_fill_to_wire
}

// ignore_for_file: camel_case_types, non_constant_identifier_names, avoid_positional_boolean_parameters, annotate_overrides, constant_identifier_names

// AUTO GENERATED FILE, DO NOT EDIT.
//
// Generated by `package:ffigen`.
// ignore_for_file: type=lint

/// generated by flutter_rust_bridge
class NativeImageCompressWire implements FlutterRustBridgeWireBase {
  @internal
  late final dartApi = DartApiDl(init_frb_dart_api_dl);

  /// Holds the symbol lookup function.
  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
      _lookup;

  /// The symbols are looked up in [dynamicLibrary].
  NativeImageCompressWire(ffi.DynamicLibrary dynamicLibrary)
      : _lookup = dynamicLibrary.lookup;

  /// The symbols are looked up with [lookup].
  NativeImageCompressWire.fromLookup(
      ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
          lookup)
      : _lookup = lookup;

  void store_dart_post_cobject(
    DartPostCObjectFnType ptr,
  ) {
    return _store_dart_post_cobject(
      ptr,
    );
  }

  late final _store_dart_post_cobjectPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(DartPostCObjectFnType)>>(
          'store_dart_post_cobject');
  late final _store_dart_post_cobject = _store_dart_post_cobjectPtr
      .asFunction<void Function(DartPostCObjectFnType)>();

  Object get_dart_object(
    int ptr,
  ) {
    return _get_dart_object(
      ptr,
    );
  }

  late final _get_dart_objectPtr =
      _lookup<ffi.NativeFunction<ffi.Handle Function(ffi.UintPtr)>>(
          'get_dart_object');
  late final _get_dart_object =
      _get_dart_objectPtr.asFunction<Object Function(int)>();

  void drop_dart_object(
    int ptr,
  ) {
    return _drop_dart_object(
      ptr,
    );
  }

  late final _drop_dart_objectPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.UintPtr)>>(
          'drop_dart_object');
  late final _drop_dart_object =
      _drop_dart_objectPtr.asFunction<void Function(int)>();

  int new_dart_opaque(
    Object handle,
  ) {
    return _new_dart_opaque(
      handle,
    );
  }

  late final _new_dart_opaquePtr =
      _lookup<ffi.NativeFunction<ffi.UintPtr Function(ffi.Handle)>>(
          'new_dart_opaque');
  late final _new_dart_opaque =
      _new_dart_opaquePtr.asFunction<int Function(Object)>();

  int init_frb_dart_api_dl(
    ffi.Pointer<ffi.Void> obj,
  ) {
    return _init_frb_dart_api_dl(
      obj,
    );
  }

  late final _init_frb_dart_api_dlPtr =
      _lookup<ffi.NativeFunction<ffi.IntPtr Function(ffi.Pointer<ffi.Void>)>>(
          'init_frb_dart_api_dl');
  late final _init_frb_dart_api_dl = _init_frb_dart_api_dlPtr
      .asFunction<int Function(ffi.Pointer<ffi.Void>)>();

  void wire_fit_width(
    int port_,
    ffi.Pointer<wire_uint_8_list> path_str,
    ffi.Pointer<ffi.Int32> compress_format,
    ffi.Pointer<ffi.Int32> max_width,
    ffi.Pointer<ffi.Uint8> quality,
    ffi.Pointer<ffi.Int32> sampling_filter,
  ) {
    return _wire_fit_width(
      port_,
      path_str,
      compress_format,
      max_width,
      quality,
      sampling_filter,
    );
  }

  late final _wire_fit_widthPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64,
              ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<ffi.Int32>,
              ffi.Pointer<ffi.Int32>,
              ffi.Pointer<ffi.Uint8>,
              ffi.Pointer<ffi.Int32>)>>('wire_fit_width');
  late final _wire_fit_width = _wire_fit_widthPtr.asFunction<
      void Function(
          int,
          ffi.Pointer<wire_uint_8_list>,
          ffi.Pointer<ffi.Int32>,
          ffi.Pointer<ffi.Int32>,
          ffi.Pointer<ffi.Uint8>,
          ffi.Pointer<ffi.Int32>)>();

  void wire_fit_height(
    int port_,
    ffi.Pointer<wire_uint_8_list> path_str,
    ffi.Pointer<ffi.Int32> compress_format,
    ffi.Pointer<ffi.Int32> max_height,
    ffi.Pointer<ffi.Uint8> quality,
    ffi.Pointer<ffi.Int32> sampling_filter,
  ) {
    return _wire_fit_height(
      port_,
      path_str,
      compress_format,
      max_height,
      quality,
      sampling_filter,
    );
  }

  late final _wire_fit_heightPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64,
              ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<ffi.Int32>,
              ffi.Pointer<ffi.Int32>,
              ffi.Pointer<ffi.Uint8>,
              ffi.Pointer<ffi.Int32>)>>('wire_fit_height');
  late final _wire_fit_height = _wire_fit_heightPtr.asFunction<
      void Function(
          int,
          ffi.Pointer<wire_uint_8_list>,
          ffi.Pointer<ffi.Int32>,
          ffi.Pointer<ffi.Int32>,
          ffi.Pointer<ffi.Uint8>,
          ffi.Pointer<ffi.Int32>)>();

  void wire_contain(
    int port_,
    ffi.Pointer<wire_uint_8_list> path_str,
    ffi.Pointer<ffi.Int32> compress_format,
    ffi.Pointer<ffi.Int32> max_width,
    ffi.Pointer<ffi.Int32> max_height,
    ffi.Pointer<ffi.Uint8> quality,
    ffi.Pointer<ffi.Int32> sampling_filter,
  ) {
    return _wire_contain(
      port_,
      path_str,
      compress_format,
      max_width,
      max_height,
      quality,
      sampling_filter,
    );
  }

  late final _wire_containPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64,
              ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<ffi.Int32>,
              ffi.Pointer<ffi.Int32>,
              ffi.Pointer<ffi.Int32>,
              ffi.Pointer<ffi.Uint8>,
              ffi.Pointer<ffi.Int32>)>>('wire_contain');
  late final _wire_contain = _wire_containPtr.asFunction<
      void Function(
          int,
          ffi.Pointer<wire_uint_8_list>,
          ffi.Pointer<ffi.Int32>,
          ffi.Pointer<ffi.Int32>,
          ffi.Pointer<ffi.Int32>,
          ffi.Pointer<ffi.Uint8>,
          ffi.Pointer<ffi.Int32>)>();

  ffi.Pointer<ffi.Int32> new_box_autoadd_compress_format_0(
    int value,
  ) {
    return _new_box_autoadd_compress_format_0(
      value,
    );
  }

  late final _new_box_autoadd_compress_format_0Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ffi.Int32> Function(ffi.Int32)>>(
          'new_box_autoadd_compress_format_0');
  late final _new_box_autoadd_compress_format_0 =
      _new_box_autoadd_compress_format_0Ptr
          .asFunction<ffi.Pointer<ffi.Int32> Function(int)>();

  ffi.Pointer<ffi.Int32> new_box_autoadd_filter_type_0(
    int value,
  ) {
    return _new_box_autoadd_filter_type_0(
      value,
    );
  }

  late final _new_box_autoadd_filter_type_0Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ffi.Int32> Function(ffi.Int32)>>(
          'new_box_autoadd_filter_type_0');
  late final _new_box_autoadd_filter_type_0 = _new_box_autoadd_filter_type_0Ptr
      .asFunction<ffi.Pointer<ffi.Int32> Function(int)>();

  ffi.Pointer<ffi.Int32> new_box_autoadd_i32_0(
    int value,
  ) {
    return _new_box_autoadd_i32_0(
      value,
    );
  }

  late final _new_box_autoadd_i32_0Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ffi.Int32> Function(ffi.Int32)>>(
          'new_box_autoadd_i32_0');
  late final _new_box_autoadd_i32_0 = _new_box_autoadd_i32_0Ptr
      .asFunction<ffi.Pointer<ffi.Int32> Function(int)>();

  ffi.Pointer<ffi.Uint8> new_box_autoadd_u8_0(
    int value,
  ) {
    return _new_box_autoadd_u8_0(
      value,
    );
  }

  late final _new_box_autoadd_u8_0Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ffi.Uint8> Function(ffi.Uint8)>>(
          'new_box_autoadd_u8_0');
  late final _new_box_autoadd_u8_0 = _new_box_autoadd_u8_0Ptr
      .asFunction<ffi.Pointer<ffi.Uint8> Function(int)>();

  ffi.Pointer<wire_uint_8_list> new_uint_8_list_0(
    int len,
  ) {
    return _new_uint_8_list_0(
      len,
    );
  }

  late final _new_uint_8_list_0Ptr = _lookup<
          ffi
          .NativeFunction<ffi.Pointer<wire_uint_8_list> Function(ffi.Int32)>>(
      'new_uint_8_list_0');
  late final _new_uint_8_list_0 = _new_uint_8_list_0Ptr
      .asFunction<ffi.Pointer<wire_uint_8_list> Function(int)>();

  void free_WireSyncReturn(
    WireSyncReturn ptr,
  ) {
    return _free_WireSyncReturn(
      ptr,
    );
  }

  late final _free_WireSyncReturnPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(WireSyncReturn)>>(
          'free_WireSyncReturn');
  late final _free_WireSyncReturn =
      _free_WireSyncReturnPtr.asFunction<void Function(WireSyncReturn)>();
}

final class _Dart_Handle extends ffi.Opaque {}

final class wire_uint_8_list extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> ptr;

  @ffi.Int32()
  external int len;
}

typedef DartPostCObjectFnType = ffi.Pointer<
    ffi.NativeFunction<
        ffi.Bool Function(DartPort port_id, ffi.Pointer<ffi.Void> message)>>;
typedef DartPort = ffi.Int64;
