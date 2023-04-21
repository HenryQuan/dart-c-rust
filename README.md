# dart-c-rust
This is an experiment on controlling a Rust raw pointer (e.g Hashmap) from C or Dart. The goal is to ask Rust to do some operations on the given pointer and return some data to another language using simple nested struct.

## ffigen

## cbindgen
This library generates C header from the Rust code. 
- Install with `cargo install --force cbindgen`
- Run with `cbindgen --config cbindgen.toml --crate pointer --output hashmap.h`
