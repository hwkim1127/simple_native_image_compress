import 'package:integration_test/integration_test.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:simple_native_image_compress/simple_native_image_compress.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  setUpAll(() async => await NativeImageCompress.init());
  // test('Can call rust function', () async {
  //   // expect(greet(name: "Tom"), "Hello, Tom!");
  // });
}
