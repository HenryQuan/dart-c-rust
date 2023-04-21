CC=gcc

# build rust lib
lib:
	cd pointer && cargo build --lib --release
	move /y pointer\target\release\pointer.dll .

c:
	$(CC) -o chashmap.exe hashmap.c pointer.dll

dart:
	dart compile exe hashmap.dart -o darthashmap.exe

all: lib c dart

header:
	cd pointer && cbindgen --config cbindgen.toml --crate pointer --output hashmap.h
	dart run ffigen

clean:
	cd pointer && cargo clean
	del *.dll *.exe
