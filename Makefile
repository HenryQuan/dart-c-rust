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

clean:
	cd pointer && cargo clean
	del *.dll *.exe
