import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';

import 'package:file_picker/file_picker.dart';
import 'package:flutter_avif/flutter_avif.dart';
import 'package:image_picker/image_picker.dart';
import 'package:simple_native_image_compress/simple_native_image_compress.dart';

Future<void> main() async {
  await NativeImageCompress.init();
  runApp(const MaterialApp(home: MyApp()));
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  Duration _duration = const Duration();
  final _picker = ImagePicker();

  Uint8List? _bytes;

  final _compressFormat = CompressFormat.avif;

  Future<void> _compressImageFromFilePath() async {
    String filePath = '';
    if (Platform.isMacOS) {
      final res = await FilePicker.platform.pickFiles(
        type: FileType.image,
        allowMultiple: false,
      );
      if (res == null) return;
      if (res.files.isEmpty) return;
      filePath = res.files[0].path!;
    } else {
      final file = await _picker.pickImage(source: ImageSource.gallery);
      if (file == null) return;
      filePath = file.path;
    }
    try {
      final startTime = DateTime.now();
      final bytes = await ImageCompress.containFromFilepath(
        filePath: filePath,
        compressFormat: _compressFormat,
        samplingFilter: FilterType.lanczos3,
        // withFileExt: true,
      );
      final endTime = DateTime.now();
      setState(() {
        _bytes = bytes;
        _duration = endTime.difference(startTime);
      });
    } catch (e) {
      if (!mounted) return;
      showDialog<void>(
        context: context,
        builder: (_) => AlertDialog(
          title: const Text(
            'Error Occured',
            style: TextStyle(color: Colors.red),
          ),
          content: Text(e.toString()),
          actions: [
            Center(
              child: TextButton(
                style: TextButton.styleFrom(
                  foregroundColor: Colors.white,
                  backgroundColor: Colors.blue, // Background Color
                ),
                onPressed: Navigator.of(context).pop,
                child: const Text('Ok'),
              ),
            ),
          ],
        ),
      );
    }
  }

  Future<void> _compressImageFromBytes() async {
    Uint8List? imgBytes;
    if (Platform.isMacOS) {
      final res = await FilePicker.platform.pickFiles(
        type: FileType.image,
        allowMultiple: false,
      );
      if (res == null) return;
      if (res.files.isEmpty) return;
      imgBytes = res.files[0].bytes;
    } else {
      final file = await _picker.pickImage(source: ImageSource.gallery);
      if (file == null) return;
      imgBytes = await file.readAsBytes();
    }

    if (imgBytes == null) return;

    try {
      final startTime = DateTime.now();
      final bytes = await ImageCompress.containFromBytes(
        bytes: imgBytes,
        compressFormat: _compressFormat,
        samplingFilter: FilterType.lanczos3,
        // withFileExt: true,
      );
      final endTime = DateTime.now();
      setState(() {
        _bytes = bytes;
        _duration = endTime.difference(startTime);
      });
    } catch (e) {
      if (!mounted) return;
      showDialog<void>(
        context: context,
        builder: (_) => AlertDialog(
          title: const Text(
            'Error Occured',
            style: TextStyle(color: Colors.red),
          ),
          content: Text(e.toString()),
          actions: [
            Center(
              child: TextButton(
                style: TextButton.styleFrom(
                  foregroundColor: Colors.white,
                  backgroundColor: Colors.blue, // Background Color
                ),
                onPressed: Navigator.of(context).pop,
                child: const Text('Ok'),
              ),
            ),
          ],
        ),
      );
    }
  }

  Future<void> _saveImage() async {
    if (Platform.isMacOS) return;
    String? outputFilePath = await FilePicker.platform.saveFile(
      dialogTitle: 'Please select an output file location',
      type: FileType.image,
    );

    if (outputFilePath == null) return;
    final tempFile = File(outputFilePath);
    await tempFile.create(recursive: true);
    RandomAccessFile raf = tempFile.openSync(mode: FileMode.write);
    try {
      raf.writeFromSync(_bytes!);
    } catch (e) {
      rethrow;
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Plugin example app')),
      body: SafeArea(
        child: Column(
          children: [
            const SizedBox(height: 20),
            Wrap(
              children: [
                ElevatedButton(
                  onPressed: _compressImageFromFilePath,
                  child: const Text('Compress from file path'),
                ),
                ElevatedButton(
                  onPressed: _compressImageFromBytes,
                  child: const Text('Compress from bytes'),
                ),
              ],
            ),
            const SizedBox(height: 20),
            Text(
              '(${kDebugMode ? 'DEBUG' : 'RELEASE'}) mode time taken: ${_duration.inMilliseconds} ms',
            ),
            const SizedBox(height: 10),
            Expanded(
              child: _bytes != null
                  ? InkWell(
                      onSecondaryTap: _saveImage,
                      child: _compressFormat == CompressFormat.avif
                          ? AvifImage.memory(_bytes!, fit: BoxFit.contain)
                          : Image.memory(_bytes!),
                    )
                  : Container(),
            ),
          ],
        ),
      ),
    );
  }
}
