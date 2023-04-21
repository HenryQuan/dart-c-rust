import 'dart:ffi';
import 'dart:io';

import 'hashmap_bindings.dart';

void main(List<String> arguments) {
  print("Dart: Hello World");
  final hashmap = RustHashmap(DynamicLibrary.open("pointer.dll"));
  final pointer = hashmap.build_hashmap();
  print("Dart: Pointer: ${pointer.address.toRadixString(16)}");
  while (true) {
    stdout.write("Dart: Enter a key: ");
    final key = stdin.readLineSync();
    if (key == null) break;
    if (key.startsWith('q')) {
      break;
    }

    final value = int.tryParse(key);
    if (value == null) {
      print("Dart: Invalid key");
      continue;
    }

    final result = hashmap.get_hashmap_value(pointer, value);
    if (result.address == 0) {
      print("Dart: Key not found");
    } else {
      stdout.write("Dart: bytes: [ ");
      final bytes = result.cast<Uint8>();
      for (var i = 0; i < 12; i++) {
        final byte = bytes.elementAt(i).value;
        stdout.write("$byte ");
      }
      print("]");
      print("Dart: value pointer: ${result.address.toRadixString(16)}");
      // cast to Foo
      final foo = result.cast<Foo>();
      print(
        "Dart: Foo: { x: ${foo.ref.x}, y: Bar: { x: ${foo.ref.y.x}, y: ${foo.ref.y.y} } }",
      );
      hashmap.free_foo(result);
      print("Dart: value freed");
    }
  }
  hashmap.free_hashmap(pointer);
  print("Dart: hashmap freed");
}
